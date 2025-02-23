// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[allow(unused_imports)]
mod objects;
use objects::account::Account;
use objects::exchange::Exchange;
use objects::order::*;
use objects::stock::{Stock, Ticker};

mod command;
use command::account::{create_account, frontend::create_account_window};
use command::stock::create_stock;

mod server;
use dotenv::dotenv;
use redis::RedisError;
use server::redis_order::RedisOrder;
use std::env;
use std::sync::Arc;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // Initialize SurrealDB
    let db = Surreal::new::<Ws>(env::var("SURREALDB_URL").expect("SURREALDB_URL must be set"))
        .await
        .expect("Failed to connect to SurrealDB");

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: env::var("SURREALDB_USER")
            .expect("SURREALDB_USER must be set")
            .as_str(),
        password: env::var("SURREALDB_PASSWORD")
            .expect("SURREALDB_PASSWORD must be set")
            .as_str(),
    })
    .await
    .expect("Failed to signin to SurrealDB");

    // Select a specific namespace / database
    db.use_ns(
        env::var("SURREALDB_NS")
            .expect("SURREALDB_NS must be set")
            .as_str(),
    )
    .use_db(
        env::var("SURREALDB_DB")
            .expect("SURREALDB_DB must be set")
            .as_str(),
    )
    .await
    .expect("Failed to select namespace / database");
    println!("SurrealDB initialized");

    // Initialize Redis
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let redis_client = redis::Client::open(redis_url).expect("Failed to create Redis client");

    if let Err(_) = redis_client.get_multiplexed_async_connection().await {
        println!("Redis initialization failed");
        return;
    }
    println!("Redis initialized");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            create_account,
            create_account_window,
            create_stock,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
