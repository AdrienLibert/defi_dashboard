// main.rs
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use dotenv::dotenv;
use chrono::{DateTime, Utc};
use tokio::fs::File;
use csv::Writer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    // Get the Dune API key from environment variables
    let dune_api_key = env::var("DUNE_API_KEY")
        .expect("DUNE_API_KEY must be set in .env file");
    Ok(())
}