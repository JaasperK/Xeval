use calamine_to_polars::{CalamineToPolarsReader, ToPolarsDataFrame};
use polars::prelude::{DataFrame, PolarsError};

pub fn start(mut reader: CalamineToPolarsReader) -> Result<DataFrame, PolarsError> {
    let df = reader.open_sheet("Sheet1").unwrap().to_df().unwrap();
    let max_idx = df.width();  // num_columns
    let ser = df.select_at_idx(6).unwrap();
    let a = ser.f64()?;
    Ok(df)
 }

fn averages(df: DataFrame) -> Vec<f64> {
    vec![0.]
}

fn std_devs(df: DataFrame) -> Vec<f64> {
    vec![0.]
}

fn freetexts(df: DataFrame) -> Vec<String> {
    vec![String::from("")]
}
