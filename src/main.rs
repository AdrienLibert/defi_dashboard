mod api;

use anyhow::Result;
use dotenv::dotenv;
use api::client::Client;

const TEST_DUNE_QUERY_ID: u32 = 5375052;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let api_key = std::env::var("API_KEY").expect("DUNE_API_KEY must be set in .env file");
    let dune_client = Client::new(api_key);
    
    match dune_client.get_cached_query_results(TEST_DUNE_QUERY_ID, Some(10), None).await {
        Ok(json_string) => {
            println!("{}", json_string);
        },
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }

    Ok(())
}