use crate::EXCHANGE;
use crate::{
    db::establish_db_connection,
    objects::{
        response::StockResponse,
        stock::{Stock, Ticker},
    },
};

#[tauri::command]
pub async fn create_stock(ticker: Ticker, total_shares: u64) -> Result<StockResponse, String> {
    let stock = Stock::new(ticker, total_shares);
    let conn = establish_db_connection().await;
    match EXCHANGE.lock().await.ipo(stock, &conn).await {
        Ok(stock) => {
            println!("Stock created successfully");
            Ok(stock.into_response())
        }
        Err(e) => {
            eprintln!("Error during IPO: {}", e); // Log error
            Err(e) // Failure: Return the error
        }
    }
}

// #[tauri::command]
// pub async fn delete_stock(ticker: Ticker) -> Result<(), String> {
//     let conn = establish_db_connection().await;
//     match EXCHANGE.lock().await.delisting(ticker, &conn).await {
//         Ok(_) => Ok(()),
//         Err(e) => Err(e),
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::objects::stock::Ticker;

    #[tokio::test]
    async fn test_create_stock() {
        // Prepare the input data
        let ticker = Ticker {
            symbol: "AAPL".to_string(),
            exchange: "NASDAQ".to_string(),
        };
        let total_shares = 1000;

        // Call the create_stock function
        let result = create_stock(ticker.clone(), total_shares).await;

        // Match the result and check if it's successful
        match result {
            Ok(response) => {
                // Validate the response, for example, check that the ticker symbol is correct
                assert_eq!(response.ticker.symbol, "AAPL");
                assert_eq!(response.total_shares, 1000);
                println!("Stock created successfully: {}", response.ticker.symbol);
            }
            Err(e) => {
                panic!("Error: {}", e); // Panic if an error occurs
            }
        }
    }
}
