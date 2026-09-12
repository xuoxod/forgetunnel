pub mod ascii;
pub mod csv;
pub mod html;
pub mod json;
pub mod markdown;

pub use ascii::AsciiFormatter;
pub use csv::CsvFormatter;
pub use html::HtmlFormatter;
pub use json::{JsonFormatter, JsonlFormatter};
pub use markdown::MarkdownFormatter;

use crate::error::ReportError;
use crate::models::ReportDocument;

pub trait ReportFormatter {
    fn render(doc: &ReportDocument) -> Result<String, ReportError>;
}
