use defi_data_pipeline::library::helper::read_from_avro;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let file_path = concat!(env!("CARGO_MANIFEST_DIR"), "/avro_data/query_results.avro");
    let rows = read_from_avro(file_path).await?;
    for row in rows {
        println!("Read row: {:?}", row);
    }
    Ok(())
}