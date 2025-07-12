use anyhow::Result;
use std::fs::{File, read_to_string};
use std::io::BufWriter;
use apache_avro::{Schema, Writer};
//use serde::{Deserialize, Deserializer};
//use chrono::{DateTime, Utc, prelude::*};
use crate::api::models::DuneQueryRow;

/*fn deserialize_block_time<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Utc.datetime_from_str(&s, "%Y-%m-%d %H:%M:%S%.3f UTC")
        .map_err(serde::de::Error::custom)
}
*/

pub async fn save_to_avro(rows: &[DuneQueryRow], file_path: &str) -> Result<()> {
    let schema_path = "./schemas/uniswap_trade_schema.json";
    let schema_json = read_to_string(schema_path)?;
    let schema = Schema::parse_str(&schema_json)?;
    let file = File::create(file_path)?;
    let writer_sink = BufWriter::new(file);

    let mut writer = Writer::new(&schema, writer_sink);

    for row in rows {
        writer.append_ser(row)?;
    }

    writer.flush()?; 
    println!("\nData successfully written to Avro file: {}", file_path);
    Ok(())
}