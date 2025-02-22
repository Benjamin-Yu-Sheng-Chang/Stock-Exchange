use dotenv::dotenv;
use redis::RedisError;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn init_redis() -> Result<Arc<Mutex<redis::Client>>, RedisError> {
    dotenv().ok();
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
    let client = redis::Client::open(redis_url)?;

    // Test connection
    let _conn = client.get_multiplexed_async_connection().await?;
    println!("Redis connection successful");

    Ok(Arc::new(Mutex::new(client)))
}
