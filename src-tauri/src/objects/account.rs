use std::default;

use super::{
    order::*,
    stock::{OccupiedStock, Ticker},
};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Default)]
pub struct Account<'a> {
    pub id: Uuid, // uuid v4
    pub name: String,
    pub balance: u64,
    pub occupied_balance: u64,
    pub created_at: DateTime<Utc>,
    pub owned_stocks: Vec<OccupiedStock>,
    current_orders: Vec<Order<'a>>,
}

impl<'a> Account<'a> {
    pub fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::from("Untitled"),
            balance: 0,
            occupied_balance: 0,
            current_orders: Vec::new(),
            created_at: Utc::now(),
            owned_stocks: Vec::new(),
        }
    }
    pub fn new(name: String, balance: u64) -> Self {
        Self {
            name,
            balance,
            ..Self::default()
        }
    }

    fn display(&self) -> String {
        format!("account: {}\n balance: {}", self.name, self.balance)
    }

    async fn ask(&mut self, mut order: Order<'a>) -> Result<OrderStatus, OrderError> {
        if order.price() * (order.volume() as f64) > (self.balance - self.occupied_balance) as f64 {
            return Err(OrderError::InsufficientBalance);
        }
        if order.expiration() < Utc::now() {
            return Err(OrderError::InvalidExpiration);
        }
        if order.volume() < 0 {
            return Err(OrderError::InvalidVolume);
        }
        let status = order.get_exchange().receive(&order).await?;
        order.update_status(status.clone());
        match status {
            OrderStatus::Received => {
                self.occupied_balance += order.volume();
                self.current_orders.push(order);
            }
            _ => {}
        }
        Ok(status)
    }

    async fn bid(&mut self, mut order: Order<'a>) -> Result<OrderStatus, OrderError> {
        let occupied_stock = self
            .owned_stocks
            .iter_mut()
            .find(|occupied_stock| occupied_stock.stock.ticker.eq(&order.get_stock().ticker))
            .ok_or(OrderError::StockNotFound)?;

        if order.volume() > occupied_stock.quantity - occupied_stock.occupied_quantity {
            return Err(OrderError::InsufficientShares);
        }

        let status = order.get_exchange().receive(&order).await?;
        order.update_status(status.clone());
        match status {
            OrderStatus::Received => {
                occupied_stock.occupied_quantity += order.volume();
                self.current_orders.push(order);
            }
            _ => {}
        }
        Ok(status)
    }
}
