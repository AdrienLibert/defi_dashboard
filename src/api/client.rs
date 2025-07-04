use anyhow::{Result, anyhow};
use reqwest;
use csv::ReaderBuilder;
use std::io::Cursor;
use url::Url;

use crate::api::models::DuneQueryRow;
const API_URL: &str = "https://api.dune.com/api/v1";

pub struct Client {
    client: reqwest::Client,
    api_key: String,
}

impl Client {
    pub fn new(api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key,
        }
    }

    pub async fn get_cached_query_results(&self, query_id: u32, limit: Option<u32>, offset: Option<u32>) -> Result<Vec<DuneQueryRow>> {
        let mut url = Url::parse(&format!("{}/query/{}/results/csv", API_URL, query_id))?;

        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(l) = limit {
                query_pairs.append_pair("limit", &l.to_string());
            }
            if let Some(o) = offset {
                query_pairs.append_pair("offset", &o.to_string());
            }
        }

        println!("Requesting URL: {}", url.as_str());

        let response = self.client.get(url.as_str())
            .header("X-Dune-Api-Key", &self.api_key)
            .send()
            .await?;

        let status = response.status();
        let response_text = response.text().await?; // Read body once

        println!("\n--- Raw CACHED RESULTS Response for Query ID: {} ---\n{}", query_id, response_text);

        if !status.is_success() {
            return Err(anyhow!("API request failed: Status {}, Response: {}", status, response_text));
        }

        let mut reader = ReaderBuilder::new()
            .has_headers(true) 
            .from_reader(Cursor::new(response_text));

        let mut records: Vec<DuneQueryRow> = Vec::new();
        for result in reader.deserialize() {
            let record: DuneQueryRow = result.map_err(|e| anyhow!("CSV deserialization error: {}", e))?;
            records.push(record);
        }

        Ok(records)
    }
}