use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};
use crate::api::models::{QueryResult, ExecuteQueryResponse};

const API_BASE_URL: &str = "";

pub struct ClientAPI {
    client: Client,
    api_key: String,
}