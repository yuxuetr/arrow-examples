use anyhow::Result;
use arrow::array::AsArray;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use std::fs::File;

const PQ_FILE: &str = "../assets/sample.parquet";

fn main() -> Result<()> {
  read_with_parquet(PQ_FILE)?;
  Ok(())
}

fn read_with_parquet(file: &str) -> Result<()> {
  let file = File::open(file)?;
  let reader = ParquetRecordBatchReaderBuilder::try_new(file)?
    .with_batch_size(8192)
    .with_limit(3)
    .build()?;

  for record_batch in reader {
    let record_batch = record_batch?;
    let emails = record_batch.column(0).as_binary::<i32>();

    for email in emails {
      let email = email.unwrap();
      println!("{:?}", String::from_utf8_lossy(email));
    }
  }
  Ok(())
}
