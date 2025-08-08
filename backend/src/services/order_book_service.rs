use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use rust_decimal::Decimal;
use uuid::Uuid;
use crate::models::{Order, Trade, OrderSide, OrderStatus};
use crate::errors::AppError;

#[derive(Debug, Clone)]
struct OrderQueue {
    orders: Vec<Order>,
}

impl OrderQueue {
    fn new() -> Self {
        Self { orders: Vec::new() }
    }

    fn add_order(&mut self, order: Order) {
        self.orders.push(order);
        // Sort by creation time (FIFO)
        self.orders.sort_by_key(|o| o.created_at);
    }

    fn remove_order(&mut self, order_id: Uuid) -> Option<Order> {
        if let Some(index) = self.orders.iter().position(|o| o.id == order_id) {
            Some(self.orders.remove(index))
        } else {
            None
        }
    }

    fn get_next_order(&mut self) -> Option<Order> {
        self.orders.pop()
    }

    fn is_empty(&self) -> bool {
        self.orders.is_empty()
    }

    fn total_quantity(&self) -> Decimal {
        self.orders.iter().map(|o| o.quantity - o.filled_quantity).sum()
    }
}

#[derive(Clone)]
pub struct OrderBookService {
    bids: Arc<RwLock<BTreeMap<Decimal, OrderQueue>>>, // Price -> Orders (descending)
    asks: Arc<RwLock<BTreeMap<Decimal, OrderQueue>>>, // Price -> Orders (ascending)
}

impl OrderBookService {
    pub fn new() -> Self {
        Self {
            bids: Arc::new(RwLock::new(BTreeMap::new())),
            asks: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }

    pub async fn add_order(&mut self, order: &Order) -> Result<Vec<Trade>, AppError> {
        let mut trades = Vec::new();

        match order.side {
            OrderSide::Buy => {
                // Try to match with existing asks
                trades.extend(self.match_buy_order(order).await?);
                
                // If order still has remaining quantity, add to bids
                if order.quantity > order.filled_quantity {
                    let remaining_quantity = order.quantity - order.filled_quantity;
                    let mut remaining_order = order.clone();
                    remaining_order.quantity = remaining_quantity;
                    remaining_order.filled_quantity = Decimal::ZERO;
                    
                    let mut bids = self.bids.write().await;
                    bids.entry(order.price)
                        .or_insert_with(OrderQueue::new)
                        .add_order(remaining_order);
                }
            }
            OrderSide::Sell => {
                // Try to match with existing bids
                trades.extend(self.match_sell_order(order).await?);
                
                // If order still has remaining quantity, add to asks
                if order.quantity > order.filled_quantity {
                    let remaining_quantity = order.quantity - order.filled_quantity;
                    let mut remaining_order = order.clone();
                    remaining_order.quantity = remaining_quantity;
                    remaining_order.filled_quantity = Decimal::ZERO;
                    
                    let mut asks = self.asks.write().await;
                    asks.entry(order.price)
                        .or_insert_with(OrderQueue::new)
                        .add_order(remaining_order);
                }
            }
        }

        Ok(trades)
    }

    async fn match_buy_order(&mut self, buy_order: &Order) -> Result<Vec<Trade>, AppError> {
        let mut trades = Vec::new();
        let mut remaining_quantity = buy_order.quantity;

        // Iterate through asks in ascending order (lowest price first)
        while remaining_quantity > Decimal::ZERO {
            let ask_price = {
                let asks = self.asks.read().await;
                if let Some((&price, _)) = asks.first_key_value() {
                    price
                } else {
                    break;
                }
            };

            // Check if buy price is >= ask price
            if buy_order.price >= ask_price {
                let mut asks = self.asks.write().await;
                if let Some(ask_queue) = asks.get_mut(&ask_price) {
                    if let Some(mut ask_order) = ask_queue.get_next_order() {
                        let trade_quantity = std::cmp::min(remaining_quantity, ask_order.quantity - ask_order.filled_quantity);
                        
                        if trade_quantity > Decimal::ZERO {
                            // Create trade
                            let trade = Trade {
                                id: Uuid::new_v4(),
                                order_id: ask_order.id,
                                symbol: buy_order.symbol.clone(),
                                quantity: trade_quantity,
                                price: ask_price,
                                executed_at: chrono::Utc::now(),
                            };
                            trades.push(trade);

                            // Update quantities
                            remaining_quantity -= trade_quantity;
                            ask_order.filled_quantity += trade_quantity;

                            // If ask order is not fully filled, put it back
                            if ask_order.filled_quantity < ask_order.quantity {
                                ask_queue.add_order(ask_order);
                            }
                        }
                    } else {
                        // No more orders at this price level
                        asks.remove(&ask_price);
                    }
                }
            } else {
                // Buy price is too low, stop matching
                break;
            }
        }

        Ok(trades)
    }

    async fn match_sell_order(&mut self, sell_order: &Order) -> Result<Vec<Trade>, AppError> {
        let mut trades = Vec::new();
        let mut remaining_quantity = sell_order.quantity;

        // Iterate through bids in descending order (highest price first)
        while remaining_quantity > Decimal::ZERO {
            let bid_price = {
                let bids = self.bids.read().await;
                if let Some((&price, _)) = bids.last_key_value() {
                    price
                } else {
                    break;
                }
            };

            // Check if sell price is <= bid price
            if sell_order.price <= bid_price {
                let mut bids = self.bids.write().await;
                if let Some(bid_queue) = bids.get_mut(&bid_price) {
                    if let Some(mut bid_order) = bid_queue.get_next_order() {
                        let trade_quantity = std::cmp::min(remaining_quantity, bid_order.quantity - bid_order.filled_quantity);
                        
                        if trade_quantity > Decimal::ZERO {
                            // Create trade
                            let trade = Trade {
                                id: Uuid::new_v4(),
                                order_id: bid_order.id,
                                symbol: sell_order.symbol.clone(),
                                quantity: trade_quantity,
                                price: bid_price,
                                executed_at: chrono::Utc::now(),
                            };
                            trades.push(trade);

                            // Update quantities
                            remaining_quantity -= trade_quantity;
                            bid_order.filled_quantity += trade_quantity;

                            // If bid order is not fully filled, put it back
                            if bid_order.filled_quantity < bid_order.quantity {
                                bid_queue.add_order(bid_order);
                            }
                        }
                    } else {
                        // No more orders at this price level
                        bids.remove(&bid_price);
                    }
                }
            } else {
                // Sell price is too high, stop matching
                break;
            }
        }

        Ok(trades)
    }

    pub async fn remove_order(&mut self, order: &Order) -> Result<(), AppError> {
        match order.side {
            OrderSide::Buy => {
                let mut bids = self.bids.write().await;
                if let Some(queue) = bids.get_mut(&order.price) {
                    queue.remove_order(order.id);
                    if queue.is_empty() {
                        bids.remove(&order.price);
                    }
                }
            }
            OrderSide::Sell => {
                let mut asks = self.asks.write().await;
                if let Some(queue) = asks.get_mut(&order.price) {
                    queue.remove_order(order.id);
                    if queue.is_empty() {
                        asks.remove(&order.price);
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn get_order_book(&self, symbol: &str) -> crate::models::OrderBook {
        let bids: Vec<crate::models::OrderBookEntry> = {
            let bids = self.bids.read().await;
            bids.iter()
                .rev() // Reverse to get highest price first
                .take(10) // Limit to top 10 levels
                .map(|(price, queue)| crate::models::OrderBookEntry {
                    price: *price,
                    quantity: queue.total_quantity(),
                    order_count: queue.orders.len() as i32,
                })
                .collect()
        };

        let asks: Vec<crate::models::OrderBookEntry> = {
            let asks = self.asks.read().await;
            asks.iter()
                .take(10) // Limit to top 10 levels
                .map(|(price, queue)| crate::models::OrderBookEntry {
                    price: *price,
                    quantity: queue.total_quantity(),
                    order_count: queue.orders.len() as i32,
                })
                .collect()
        };

        crate::models::OrderBook {
            symbol: symbol.to_string(),
            bids,
            asks,
            last_updated: chrono::Utc::now(),
        }
    }
} 