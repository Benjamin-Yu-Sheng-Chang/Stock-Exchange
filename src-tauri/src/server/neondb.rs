use dotenv::dotenv;
use openssl::ssl::{SslConnector, SslMethod};
use postgres_openssl::MakeTlsConnector;
use std::env;
use std::error::Error;
use tokio_postgres::{Client, NoTls};

pub async fn init_neondb() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let neon_connection_string = env::var("NEON_CONNECTION_STRING").unwrap();

    let builder = SslConnector::builder(SslMethod::tls())?;
    let connector = MakeTlsConnector::new(builder.build());

    let (client, connection) = tokio_postgres::connect(&neon_connection_string, connector).await?;

    // Spawn the connection task
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    // Query execution
    let rows = client.query("SELECT 42", &[]).await?;
    for row in rows {
        let ret: i32 = row.get(0);
        println!("Result = {}", ret);
    }
    Ok(())
}
