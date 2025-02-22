use crate::objects::order::*;
use crate::objects::stock::*;
use crate::server::redis_order::RedisOrder;
use redis::AsyncCommands;
use redis::RedisError;
use std::sync::Arc;
use tokio::sync::Mutex;
pub struct Exchange {
    name: String,
    stocks: Vec<Stock>,
    redis_client: Arc<Mutex<redis::Client>>,
}

impl Exchange {
    pub fn new(name: String, redis_client: Arc<Mutex<redis::Client>>) -> Self {
        Self {
            name,
            stocks: Vec::new(),
            redis_client,
        }
    }
    pub fn display(&self) -> String {
        format!("{}", self.name)
    }
    pub fn ipo(&mut self, stock: Stock) {
        self.stocks.push(stock);
    }
    pub async fn receive<'a>(&'a self, order: &'a Order<'a>) -> Result<OrderStatus, OrderError> {
        let mut conn = self
            .redis_client
            .lock()
            .await
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| OrderError::RedisError(e.to_string()))?;

        let order_key = format!(
            "orderbook:{}:{}",
            order.get_stock().ticker.symbol,
            match order.side() {
                OrderSide::Buy => "buy",
                OrderSide::Sell => "sell",
            }
        );

        let redis_order = RedisOrder::into(order);
        let score = order.time().timestamp_millis();

        let _: () = conn
            .zadd(&order_key, order.id().to_string(), score)
            .await
            .map_err(|e| OrderError::RedisError(e.to_string()))?;

        let order_key = format!("order:{}", order.id());
        let order_json = serde_json::to_string(&redis_order)
            .map_err(|e| OrderError::RedisError(e.to_string()))?;
        let _: () = conn
            .hset(order_key, "data", order_json)
            .await
            .map_err(|e| OrderError::RedisError(e.to_string()))?;

        Ok(OrderStatus::Received)
    }
}
