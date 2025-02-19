use crate::objects::{
    response::StockResponse,
    stock::{Stock, Ticker},
};

#[tauri::command]
pub fn create_stock(ticker: Ticker, total_shares: u64) -> Result<StockResponse, String> {
    let stock = Stock::new(ticker, total_shares);
    Ok(stock.into_response())
}
