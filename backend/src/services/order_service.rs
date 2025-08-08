#[cfg(feature = "database")]
use sqlx::PgPool;
use uuid::Uuid;
use std::sync::Arc;
use crate::models::{Order, CreateOrderRequest, OrderResponse, OrderStatus, OrderSide, OrderType};
use crate::errors::AppError;
use crate::handlers::orders::OrderQuery;
use super::order_book_service::OrderBookService;

#[derive(Clone)]
pub struct OrderService {
    #[cfg(feature = "database")]
    pool: Arc<PgPool>,
    order_book: Arc<OrderBookService>,
}

impl OrderService {
    #[cfg(feature = "database")]
    pub fn new(pool: PgPool, order_book: OrderBookService) -> Self {
        Self { 
            pool: Arc::new(pool), 
            order_book: Arc::new(order_book) 
        }
    }

    #[cfg(not(feature = "database"))]
    pub fn new(order_book: OrderBookService) -> Self {
        Self { 
            order_book: Arc::new(order_book) 
        }
    }

    pub async fn create_order(&self, request: CreateOrderRequest) -> Result<OrderResponse, AppError> {
        // Validate order
        self.validate_order(&request).await?;

        #[cfg(feature = "database")]
        {
            // Create order in database
            let order = sqlx::query_as!(
                Order,
                r#"
                INSERT INTO orders (user_id, symbol, side, quantity, price, order_type, status)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING *
                "#,
                Uuid::new_v4(), // TODO: Get from auth context
                request.symbol,
                request.side as OrderSide,
                request.quantity,
                request.price,
                request.order_type as OrderType,
                OrderStatus::New as OrderStatus
            )
            .fetch_one(&self.pool)
            .await?;

            // Add to order book
            let trades = self.order_book.add_order(&order).await?;

            // Update order status if trades occurred
            if !trades.is_empty() {
                let filled_quantity: rust_decimal::Decimal = trades.iter()
                    .map(|t| t.quantity)
                    .sum();
                
                let status = if filled_quantity >= order.quantity {
                    OrderStatus::Filled
                } else {
                    OrderStatus::PartiallyFilled
                };

                sqlx::query!(
                    "UPDATE orders SET status = $1, filled_quantity = $2 WHERE id = $3",
                    status as OrderStatus,
                    filled_quantity,
                    order.id
                )
                .execute(&self.pool)
                .await?;
            }

            Ok(OrderResponse::from(order))
        }

        #[cfg(not(feature = "database"))]
        {
            // Mock implementation
            let order = Order {
                id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                symbol: request.symbol,
                side: request.side,
                quantity: request.quantity,
                price: request.price,
                order_type: request.order_type,
                status: OrderStatus::New,
                filled_quantity: rust_decimal::Decimal::ZERO,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };
            
            Ok(OrderResponse::from(order))
        }
    }

    pub async fn get_order(&self, order_id: Uuid) -> Result<OrderResponse, AppError> {
        #[cfg(feature = "database")]
        {
            let order = sqlx::query_as!(
                Order,
                "SELECT * FROM orders WHERE id = $1",
                order_id
            )
            .fetch_optional(&self.pool)
            .await?
            .ok_or_else(|| AppError::NotFound("Order not found".to_string()))?;

            Ok(OrderResponse::from(order))
        }

        #[cfg(not(feature = "database"))]
        {
            // Mock implementation
            Err(AppError::NotFound("Order not found".to_string()))
        }
    }

    pub async fn get_orders(&self, query: &OrderQuery) -> Result<Vec<OrderResponse>, AppError> {
        #[cfg(feature = "database")]
        {
            let mut sql = "SELECT * FROM orders WHERE 1=1".to_string();
            let mut params: Vec<Box<dyn sqlx::Encode<'_, sqlx::Postgres> + Send + Sync>> = vec![];
            let mut param_count = 0;

            if let Some(ref symbol) = query.symbol {
                param_count += 1;
                sql.push_str(&format!(" AND symbol = ${}", param_count));
                params.push(Box::new(symbol.clone()));
            }

            if let Some(ref status) = query.status {
                param_count += 1;
                sql.push_str(&format!(" AND status = ${}", param_count));
                params.push(Box::new(status.clone()));
            }

            sql.push_str(" ORDER BY created_at DESC");

            if let Some(limit) = query.limit {
                param_count += 1;
                sql.push_str(&format!(" LIMIT ${}", param_count));
                params.push(Box::new(limit));
            }

            if let Some(offset) = query.offset {
                param_count += 1;
                sql.push_str(&format!(" OFFSET ${}", param_count));
                params.push(Box::new(offset));
            }

            let orders = sqlx::query_as::<_, Order>(&sql)
                .fetch_all(&self.pool)
                .await?;

            Ok(orders.into_iter().map(OrderResponse::from).collect())
        }

        #[cfg(not(feature = "database"))]
        {
            // Mock implementation
            Ok(vec![])
        }
    }

    pub async fn cancel_order(&self, order_id: Uuid) -> Result<OrderResponse, AppError> {
        #[cfg(feature = "database")]
        {
            let order = sqlx::query_as!(
                Order,
                "SELECT * FROM orders WHERE id = $1 AND status IN ('new', 'open', 'partially_filled')",
                order_id
            )
            .fetch_optional(&self.pool)
            .await?
            .ok_or_else(|| AppError::NotFound("Order not found or cannot be cancelled".to_string()))?;

            // Remove from order book
            self.order_book.remove_order(&order).await?;

            // Update status
            let updated_order = sqlx::query_as!(
                Order,
                "UPDATE orders SET status = $1 WHERE id = $2 RETURNING *",
                OrderStatus::Cancelled as OrderStatus,
                order_id
            )
            .fetch_one(&self.pool)
            .await?;

            Ok(OrderResponse::from(updated_order))
        }

        #[cfg(not(feature = "database"))]
        {
            // Mock implementation
            Err(AppError::NotFound("Order not found or cannot be cancelled".to_string()))
        }
    }

    pub async fn get_order_trades(&self, order_id: Uuid) -> Result<Vec<crate::models::TradeResponse>, AppError> {
        #[cfg(feature = "database")]
        {
            let trades = sqlx::query_as!(
                crate::models::Trade,
                "SELECT * FROM trades WHERE order_id = $1 ORDER BY executed_at DESC",
                order_id
            )
            .fetch_all(&self.pool)
            .await?;

            Ok(trades.into_iter().map(crate::models::TradeResponse::from).collect())
        }

        #[cfg(not(feature = "database"))]
        {
            // Mock implementation
            Ok(vec![])
        }
    }

    async fn validate_order(&self, request: &CreateOrderRequest) -> Result<(), AppError> {
        // Check if user has sufficient balance
        // TODO: Implement balance checking logic
        
        // Check if symbol is valid
        // TODO: Implement symbol validation
        
        // Check if price is within acceptable range
        // TODO: Implement price validation
        
        Ok(())
    }
}

impl From<Order> for OrderResponse {
    fn from(order: Order) -> Self {
        Self {
            id: order.id,
            symbol: order.symbol,
            side: order.side,
            quantity: order.quantity,
            price: order.price,
            order_type: order.order_type,
            status: order.status,
            filled_quantity: order.filled_quantity,
            created_at: order.created_at,
        }
    }
} 