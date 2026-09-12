use forge_ledger::LedgerSummaryStats;
use forge_report::{
    CsvFormatter, HtmlFormatter, JsonFormatter, MarkdownFormatter, ReportDocument,
    ReportFormatter, SystemMetadata,
};

#[test]
fn test_ec_empty_ledger_report_rendering() {
    let doc = ReportDocument {
        metadata: SystemMetadata {
            host: "empty-node".to_string(),
            version: "0.1.0".to_string(),
            environment: "staging".to_string(),
            generated_at_utc: "2026-09-11T20:20:00Z".to_string(),
            uptime_seconds: 0,
        },
        summary: LedgerSummaryStats::default(),
        ai_sessions: Vec::new(),
        webhooks: Vec::new(),
        blocks: Vec::new(),
    };

    let html = HtmlFormatter::render(&doc).unwrap();
    assert!(html.contains("0 Total Invocations") || html.contains("No active AI sessions recorded"));

    let md = MarkdownFormatter::render(&doc).unwrap();
    assert!(md.contains("No AI sessions recorded"));

    let csv = CsvFormatter::render(&doc).unwrap();
    assert!(csv.lines().count() >= 1); // Header line present

    let json = JsonFormatter::render(&doc).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["summary"]["total_ai_sessions"], 0);
}

#[test]
fn test_ec_massive_token_counts_and_zero_ttft() {
    let mut doc = ReportDocument::default();
    doc.summary.total_prompt_tokens = 500_000_000;
    doc.summary.total_completion_tokens = 1_500_000_000;
    doc.summary.avg_ttft_ms = 0.0;
    doc.summary.avg_tps = 1500.75;

    let html = HtmlFormatter::render(&doc).unwrap();
    assert!(html.contains("500,000,000") || html.contains("500000000"));
    assert!(html.contains("1,500,000,000") || html.contains("1500000000"));
}

#[test]
fn test_ec_multilingual_unicode_and_special_characters() {
    let mut doc = ReportDocument::default();
    doc.metadata.host = "测试主机-01".to_string();
    doc.ai_sessions.push(forge_ledger::AiSessionRecord {
        session_id: "sess_zh_01".to_string(),
        model: "通义千问-Qwen:72b".to_string(),
        prompt_preview: "请用Rust编写一个高性能反向代理服务 🚀".to_string(),
        prompt_tokens: 50,
        completion_tokens: 500,
        ttft_ms: 19.5,
        tps: 80.0,
        duration_ms: 6250.0,
        status: "COMPLETED".to_string(),
        created_at: "2026-09-11T20:25:00Z".to_string(),
    });

    let html = HtmlFormatter::render(&doc).unwrap();
    assert!(html.contains("通义千问-Qwen:72b"));
    assert!(html.contains("请用Rust编写一个高性能反向代理服务 🚀"));

    let md = MarkdownFormatter::render(&doc).unwrap();
    assert!(md.contains("通义千问-Qwen:72b"));
}
