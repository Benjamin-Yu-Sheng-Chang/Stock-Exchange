use crate::objects::stock::Ticker;
use serde::Serialize;

#[derive(Serialize)]
pub struct AccountResponse {
    pub id: String,
    pub name: String,
    pub balance: u64,
    pub occupied_balance: u64,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct StockResponse {
    pub ticker: Ticker,
    pub ipo_date_time: String,
    pub total_shares: u64,
    pub last_match_price: f64,
}
