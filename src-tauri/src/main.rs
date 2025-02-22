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
use server::neondb::init_neondb;
use server::redis::init_redis;
use server::redis_order::RedisOrder;

#[tokio::main]
async fn main() {
    let neon_init_result = init_neondb().await;
    if neon_init_result.is_err() {
        println!("NeonDB initialization failed");
        return;
    }
    println!("NeonDB initialized");

    let redis_init_result = init_redis().await;
    if redis_init_result.is_err() {
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
