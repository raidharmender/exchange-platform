use serde::{Deserialize, Serialize};
#[cfg(feature = "database")]
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "database", derive(FromRow))]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub status: UserStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "database", derive(sqlx::Type))]
#[cfg_attr(feature = "database", sqlx(type_name = "user_status", rename_all = "lowercase"))]
pub enum UserStatus {
    Active,
    Inactive,
    Suspended,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "database", derive(FromRow))]
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
    pub symbol: String,
    pub side: OrderSide,
    pub quantity: Decimal,
    pub price: Decimal,
    pub order_type: OrderType,
    pub status: OrderStatus,
    pub filled_quantity: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "database", derive(sqlx::Type))]
#[cfg_attr(feature = "database", sqlx(type_name = "order_side", rename_all = "lowercase"))]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "database", derive(sqlx::Type))]
#[cfg_attr(feature = "database", sqlx(type_name = "order_type", rename_all = "lowercase"))]
pub enum OrderType {
    Market,
    Limit,
    Stop,
    StopLimit,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "database", derive(sqlx::Type))]
#[cfg_attr(feature = "database", sqlx(type_name = "order_status", rename_all = "lowercase"))]
pub enum OrderStatus {
    New,
    Open,
    PartiallyFilled,
    Filled,
    Cancelled,
    Rejected,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "database", derive(FromRow))]
pub struct Trade {
    pub id: Uuid,
    pub order_id: Uuid,
    pub symbol: String,
    pub quantity: Decimal,
    pub price: Decimal,
    pub executed_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    pub symbol: String,
    pub side: OrderSide,
    pub quantity: Decimal,
    pub price: Decimal,
    pub order_type: OrderType,
}

impl CreateOrderRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.symbol.is_empty() || self.symbol.len() > 20 {
            return Err("Symbol must be between 1 and 20 characters".to_string());
        }
        
        if self.quantity <= Decimal::ZERO {
            return Err("Quantity must be greater than 0".to_string());
        }
        
        if self.price <= Decimal::ZERO {
            return Err("Price must be greater than 0".to_string());
        }
        
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderResponse {
    pub id: Uuid,
    pub symbol: String,
    pub side: OrderSide,
    pub quantity: Decimal,
    pub price: Decimal,
    pub order_type: OrderType,
    pub status: OrderStatus,
    pub filled_quantity: Decimal,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeResponse {
    pub id: Uuid,
    pub symbol: String,
    pub quantity: Decimal,
    pub price: Decimal,
    pub executed_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderBookEntry {
    pub price: Decimal,
    pub quantity: Decimal,
    pub order_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderBook {
    pub symbol: String,
    pub bids: Vec<OrderBookEntry>,
    pub asks: Vec<OrderBookEntry>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketData {
    pub symbol: String,
    pub last_price: Decimal,
    pub volume_24h: Decimal,
    pub change_24h: Decimal,
    pub high_24h: Decimal,
    pub low_24h: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebSocketMessage {
    pub message_type: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum WebSocketMessageType {
    OrderUpdate,
    TradeUpdate,
    MarketData,
    Error,
} 

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    #[test]
    fn test_create_order_request_validation() {
        // Test valid request
        let valid_request = CreateOrderRequest {
            symbol: "BTC/USD".to_string(),
            side: OrderSide::Buy,
            quantity: Decimal::new(100, 2), // 1.00
            price: Decimal::new(5000000, 2), // 50000.00
            order_type: OrderType::Limit,
        };
        assert!(valid_request.validate().is_ok());

        // Test invalid symbol (empty)
        let invalid_symbol = CreateOrderRequest {
            symbol: "".to_string(),
            side: OrderSide::Buy,
            quantity: Decimal::new(100, 2),
            price: Decimal::new(5000000, 2),
            order_type: OrderType::Limit,
        };
        assert!(invalid_symbol.validate().is_err());

        // Test invalid quantity (zero)
        let invalid_quantity = CreateOrderRequest {
            symbol: "BTC/USD".to_string(),
            side: OrderSide::Buy,
            quantity: Decimal::ZERO,
            price: Decimal::new(5000000, 2),
            order_type: OrderType::Limit,
        };
        assert!(invalid_quantity.validate().is_err());

        // Test invalid price (negative)
        let invalid_price = CreateOrderRequest {
            symbol: "BTC/USD".to_string(),
            side: OrderSide::Buy,
            quantity: Decimal::new(100, 2),
            price: Decimal::new(-10000, 2), // -100.00
            order_type: OrderType::Limit,
        };
        assert!(invalid_price.validate().is_err());
    }
} 