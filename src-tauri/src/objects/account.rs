use std::default;

use super::order::*;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Default)]
pub struct Account<'a> {
    pub id: Uuid, // uuid v4
    pub name: String,
    pub balance: u64,
    pub occupied_balance: u64,
    pub created_at: DateTime<Utc>,
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

    fn ask(&mut self, order: &mut Order) -> Result<OrderStatus, OrderError> {
        if order.get_price() * (order.get_volume() as f64)
            > (self.balance - self.occupied_balance) as f64
        {
            return Err(OrderError::InsufficientBalance);
        }
        if order.get_expiration() < Utc::now() {
            return Err(OrderError::InvalidExpiration);
        }
        if order.get_volume() < 0 {
            return Err(OrderError::InvalidVolume);
        }
        let status = order.get_exchange().receive(&*order);
        match status {
            Ok(status) => {
                order.update_status(status.clone());
                Ok(status)
            }
            Err(e) => Err(e),
        }
    }

    fn bid(&mut self, mut order: Order) -> Result<OrderStatus, OrderError> {
        let status = order.get_exchange().receive(&order);
        match status {
            Ok(status) => {
                order.update_status(status.clone());
                Ok(status)
            }
            Err(e) => Err(e),
        }
    }
}
