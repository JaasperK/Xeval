use std::path::PathBuf;
use calamine_to_polars::CalamineToPolarsReader;

pub fn new(p: PathBuf) -> CalamineToPolarsReader {
    CalamineToPolarsReader::new(p.as_path())
}
