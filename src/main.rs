mod api;

use anyhow::Result;
use dotenv::dotenv;
use api::client::Client;
use api::helper::save_to_avro;

const QUERY_ID: u32 = 5375052;

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
                let avro_file_path = "./avro_data/query_results.avro";
                save_to_avro(&records, avro_file_path).await?;
            }
        },
        Err(e) => {
            eprintln!("Failed to fetch or parse query results: {:?}", e);
        }
    }

    Ok(())
}