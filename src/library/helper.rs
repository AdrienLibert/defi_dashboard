use anyhow::Result;
use std::fs::{File, read_to_string};
use std::io::{BufWriter, BufReader};
use apache_avro::{Schema, Writer, Reader};
use std::path::Path;
use crate::library::models::DuneQueryRow;

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

pub async fn read_from_avro<P: AsRef<Path>>(file_path: P) -> Result<Vec<DuneQueryRow>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let avro_reader = Reader::new(reader)?;
    let mut rows = Vec::new();
    for value in avro_reader {
        let row = apache_avro::from_value::<DuneQueryRow>(&value?)?;
        rows.push(row);
    }

    println!("Successfully read {} rows from Avro file", rows.len());
    Ok(rows)
}