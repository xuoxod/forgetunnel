use forge_core::ForgeConfig;
use forge_ledger::{AiSessionRecord, ForgeLedger, WebhookRecord};
use forge_report::{AsciiFormatter, HtmlFormatter, JsonFormatter, MarkdownFormatter, ReportFormatter, ReportOrchestrator};
use tempfile::NamedTempFile;

#[test]
fn test_rw_cli_generate_config_template() {
    let template = ForgeConfig::generate_default_template();
    assert!(template.contains("node_label = \"hyperion-prime\""));
    assert!(template.contains("relay_url = \"wss://relay.example.com:8084/ws/tunnel\""));
    assert!(template.contains("name = \"ollama\""));
    assert!(template.contains("name = \"webhooks\""));
}

#[test]
fn test_rw_cli_report_generation_from_ledger() {
    let tmp = NamedTempFile::new().unwrap();
    let db_path = tmp.path().to_path_buf();

    let ledger = ForgeLedger::open(&db_path).expect("Failed to open DB");
    ledger.record_ai_session(&AiSessionRecord {
        session_id: "cli_sess_01".into(),
        model: "llama3:latest".into(),
        prompt_preview: "CLI report test prompt".into(),
        prompt_tokens: 25,
        completion_tokens: 120,
        ttft_ms: 22.0,
        tps: 55.0,
        duration_ms: 2180.0,
        status: "COMPLETED".into(),
        created_at: "2026-09-11T20:20:00Z".into(),
    }).unwrap();

    ledger.record_webhook(&WebhookRecord {
        event_id: "cli_wh_01".into(),
        provider: "Stripe".into(),
        event_type: "charge.succeeded".into(),
        signature_header: "sig_abc".into(),
        signature_valid: true,
        payload_bytes: 512,
        payload_body: "{}".into(),
        replay_count: 0,
        status_code: 200,
        created_at: "2026-09-11T20:21:00Z".into(),
    }).unwrap();

    let orchestrator = ReportOrchestrator::new(&ledger);
    let doc = orchestrator.build_report_document("cli-test-node", "testing", 120).unwrap();

    let ascii = AsciiFormatter::render(&doc).unwrap();
    assert!(ascii.contains("cli_sess_01"));
    assert!(ascii.contains("cli_wh_01"));

    let html = HtmlFormatter::render(&doc).unwrap();
    assert!(html.contains("cli_sess_01"));
    assert!(html.contains("<!DOCTYPE html>"));

    let json = JsonFormatter::render(&doc).unwrap();
    assert!(json.contains("\"total_ai_sessions\": 1"));

    let md = MarkdownFormatter::render(&doc).unwrap();
    assert!(md.contains("cli_sess_01"));
}

#[test]
fn test_rw_cli_blockchain_audit_verification() {
    let tmp = NamedTempFile::new().unwrap();
    let db_path = tmp.path().to_path_buf();

    let ledger = ForgeLedger::open(&db_path).expect("Failed to open DB");
    let mut chain = forge_telemetry::HashChain::new("cli-audit-seed");

    for i in 0..3 {
        let entry = forge_telemetry::TelemetryProvenance::new(
            format!("user_{}", i),
            "127.0.0.1",
            "127.0.0.1:11434",
            "chat",
            "TLS_WS",
            forge_telemetry::TelemetryMinutiae::default(),
        );
        let block = chain.append(entry);
        ledger.record_block(&block).unwrap();
    }

    let verified = ledger.verify_stored_chain_integrity().unwrap();
    assert_eq!(verified, 3);
}
