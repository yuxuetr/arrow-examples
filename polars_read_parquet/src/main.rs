use anyhow::Result;
use polars::{prelude::*, sql::SQLContext};

const PQ_FILE: &str = "../assets/sample.parquet";

fn main() -> Result<()> {
  read_with_polars(PQ_FILE)?;
  Ok(())
}

fn read_with_polars(file: &str) -> Result<()> {
  let df = LazyFrame::scan_parquet(file, Default::default())?;
  let mut ctx = SQLContext::new();

  ctx.register("stats", df);
  let df = ctx
    .execute("SELECT name::text name, email::text email FROM stats")?
    .collect()?;
  println!("{:?}", df);

  Ok(())
}
