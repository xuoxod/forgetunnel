use crate::error::ReportError;
use crate::formatters::ReportFormatter;
use crate::models::ReportDocument;
use crate::sanitizer::sanitize_csv_cell;

pub struct CsvFormatter;

impl ReportFormatter for CsvFormatter {
    fn render(doc: &ReportDocument) -> Result<String, ReportError> {
        let mut out = String::new();
        out.push_str("Record_Type,ID,Model_Provider,Event_Action,Prompt_Tokens,Completion_Tokens,TTFT_ms,TPS,Status,Timestamp_UTC\n");

        for s in &doc.ai_sessions {
            out.push_str(&format!("AI_SESSION,{},{},{},{},{},{:.1},{:.1},{},{}\n",
                sanitize_csv_cell(&s.session_id),
                sanitize_csv_cell(&s.model),
                sanitize_csv_cell(&s.prompt_preview),
                s.prompt_tokens,
                s.completion_tokens,
                s.ttft_ms,
                s.tps,
                sanitize_csv_cell(&s.status),
                sanitize_csv_cell(&s.created_at)
            ));
        }

        for w in &doc.webhooks {
            out.push_str(&format!("WEBHOOK,{},{},{},0,0,0.0,0.0,{},{}\n",
                sanitize_csv_cell(&w.event_id),
                sanitize_csv_cell(&w.provider),
                sanitize_csv_cell(&w.event_type),
                if w.signature_valid { "VALID_SIGNATURE" } else { "INVALID_SIGNATURE" },
                sanitize_csv_cell(&w.created_at)
            ));
        }

        Ok(out)
    }
}
