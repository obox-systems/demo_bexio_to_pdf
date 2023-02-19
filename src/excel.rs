use std::path::Path;

use anyhow::{Result, Ok};
use calamine::{open_workbook, Xlsx, Reader};

use crate::report::Report;

pub struct ExcelExtractor{}

impl ExcelExtractor {
  pub fn new() -> Self {
    Self {}
  }

  pub fn extract_workers(&self, path: impl AsRef<Path>) -> Result<Vec<Report>> {
    let mut list = vec![];
    let mut book: Xlsx<_> = open_workbook(path)?;
    let range = book.worksheet_range_at(0).unwrap()?;
    for row in range.rows().skip(1) {
      let name = row.get(0).unwrap().get_string().unwrap();
      let hours = row.get(1).unwrap().get_float().unwrap();
      list.push(Report::new(name, hours as i64));
    }
    Ok(list)
  }
}