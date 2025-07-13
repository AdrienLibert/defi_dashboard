use serde::{Deserialize, Deserializer, Serialize, Serializer};
use chrono::{DateTime, Utc, NaiveDateTime};

fn deserialize_block_time<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let naive_datetime = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S%.3f UTC")
        .map_err(serde::de::Error::custom)?;

    Ok(naive_datetime.and_utc())
}

fn serialize_block_time_to_avro<S>(datetime: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_i64(datetime.timestamp_millis())
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct DuneQueryRow {
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub block_time: DateTime<Utc>,
    pub token_pair: String,
    pub token_bought_symbol: String,
    pub token_sold_symbol: String,
    pub total_bought_amount: f64,
    pub total_sold_amount: f64,
    pub total_volume_usd: f64,
    pub project_contract_address: String,
    pub swap_count: i64,
}