use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Serialize)]
pub struct DuneQueryRow {
    pub block_time: DateTime<Utc>,
    pub from: String,
    pub to: String,
    pub formatted_value: f64,
    pub blockchain: String,
}