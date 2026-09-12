use crate::error::ReportError;
use crate::formatters::ReportFormatter;
use crate::models::ReportDocument;

pub struct JsonFormatter;

impl ReportFormatter for JsonFormatter {
    fn render(doc: &ReportDocument) -> Result<String, ReportError> {
        serde_json::to_string_pretty(doc).map_err(ReportError::Serialization)
    }
}

pub struct JsonlFormatter;

impl ReportFormatter for JsonlFormatter {
    fn render(doc: &ReportDocument) -> Result<String, ReportError> {
        let mut out = String::new();
        for s in &doc.ai_sessions {
            let line = serde_json::to_string(s).map_err(ReportError::Serialization)?;
            out.push_str(&line);
            out.push('\n');
        }
        for w in &doc.webhooks {
            let line = serde_json::to_string(w).map_err(ReportError::Serialization)?;
            out.push_str(&line);
            out.push('\n');
        }
        Ok(out)
    }
}
