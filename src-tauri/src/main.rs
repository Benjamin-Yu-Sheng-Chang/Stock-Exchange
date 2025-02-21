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
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            create_account,
            create_account_window,
            create_stock,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    // let account = Account::new("John Doe".to_string(), 1000);
    // let exchange = Exchange::new("NASDAQ".to_string());
    // let stock = Stock::new(Ticker::new("AAPL".to_string(), "NASDAQ".to_string()), 100);
    // let order = Order::new(
    //     stock,
    //     account.get_id(),
    //     OrderSide::Buy,
    //     &exchange,
    //     100.0,
    //     100,
    //     OrderType::Market,
    // );
    // let status = exchange.receive(&order);
    // dbg!(status);
}
