use forge_ledger::{AiSessionRecord, ForgeLedger, WebhookRecord};
use forge_report::{HtmlFormatter, MarkdownFormatter, ReportFormatter, ReportOrchestrator};

#[test]
fn test_sim_chaos_100_ai_sessions_and_50_webhooks_pipeline() {
    let ledger = ForgeLedger::open_in_memory().expect("Failed to open in-memory ledger");

    // Ingest 100 AI sessions with diverse models and metrics
    let models = ["llama3:latest", "mistral:7b", "qwen2.5-coder:14b", "deepseek-coder:6.7b", "gemma2:9b"];
    for i in 0..100 {
        let model = models[i % models.len()];
        let prompt_tokens = (10 + (i * 7) % 500) as u64;
        let completion_tokens = (50 + (i * 13) % 2000) as u64;
        let ttft_ms = 15.0 + ((i * 3) % 40) as f64;
        let tps = 35.0 + ((i * 5) % 60) as f64;

        ledger.record_ai_session(&AiSessionRecord {
            session_id: format!("sim_sess_{:04}", i),
            model: model.to_string(),
            prompt_preview: format!("Simulated prompt request number {}", i),
            prompt_tokens,
            completion_tokens,
            ttft_ms,
            tps,
            duration_ms: (completion_tokens as f64 / tps) * 1000.0,
            status: if i % 15 == 0 { "ERROR".into() } else { "COMPLETED".into() },
            created_at: format!("2026-09-11T20:{:02}:{:02}Z", (i / 60) % 60, i % 60),
        }).unwrap();
    }

    // Ingest 50 Webhook events (Stripe, GitHub, Shopify, Slack)
    let providers = [("Stripe", "charge.succeeded"), ("GitHub", "push"), ("Shopify", "orders/create"), ("Slack", "event_callback")];
    for i in 0..50 {
        let (prov, evt) = providers[i % providers.len()];
        let valid = i % 10 != 0; // 10% invalid signatures
        ledger.record_webhook(&WebhookRecord {
            event_id: format!("sim_evt_{:04}", i),
            provider: prov.to_string(),
            event_type: evt.to_string(),
            signature_header: format!("sig_mock_{}", i),
            signature_valid: valid,
            payload_bytes: 512 + (i * 32),
            payload_body: format!("{{\"sim_index\":{},\"event\":\"{}\"}}", i, evt),
            replay_count: if i % 4 == 0 { 1 } else { 0 },
            status_code: if valid { 200 } else { 401 },
            created_at: format!("2026-09-11T20:{:02}:{:02}Z", (i / 60) % 60, i % 60),
        }).unwrap();
    }

    // Orchestrate report from ledger
    let orchestrator = ReportOrchestrator::new(&ledger);
    let doc = orchestrator.build_report_document("sim-cluster-01", "production", 3600).expect("Failed to build report document");

    assert_eq!(doc.summary.total_ai_sessions, 100);
    assert_eq!(doc.summary.total_webhooks, 50);
    assert!(doc.summary.avg_ttft_ms > 0.0);
    assert!(doc.summary.avg_tps > 0.0);

    // Verify HTML rendering scales cleanly
    let html = HtmlFormatter::render(&doc).expect("HTML render failed");
    assert!(html.contains("sim_sess_0000"));
    assert!(html.contains("sim_sess_0099"));
    assert!(html.contains("sim_evt_0000"));
    assert!(html.contains("sim_evt_0049"));

    // Verify Markdown rendering
    let md = MarkdownFormatter::render(&doc).expect("Markdown render failed");
    assert!(md.contains("Total AI Sessions") && md.contains("`100`"));
    assert!(md.contains("Total Webhooks") && md.contains("`50`"));
}
