use crate::error::ReportError;
use crate::formatters::ReportFormatter;
use crate::models::ReportDocument;
use crate::storyteller::NarrativeStoryteller;

pub struct MarkdownFormatter;

impl ReportFormatter for MarkdownFormatter {
    fn render(doc: &ReportDocument) -> Result<String, ReportError> {
        let mut out = String::new();
        out.push_str(&format!("# 🛰️ ForgeTunnel Sovereign Gateway — `{}`\n\n", doc.metadata.host));
        out.push_str(&format!("> **Environment:** `{}` | **Version:** `v{}` | **Generated UTC:** `{}`\n\n", doc.metadata.environment, doc.metadata.version, doc.metadata.generated_at_utc));

        out.push_str("## 📋 Executive Operational Summary\n\n```text\n");
        out.push_str(&NarrativeStoryteller::generate_summary(doc));
        out.push_str("\n```\n\n---\n\n");

        out.push_str("## 📊 Token Velocity & Telemetry Metrics\n\n");
        out.push_str("| Metric | Value | Description |\n");
        out.push_str("| :--- | :--- | :--- |\n");
        out.push_str(&format!("| **Total AI Sessions** | `{}` | Local Ollama/vLLM streaming invocations |\n", doc.summary.total_ai_sessions));
        out.push_str(&format!("| **Total Prompt Tokens** | `{}` | Inbound token consumption |\n", doc.summary.total_prompt_tokens));
        out.push_str(&format!("| **Total Completion Tokens** | `{}` | Outbound token generation |\n", doc.summary.total_completion_tokens));
        out.push_str(&format!("| **Average TTFT** | `{:.1} ms` | Time to First Token |\n", doc.summary.avg_ttft_ms));
        out.push_str(&format!("| **Average Velocity** | `{:.1} TPS` | Tokens Per Second |\n", doc.summary.avg_tps));
        out.push_str(&format!("| **Total Webhooks** | `{}` | Intercepted webhook events |\n", doc.summary.total_webhooks));
        out.push_str(&format!("| **Valid Signatures** | `{}` | HMAC-SHA256 verified payloads |\n", doc.summary.valid_webhooks));
        out.push_str(&format!("| **Replays Executed** | `{}` | Local developer webhook replays |\n", doc.summary.total_replays));
        out.push_str(&format!("| **Blockchain Blocks** | `{}` | Tamper-evident ledger blocks |\n\n", doc.summary.total_blocks));

        out.push_str("## 🤖 Local AI Streaming Invocations\n\n");
        if doc.ai_sessions.is_empty() {
            out.push_str("_No AI sessions recorded._\n\n");
        } else {
            out.push_str("| Session ID | Model | Prompt Tokens | Completion Tokens | TTFT | TPS | Status |\n");
            out.push_str("| :--- | :--- | :--- | :--- | :--- | :--- | :--- |\n");
            for s in &doc.ai_sessions {
                out.push_str(&format!("| `{}` | `{}` | `{}` | `{}` | `{:.1} ms` | `{:.1}` | `{}` |\n",
                    s.session_id, s.model, s.prompt_tokens, s.completion_tokens, s.ttft_ms, s.tps, s.status
                ));
            }
            out.push_str("\n");
        }

        out.push_str("## 🪝 Webhook Interceptions & Replays\n\n");
        if doc.webhooks.is_empty() {
            out.push_str("_No webhooks recorded._\n\n");
        } else {
            out.push_str("| Event ID | Provider | Event Type | Signature | Payload Size | Replays |\n");
            out.push_str("| :--- | :--- | :--- | :--- | :--- | :--- |\n");
            for w in &doc.webhooks {
                out.push_str(&format!("| `{}` | `{}` | `{}` | `{}` | `{} B` | `{}` |\n",
                    w.event_id, w.provider, w.event_type, if w.signature_valid { "VALID" } else { "INVALID" }, w.payload_bytes, w.replay_count
                ));
            }
            out.push_str("\n");
        }

        out.push_str("---\n\n© 2026 **ForgeTunnel Sovereign Gateway**. All rights reserved.\n");
        Ok(out)
    }
}
