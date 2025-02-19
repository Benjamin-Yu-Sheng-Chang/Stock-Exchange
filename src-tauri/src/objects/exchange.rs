use crate::objects::order::*;
use crate::objects::stock::*;

pub struct Exchange {
    name: String,
    stocks: Vec<Stock>,
}

impl Exchange {
    pub fn new(name: String) -> Self {
        Self {
            name,
            stocks: Vec::new(),
        }
    }
    pub fn display(&self) -> String {
        format!("{}", self.name)
    }
    pub fn ipo(&mut self, stock: Stock) {
        self.stocks.push(stock);
    }
    pub fn receive(&self, order: &Order) -> Result<OrderStatus, OrderError> {
        // TODO: insert order to redis

        Ok(OrderStatus::Received)
    }
}
