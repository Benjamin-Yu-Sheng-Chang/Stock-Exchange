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
    pub async fn new(name: String, redis_url: String) -> Result<Self, RedisError> {
        let client = redis::Client::open(redis_url)?;
        Ok(Self {
            name,
            stocks: Vec::new(),
            redis_client: Arc::new(Mutex::new(client)),
        })
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
            .map_err(|e| OrderError::RedisError(e.to_string()))?; // Explicit error type

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

        conn.zadd(&order_key.as_str(), &order.id().to_string().as_str(), score)
            .await
            .map_err(|e| OrderError::RedisError(e.to_string()))?;

        let order_key = format!("order:{}", order.id());
        let order_json = serde_json::to_string(&redis_order)
            .map_err(|e| OrderError::RedisError(e.to_string()))?;

        conn.hset(order_key, "data", order_json)
            .await
            .map_err(|e| OrderError::RedisError(e.to_string()))?;

        Ok(OrderStatus::Received)
    }
}
