use anyhow::Result;
use datafusion::{arrow::array::AsArray, execution::context::SessionContext};

const PQ_FILE: &str = "../assets/sample.parquet";

#[tokio::main]
async fn main() -> Result<()> {
  read_with_datafusion(PQ_FILE).await?;
  Ok(())
}

async fn read_with_datafusion(file: &str) -> Result<()> {
  let ctx = SessionContext::new();
  ctx
    .register_parquet("stats", file, Default::default())
    .await?;

  let ret = ctx
    .sql("SELECT name::text name, email::text email FROM stats limit 3")
    .await?
    .collect()
    .await?;

  for batch in ret {
    let names = batch.column(0).as_string::<i32>();
    let emails = batch.column(1).as_string::<i32>();

    for (name, email) in names.iter().zip(emails.iter()) {
      let (name, email) = (name.unwrap(), email.unwrap());
      println!("{}: {}", name, email);
    }
  }
  Ok(())
}
