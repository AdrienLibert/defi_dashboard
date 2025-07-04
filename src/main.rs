mod api;

use anyhow::Result;
use dotenv::dotenv;
use api::client::Client;

const QUERY_ID: u32 = 0;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let api_key = std::env::var("API_KEY").expect("DUNE_API_KEY must be set in .env file");
    let dune_client = Client::new(api_key);
    
    match dune_client.get_cached_query_results(QUERY_ID, Some(100), None).await {
        Ok(records) => {
            println!("\nSuccessfully fetched and parsed {} rows!", records.len());
            if records.is_empty() {
                println!("No records found.");
            } else {
                println!("\n--- Parsed Data ---");
                println!("{: <25} | {: <42} | {: <42} | {: <20} | {: <10}",
                         "block_time", "from", "to", "formatted_value", "blockchain");
                println!("{}", "-".repeat(150));

                for record in records {
                    println!("{: <25} | {: <42} | {: <42} | {: <20.10e} | {: <10}",
                             record.block_time.format("%Y-%m-%d %H:%M:%S UTC"),
                             record.from,
                             record.to,
                             record.formatted_value,
                             record.blockchain);
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to fetch or parse query results: {:?}", e);
        }
    }

    Ok(())
}