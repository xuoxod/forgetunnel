use crate::error::ReportError;
use crate::formatters::ReportFormatter;
use crate::models::ReportDocument;
use crate::storyteller::NarrativeStoryteller;

pub struct AsciiFormatter;

impl ReportFormatter for AsciiFormatter {
    fn render(doc: &ReportDocument) -> Result<String, ReportError> {
        let mut out = String::new();
        out.push_str("================================================================================\n");
        out.push_str(&format!("  🛰️  FORGETUNNEL SOVEREIGN GATEWAY REPORT — Host: {}\n", doc.metadata.host));
        out.push_str(&format!("  Environment: {} | Version: {} | Generated: {}\n", doc.metadata.environment, doc.metadata.version, doc.metadata.generated_at_utc));
        out.push_str("================================================================================\n\n");

        out.push_str(">> EXECUTIVE PERFORMANCE SUMMARY\n");
        out.push_str("--------------------------------------------------------------------------------\n");
        out.push_str(&NarrativeStoryteller::generate_summary(doc));
        out.push_str("\n\n");

        out.push_str(">> AGGREGATE METRICS\n");
        out.push_str("--------------------------------------------------------------------------------\n");
        out.push_str(&format!("  • Total AI Sessions:       {}\n", doc.summary.total_ai_sessions));
        out.push_str(&format!("  • Total Prompt Tokens:     {}\n", doc.summary.total_prompt_tokens));
        out.push_str(&format!("  • Total Completion Tokens: {}\n", doc.summary.total_completion_tokens));
        out.push_str(&format!("  • Average Latency (TTFT):  {:.1} ms\n", doc.summary.avg_ttft_ms));
        out.push_str(&format!("  • Average Token Velocity:  {:.1} tokens/sec\n", doc.summary.avg_tps));
        out.push_str(&format!("  • Intercepted Webhooks:    {} (Valid: {}, Replays: {})\n", doc.summary.total_webhooks, doc.summary.valid_webhooks, doc.summary.total_replays));
        out.push_str(&format!("  • Confirmed Blocks:        {}\n\n", doc.summary.total_blocks));

        if !doc.ai_sessions.is_empty() {
            out.push_str(">> RECENT LOCAL AI INVOCATIONS\n");
            out.push_str("--------------------------------------------------------------------------------\n");
            out.push_str(&format!("{:<20} {:<20} {:<15} {:<12} {:<10}\n", "SESSION ID", "MODEL", "TOKENS (P/C)", "TTFT", "TPS"));
            for s in &doc.ai_sessions {
                out.push_str(&format!("{:<20} {:<20} {:<15} {:<12.1} {:<10.1}\n",
                    s.session_id.chars().take(18).collect::<String>(),
                    s.model.chars().take(18).collect::<String>(),
                    format!("{}/{}", s.prompt_tokens, s.completion_tokens),
                    s.ttft_ms,
                    s.tps
                ));
            }
            out.push_str("\n");
        }

        if !doc.webhooks.is_empty() {
            out.push_str(">> RECENT INTERCEPTED WEBHOOKS\n");
            out.push_str("--------------------------------------------------------------------------------\n");
            out.push_str(&format!("{:<22} {:<12} {:<24} {:<10}\n", "EVENT ID", "PROVIDER", "EVENT TYPE", "SIGNATURE"));
            for w in &doc.webhooks {
                out.push_str(&format!("{:<22} {:<12} {:<24} {:<10}\n",
                    w.event_id.chars().take(20).collect::<String>(),
                    w.provider,
                    w.event_type.chars().take(22).collect::<String>(),
                    if w.signature_valid { "VALID" } else { "INVALID" }
                ));
            }
            out.push_str("\n");
        }

        out.push_str("================================================================================\n");
        Ok(out)
    }
}
