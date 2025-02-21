use crate::objects::order::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct RedisOrder {
    id: Uuid, // uuid v7
    account_id: Uuid,
    side: OrderSide,
    stock_symbol: String,
    price: f64,
    volume: u64,
    order_type: OrderType,
    time: DateTime<Utc>,
    expiration: DateTime<Utc>,
    shares_filled: u64,
    status: OrderStatus,
}

impl RedisOrder {
    pub fn into(order: &Order) -> Self {
        Self {
            id: order.id(),
            account_id: order.account_id(),
            side: order.side(),
            stock_symbol: order.get_stock().ticker.symbol.clone(),
            price: order.price(),
            volume: order.volume(),
            order_type: order.order_type(),
            time: order.time(),
            expiration: order.expiration(),
            shares_filled: order.shares_filled(),
            status: order.status(),
        }
    }
}
