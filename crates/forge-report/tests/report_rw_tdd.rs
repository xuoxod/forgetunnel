use forge_ledger::{AiSessionRecord, BlockRecord, LedgerSummaryStats, WebhookRecord};
use forge_report::{
    AsciiFormatter, CsvFormatter, HtmlFormatter, JsonFormatter, JsonlFormatter,
    MarkdownFormatter, ReportDocument, ReportFormatter, SystemMetadata,
};

fn sample_report_document() -> ReportDocument {
    let metadata = SystemMetadata {
        host: "xuoux".to_string(),
        version: "0.1.0".to_string(),
        environment: "production".to_string(),
        generated_at_utc: "2026-09-11T20:15:00Z".to_string(),
        uptime_seconds: 86400,
    };

    let summary = LedgerSummaryStats {
        total_ai_sessions: 2,
        total_prompt_tokens: 64,
        total_completion_tokens: 512,
        avg_ttft_ms: 32.5,
        avg_tps: 65.4,
        total_webhooks: 1,
        valid_webhooks: 1,
        total_replays: 2,
        total_blocks: 3,
    };

    let ai_sessions = vec![
        AiSessionRecord {
            session_id: "sess_llama3_01".to_string(),
            model: "llama3:latest".to_string(),
            prompt_preview: "Explain quantum error correction".to_string(),
            prompt_tokens: 32,
            completion_tokens: 256,
            ttft_ms: 30.0,
            tps: 68.0,
            duration_ms: 3800.0,
            status: "COMPLETED".to_string(),
            created_at: "2026-09-11T20:10:00Z".to_string(),
        },
        AiSessionRecord {
            session_id: "sess_qwen_02".to_string(),
            model: "qwen2.5-coder:7b".to_string(),
            prompt_preview: "Write a Rust zero-allocation parser".to_string(),
            prompt_tokens: 32,
            completion_tokens: 256,
            ttft_ms: 35.0,
            tps: 62.8,
            duration_ms: 4100.0,
            status: "COMPLETED".to_string(),
            created_at: "2026-09-11T20:12:00Z".to_string(),
        },
    ];

    let webhooks = vec![
        WebhookRecord {
            event_id: "evt_stripe_001".to_string(),
            provider: "Stripe".to_string(),
            event_type: "payment_intent.succeeded".to_string(),
            signature_header: "t=1789094000,v1=abc123".to_string(),
            signature_valid: true,
            payload_bytes: 1024,
            payload_body: r#"{"id":"pi_01","amount":9900}"#.to_string(),
            replay_count: 2,
            status_code: 200,
            created_at: "2026-09-11T20:14:00Z".to_string(),
        },
    ];

    let blocks = vec![
        BlockRecord {
            index: 0,
            block_hash: "a1b2c3d4e5f601020304050607080900112233445566778899aabbccddeeff00".to_string(),
            prev_hash: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            payload_digest: "11223344556677889900aabbccddeeff0011223344556677889900aabbccddeeff".to_string(),
            who: "tenant:dev-admin".to_string(),
            source_addr: "192.168.1.160:54321".to_string(),
            dest_addr: "127.0.0.1:11434".to_string(),
            action: "ollama:generate:llama3".to_string(),
            transport: "TLS_WS_CONDUIT_V1".to_string(),
            safety_tier: "Tier2".to_string(),
            prompt_tokens: 32,
            completion_tokens: 256,
            ttft_ms: 30.0,
            tps: 68.0,
            duration_ps: 3_800_000_000_000,
            timestamp_utc: "2026-09-11T20:10:00Z".to_string(),
            raw_json: "{}".to_string(),
        },
    ];

    ReportDocument {
        metadata,
        summary,
        ai_sessions,
        webhooks,
        blocks,
    }
}

#[test]
fn test_rw_html_formatter_native_and_responsive() {
    let doc = sample_report_document();
    let html = HtmlFormatter::render(&doc).expect("HTML render failed");

    // Must be valid HTML5 document
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("<html lang=\"en\">"));
    assert!(html.contains("</html>"));

    // Must contain separated style and script blocks, NOT inline styles/scripts on tags
    assert!(html.contains("<style>"));
    assert!(html.contains("</style>"));
    assert!(html.contains("<script>"));
    assert!(html.contains("</script>"));

    // Content checks
    assert!(html.contains("ForgeTunnel Sovereign Gateway"));
    assert!(html.contains("llama3:latest"));
    assert!(html.contains("qwen2.5-coder:7b"));
    assert!(html.contains("evt_stripe_001"));
    assert!(html.contains("65.4")); // Avg TPS
}

#[test]
fn test_rw_ascii_formatter_ansi_tables() {
    let doc = sample_report_document();
    let ascii = AsciiFormatter::render(&doc).expect("ASCII render failed");

    assert!(ascii.contains("FORGETUNNEL SOVEREIGN GATEWAY REPORT"));
    assert!(ascii.contains("llama3:latest"));
    assert!(ascii.contains("evt_stripe_001"));
    assert!(ascii.contains("Total AI Sessions"));
}

#[test]
fn test_rw_markdown_formatter_github_flavor() {
    let doc = sample_report_document();
    let md = MarkdownFormatter::render(&doc).expect("Markdown render failed");

    assert!(md.contains("# 🛰️ ForgeTunnel Sovereign Gateway"));
    assert!(md.contains("| Session ID | Model |"));
    assert!(md.contains("| Event ID | Provider |"));
    assert!(md.contains("```"));
}

#[test]
fn test_rw_csv_formatter_rfc4180() {
    let doc = sample_report_document();
    let csv = CsvFormatter::render(&doc).expect("CSV render failed");

    let lines: Vec<&str> = csv.lines().collect();
    assert!(lines.len() >= 2);
    assert!(lines[0].contains("Record_Type,ID,Model_Provider,Event_Action,Prompt_Tokens,Completion_Tokens,TTFT_ms,TPS,Status,Timestamp_UTC"));
    assert!(csv.contains("AI_SESSION,sess_llama3_01,llama3:latest"));
    assert!(csv.contains("WEBHOOK,evt_stripe_001,Stripe"));
}

#[test]
fn test_rw_json_and_jsonl_formatters() {
    let doc = sample_report_document();
    let json = JsonFormatter::render(&doc).expect("JSON render failed");
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("Invalid JSON output");
    assert_eq!(parsed["summary"]["total_ai_sessions"], 2);

    let jsonl = JsonlFormatter::render(&doc).expect("JSONL render failed");
    let jsonl_lines: Vec<&str> = jsonl.lines().filter(|l| !l.trim().is_empty()).collect();
    assert_eq!(jsonl_lines.len(), 3); // 2 AI sessions + 1 Webhook
}
