use anyhow::Result;
use csv::WriterBuilder;
use std::fs::File;
use std::io::BufWriter;

use crate::api::models::DuneQueryRow;

pub async fn write_dune_rows_to_csv(rows: &[DuneQueryRow], file_path: &str) -> Result<()> {
    let file = File::create(file_path)?;
    let writer = BufWriter::new(file);

    let mut csv_writer = WriterBuilder::new()
        .has_headers(true)
        .from_writer(writer);

    for row in rows {
        csv_writer.serialize(row)?;
    }

    csv_writer.flush()?;
    println!("Data successfully written to {}", file_path);
    Ok(())
}