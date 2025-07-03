use reqwest;
use anyhow::{Result};

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

    pub async fn get_cached_query_results(&self, query_id: u32, limit: Option<u32>, offset: Option<u32>) -> Result<String> {
        let mut url = format!("{}/query/{}/results", API_URL, query_id);

        let mut query_params = Vec::new();
        if let Some(l) = limit {
            query_params.push(format!("limit={}", l));
        }
        if let Some(o) = offset {
            query_params.push(format!("offset={}", o));
        }

        if !query_params.is_empty() {
            url.push_str("?");
            url.push_str(&query_params.join("&"));
        }

        println!("Requesting URL: {}", url);
        let response = self.client.get(&url)
            .header("X-Dune-Api-Key", &self.api_key)
            .send()
            .await?;

        let status = response.status();
        let response_text = response.text().await?;
        
        println!("\n--- Raw CACHED RESULTS Response for Query ID: {} ---\n{}", query_id, response_text);
        return Ok(response_text);
    }
}