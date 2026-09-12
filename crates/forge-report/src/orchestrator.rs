use crate::error::ReportError;
use crate::models::{ReportDocument, SystemMetadata};
use chrono::Utc;
use forge_ledger::ForgeLedger;

pub struct ReportOrchestrator<'a> {
    ledger: &'a ForgeLedger,
}

impl<'a> ReportOrchestrator<'a> {
    pub fn new(ledger: &'a ForgeLedger) -> Self {
        Self { ledger }
    }

    pub fn build_report_document(
        &self,
        host: &str,
        environment: &str,
        uptime_seconds: u64,
    ) -> Result<ReportDocument, ReportError> {
        let metadata = SystemMetadata {
            host: host.to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            environment: environment.to_string(),
            generated_at_utc: Utc::now().to_rfc3339(),
            uptime_seconds,
        };

        let summary = self.ledger.get_summary_stats()?;
        let ai_sessions = self.ledger.query_ai_sessions(100)?;
        let webhooks = self.ledger.query_webhooks(100)?;
        let blocks = self.ledger.query_blocks(100)?;

        Ok(ReportDocument {
            metadata,
            summary,
            ai_sessions,
            webhooks,
            blocks,
        })
    }
}
