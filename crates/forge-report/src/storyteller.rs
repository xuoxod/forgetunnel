use crate::models::ReportDocument;

/// Executive Narrative Storyteller for operational summaries
pub struct NarrativeStoryteller;

impl NarrativeStoryteller {
    pub fn generate_summary(doc: &ReportDocument) -> String {
        let total_tokens = doc.summary.total_prompt_tokens + doc.summary.total_completion_tokens;
        let ai_count = doc.summary.total_ai_sessions;
        let wh_count = doc.summary.total_webhooks;

        let ai_status = if ai_count > 0 || total_tokens > 0 {
            format!(
                "processed {} AI streaming invocations across local LLMs with an average TTFT of {:.1}ms and token velocity of {:.1} tokens/sec. Total compute consumed: {} tokens ({} prompt / {} completion).",
                ai_count, doc.summary.avg_ttft_ms, doc.summary.avg_tps, total_tokens, doc.summary.total_prompt_tokens, doc.summary.total_completion_tokens
            )
        } else {
            "no AI streaming sessions recorded in this reporting period.".to_string()
        };

        let wh_status = if wh_count > 0 {
            format!(
                "intercepted {} webhooks ({} valid signatures, {} replays executed).",
                wh_count, doc.summary.valid_webhooks, doc.summary.total_replays
            )
        } else {
            "zero webhook payloads received.".to_string()
        };

        format!(
            "ForgeTunnel Gateway on host '{}' ({}) has {}\nAdditionally, the gateway has {}\nCryptographic ledger status: {} tamper-evident SHA-256 blocks confirmed in SQLite WAL storage.",
            doc.metadata.host, doc.metadata.environment, ai_status, wh_status, doc.summary.total_blocks
        )
    }
}
