mod path_getter;
mod excel_reader;

use polars::prelude::*;
use calamine_to_polars::ToPolarsDataFrame;

fn main() {
    /*
    Plan:
    1. Get xlsx path           J
    2. Read data               J
    3. Process data            X
    4. Create visualizations   X
    5. Create output pdf       X
    */

    let path_buf = path_getter::get_path().expect("Couldn't open file.");
    let mut reader = excel_reader::new(path_buf);
    let df = reader.open_sheet("Sheet1").unwrap().to_df().unwrap();
    println!("{:#?}", df);
}
