use crate::objects::account::*;
use crate::objects::exchange::*;
use crate::objects::stock::*;
use chrono::{DateTime, Utc};
use getset::{CopyGetters, Getters, Setters};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum OrderType {
    Market,
    Limit,
    Stop,
    StopLimit,
    TrailingStop,
    // Fill or Kill
    FOK,
    // Immediate or Cancel
    IOC,
    // Good Till Canceled
    GTC,
}

impl OrderType {
    pub fn clone(&self) -> Self {
        match self {
            OrderType::Market => OrderType::Market,
            OrderType::Limit => OrderType::Limit,
            OrderType::Stop => OrderType::Stop,
            OrderType::StopLimit => OrderType::StopLimit,
            OrderType::TrailingStop => OrderType::TrailingStop,
            OrderType::FOK => OrderType::FOK,
            OrderType::IOC => OrderType::IOC,
            OrderType::GTC => OrderType::GTC,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum OrderStatus {
    Pending,
    Received,
    Filled,
    PartiallyFilled,
    Cancelled,
}

impl OrderStatus {
    pub fn clone(&self) -> Self {
        match self {
            OrderStatus::Pending => OrderStatus::Pending,
            OrderStatus::Received => OrderStatus::Received,
            OrderStatus::Filled => OrderStatus::Filled,
            OrderStatus::PartiallyFilled => OrderStatus::PartiallyFilled,
            OrderStatus::Cancelled => OrderStatus::Cancelled,
        }
    }
}

#[derive(Debug)]
pub enum OrderError {
    InsufficientBalance,
    InsufficientShares,
    InvalidExpiration,
    InvalidVolume,
    StockNotFound,
    RedisError(String),
}

pub enum DBOrderError {}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Getters, Setters, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct Order<'a> {
    id: Uuid, // uuid v7
    account_id: Uuid,
    side: OrderSide,
    #[getset(skip)]
    stock: Stock,
    #[getset(skip)]
    exchange: &'a Exchange,
    price: f64,
    volume: u64,
    order_type: OrderType,
    time: DateTime<Utc>,
    expiration: DateTime<Utc>,
    shares_filled: u64,
    status: OrderStatus,
    #[getset(skip)]
    processed_log: Vec<OrderLog>,
}

pub struct OrderLog {
    time: DateTime<Utc>,
    volume_filled: u64,
    status: OrderStatus,
    avg_price: f64,
    buyer_id: Uuid,
    seller_id: Uuid,
}

impl OrderLog {
    pub fn new(
        volume_filled: u64,
        status: OrderStatus,
        avg_price: f64,
        buyer_id: Uuid,
        seller_id: Uuid,
    ) -> Self {
        Self {
            time: Utc::now(),
            volume_filled,
            status,
            avg_price,
            buyer_id,
            seller_id,
        }
    }
}

impl<'a> Order<'a> {
    pub fn new(
        stock: Stock,
        account_id: Uuid,
        side: OrderSide,
        exchange: &'a Exchange,
        price: f64,
        volume: u64,
        order_type: OrderType,
    ) -> Self {
        Self {
            id: Uuid::now_v7(),
            account_id,
            side,
            stock,
            exchange,
            price,
            volume,
            order_type,
            time: Utc::now(),
            expiration: Utc::now(),
            shares_filled: 0,
            status: OrderStatus::Pending,
            processed_log: Vec::<OrderLog>::new(),
        }
    }
    pub fn update_status(&mut self, status: OrderStatus) {
        self.status = status;
    }
    pub fn update_processed_log(&mut self, log: OrderLog) {
        self.processed_log.push(log);
    }
    pub fn get_stock(&self) -> &Stock {
        &self.stock
    }
    pub fn get_exchange(&self) -> &Exchange {
        &self.exchange
    }
    pub fn get_processed_log(&self) -> &Vec<OrderLog> {
        &self.processed_log
    }
}
