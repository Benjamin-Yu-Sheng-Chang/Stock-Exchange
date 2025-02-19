use crate::objects::account::*;
use crate::objects::exchange::*;
use crate::objects::stock::*;
use chrono::{DateTime, Utc};
use uuid::Uuid;

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

#[derive(Debug)]
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
    InvalidExpiration,
    InvalidVolume,
}

pub enum DBOrderError {}

pub enum OrderSide {
    Buy,
    Sell,
}

pub struct Order<'a> {
    id: Uuid, // uuid v7
    account_id: Uuid,
    side: OrderSide,
    stock: Stock,
    exchange: &'a Exchange,
    price: f64,
    volume: u64,
    order_type: OrderType,
    time: DateTime<Utc>,
    expiration: DateTime<Utc>,
    shares_filled: u64,
    status: OrderStatus,
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
    pub fn get_id(&self) -> Uuid {
        self.id
    }
    pub fn get_stock(&self) -> Stock {
        self.stock.clone()
    }
    pub fn get_exchange(&self) -> &Exchange {
        &self.exchange
    }
    pub fn get_price(&self) -> f64 {
        self.price
    }
    pub fn get_volume(&self) -> u64 {
        self.volume
    }
    pub fn get_order_type(&self) -> OrderType {
        self.order_type.clone()
    }
    pub fn get_time(&self) -> DateTime<Utc> {
        self.time
    }
    pub fn get_expiration(&self) -> DateTime<Utc> {
        self.expiration
    }
}
