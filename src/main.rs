use bexio_to_pdf::excel::ExcelExtractor;

fn main() {
  let extractor = ExcelExtractor::new();

  if let Ok(workers) = extractor.extract_workers("./bexio-dump.xlsx") {
    for worker in workers {
      println!("Exporting pdf for {}", worker.name);
      worker.export_to_pdf_by_name();
    }
  } else {
    eprintln!("Failed to open bexio dump!")
  }
}