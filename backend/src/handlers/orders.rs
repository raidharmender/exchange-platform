use actix_web::{web, HttpResponse, get, post, put, delete};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::{CreateOrderRequest, OrderResponse, Order, OrderStatus};
use crate::errors::AppError;
use crate::services::order_service::OrderService;

#[derive(Deserialize)]
pub struct OrderQuery {
    pub symbol: Option<String>,
    pub status: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[get("/orders")]
pub async fn get_orders(
    query: web::Query<OrderQuery>,
    order_service: web::Data<OrderService>,
) -> Result<HttpResponse, AppError> {
    let orders = order_service.get_orders(&query).await?;
    Ok(HttpResponse::Ok().json(orders))
}

#[get("/orders/{id}")]
pub async fn get_order(
    path: web::Path<Uuid>,
    order_service: web::Data<OrderService>,
) -> Result<HttpResponse, AppError> {
    let order_id = path.into_inner();
    let order = order_service.get_order(order_id).await?;
    Ok(HttpResponse::Ok().json(order))
}

#[post("/orders")]
pub async fn create_order(
    order_request: web::Json<CreateOrderRequest>,
    order_service: web::Data<OrderService>,
) -> Result<HttpResponse, AppError> {
    // Validate the request
    order_request.validate().map_err(|e| AppError::Validation(e))?;
    
    let order = order_service.create_order(order_request.into_inner()).await?;
    Ok(HttpResponse::Created().json(order))
}

#[put("/orders/{id}/cancel")]
pub async fn cancel_order(
    path: web::Path<Uuid>,
    order_service: web::Data<OrderService>,
) -> Result<HttpResponse, AppError> {
    let order_id = path.into_inner();
    let order = order_service.cancel_order(order_id).await?;
    Ok(HttpResponse::Ok().json(order))
}

#[get("/orders/{id}/trades")]
pub async fn get_order_trades(
    path: web::Path<Uuid>,
    order_service: web::Data<OrderService>,
) -> Result<HttpResponse, AppError> {
    let order_id = path.into_inner();
    let trades = order_service.get_order_trades(order_id).await?;
    Ok(HttpResponse::Ok().json(trades))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/orders")
            .service(get_orders)
            .service(get_order)
            .service(create_order)
            .service(cancel_order)
            .service(get_order_trades)
    );
} 