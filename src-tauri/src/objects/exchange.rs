use crate::objects::order::*;
use crate::objects::stock::*;
use crate::server::redis_order::RedisOrder;
use redis::AsyncCommands;
use redis::RedisError;
use std::sync::Arc;
use surrealdb::{engine::remote::ws::Client, Surreal};
use tokio::sync::Mutex;
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

    pub async fn ipo(&mut self, stock: Stock, conn: &Surreal<Client>) -> Result<Stock, String> {
        let record: Option<Stock> = conn
            .select(("stock", &stock.ticker.symbol)) // Check if stock exists first
            .await
            .unwrap_or(None);

        if record.is_some() {
            Err("Stock already exists".to_string())
        } else {
            let created: Option<Stock> = conn
                .create(("stock", &stock.ticker.symbol))
                .content(stock.clone())
                .await
                .unwrap_or_else(|e| {
                    println!("SurrealDB Error: {}", e);
                    None
                });

            if created.is_none() {
                Err("Failed to create stock".to_string())
            } else {
                self.stocks.push(stock);
                Ok(created.unwrap())
            }
        }
    }

    // pub async fn delisting(
    //     &mut self,
    //     ticker: Ticker,
    //     conn: &Surreal<Client>,
    // ) -> Result<(), String> {
    //     self.stocks
    //         .remove(self.stocks.iter().position(|s| s.ticker == ticker).unwrap());
    //     let record: Option<Stock> = conn.delete(("stock", &ticker)).await.unwrap_or(None);
    //     if record.is_none() {
    //         Err("Failed to delete stock".to_string())
    //     } else {
    //         Ok(())
    //     }
    // }

    pub async fn receive<'a>(
        &'a self,
        order: &'a Order<'a>,
        redis_client: &redis::Client,
    ) -> Result<OrderStatus, OrderError> {
        let mut conn = redis_client
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

        conn.zadd(&order_key, order.id().to_string(), score)
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
