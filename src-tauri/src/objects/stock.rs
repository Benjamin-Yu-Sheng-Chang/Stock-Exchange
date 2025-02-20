use crate::objects::response::StockResponse;
use chrono::{DateTime, Utc};

pub struct Stock {
    pub ticker: Ticker,
    pub ipo_date_time: DateTime<Utc>,
    pub total_shares: u64,
    pub last_match_price: f64,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Ticker {
    pub symbol: String,
    pub exchange: String,
}

impl Ticker {
    pub fn new(symbol: String, exchange: String) -> Self {
        Self { symbol, exchange }
    }
}

impl Stock {
    pub fn new(ticker: Ticker, total_shares: u64) -> Self {
        Self {
            ticker,
            ipo_date_time: Utc::now(),
            total_shares,
            last_match_price: 10.0,
        }
    }
    pub fn display(&self) -> String {
        format!("{}:{}", self.ticker.symbol, self.ticker.exchange)
    }
    pub fn offering(&self, shares: u64) -> u64 {
        self.total_shares + shares
    }
    pub fn buyback(&self, shares: u64) -> u64 {
        self.total_shares - shares
    }
    pub fn clone(&self) -> Self {
        Self {
            ticker: self.ticker.clone(),
            ipo_date_time: self.ipo_date_time,
            total_shares: self.total_shares,
            last_match_price: self.last_match_price,
        }
    }
    pub fn into_response(&self) -> StockResponse {
        StockResponse {
            ticker: self.ticker.clone(),
            ipo_date_time: self.ipo_date_time.to_rfc3339(),
            total_shares: self.total_shares,
            last_match_price: self.last_match_price,
        }
    }
}
