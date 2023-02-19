use std::path::Path;

use genpdf::{elements::{Paragraph, Break}, Alignment, style::Style, Document, SimplePageDecorator, Element};

pub struct Report {
  pub name: String,
  pub hours: i64,
}

impl Report {
  pub fn new(name: &str, hours: i64) -> Self {
    Self { name: name.into(), hours }
  }

  pub fn export_to_pdf_by_name(&self) {
    self.export_to_pdf(format!("{}.pdf", self.name));
  }

  pub fn export_to_pdf(&self, path: impl AsRef<Path>) {
    let font = genpdf::fonts::from_files("./fonts", "LiberationSans", None).unwrap();
    let mut document = Document::new(font);
    document.set_title("Work summary");
    let mut decorator = SimplePageDecorator::new();
    decorator.set_margins(10);
    document.set_page_decorator(decorator);
    document.push(
      Paragraph::new("Work summary")
        .aligned(Alignment::Center)
        .styled(Style::new().bold().with_font_size(25))
    );
    document.push(Break::new(1.5));
    document.push(
      Paragraph::new("Hello there, ")
        .styled_string(&self.name, Style::new().bold())
        .string(", here's you work summary for this week: ")
        .styled_string(self.hours.to_string(), Style::new().bold())
        .string(" hours!")
    );
    document.render_to_file(path).unwrap();
  }
}