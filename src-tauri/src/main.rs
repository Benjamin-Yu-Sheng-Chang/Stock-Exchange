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
use server::redis_order::RedisOrder;

use dotenv::dotenv;
use once_cell::sync::{Lazy, OnceCell};
use std::sync::{Arc, LazyLock};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};
use tokio::sync::Mutex;

mod db;

static EXCHANGE: Lazy<Arc<Mutex<Exchange>>> =
    Lazy::new(|| Arc::new(Mutex::new(Exchange::new("test".to_string()))));

#[tokio::main]
async fn main() {
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
