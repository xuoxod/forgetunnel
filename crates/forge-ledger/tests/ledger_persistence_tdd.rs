use forge_ledger::{AiSessionRecord, ForgeLedger, WebhookRecord};
use tempfile::NamedTempFile;

#[test]
fn test_rw_ai_session_persistence_and_query() {
    let ledger = ForgeLedger::open_in_memory().expect("Failed to open in-memory ledger");

    let session = AiSessionRecord {
        session_id: "sess_llama3_001".to_string(),
        model: "llama3:latest".to_string(),
        prompt_preview: "Explain quantum computing in simple terms".to_string(),
        prompt_tokens: 32,
        completion_tokens: 256,
        ttft_ms: 28.5,
        tps: 58.2,
        duration_ms: 4400.0,
        status: "COMPLETED".to_string(),
        created_at: "2026-09-11T20:00:00Z".to_string(),
    };

    ledger.record_ai_session(&session).expect("Failed to record session");

    let sessions = ledger.query_ai_sessions(10).expect("Failed to query sessions");
    assert_eq!(sessions.len(), 1);
    assert_eq!(sessions[0].session_id, "sess_llama3_001");
    assert_eq!(sessions[0].prompt_tokens, 32);
    assert_eq!(sessions[0].completion_tokens, 256);
    assert_eq!(sessions[0].ttft_ms, 28.5);
    assert_eq!(sessions[0].tps, 58.2);
}

#[test]
fn test_rw_webhook_event_interception_and_replay_increment() {
    let ledger = ForgeLedger::open_in_memory().expect("Failed to open in-memory ledger");

    let webhook = WebhookRecord {
        event_id: "evt_stripe_charge_001".to_string(),
        provider: "Stripe".to_string(),
        event_type: "payment_intent.succeeded".to_string(),
        signature_header: "t=1789094000,v1=abc123def456".to_string(),
        signature_valid: true,
        payload_bytes: 1024,
        payload_body: r#"{"id":"pi_123","amount":5000}"#.to_string(),
        replay_count: 0,
        status_code: 200,
        created_at: "2026-09-11T20:05:00Z".to_string(),
    };

    ledger.record_webhook(&webhook).expect("Failed to record webhook");

    let webhooks = ledger.query_webhooks(10).expect("Failed to query webhooks");
    assert_eq!(webhooks.len(), 1);
    assert_eq!(webhooks[0].replay_count, 0);

    // Replay twice
    let count1 = ledger.increment_webhook_replay("evt_stripe_charge_001").expect("Replay 1 failed");
    assert_eq!(count1, 1);
    let count2 = ledger.increment_webhook_replay("evt_stripe_charge_001").expect("Replay 2 failed");
    assert_eq!(count2, 2);

    let updated = ledger.query_webhooks(10).expect("Failed to query webhooks");
    assert_eq!(updated[0].replay_count, 2);
}

#[test]
fn test_rw_summary_stats_aggregation() {
    let ledger = ForgeLedger::open_in_memory().expect("Failed to open in-memory ledger");

    // Add 2 AI sessions
    ledger.record_ai_session(&AiSessionRecord {
        session_id: "s1".into(),
        model: "llama3".into(),
        prompt_preview: "p1".into(),
        prompt_tokens: 10,
        completion_tokens: 100,
        ttft_ms: 20.0,
        tps: 50.0,
        duration_ms: 2000.0,
        status: "COMPLETED".into(),
        created_at: "2026-09-11T20:00:00Z".into(),
    }).unwrap();

    ledger.record_ai_session(&AiSessionRecord {
        session_id: "s2".into(),
        model: "llama3".into(),
        prompt_preview: "p2".into(),
        prompt_tokens: 20,
        completion_tokens: 200,
        ttft_ms: 30.0,
        tps: 60.0,
        duration_ms: 3000.0,
        status: "COMPLETED".into(),
        created_at: "2026-09-11T20:01:00Z".into(),
    }).unwrap();

    // Add 1 valid webhook with 1 replay
    ledger.record_webhook(&WebhookRecord {
        event_id: "w1".into(),
        provider: "GitHub".into(),
        event_type: "push".into(),
        signature_header: "sha256=abc".into(),
        signature_valid: true,
        payload_bytes: 512,
        payload_body: "{}".into(),
        replay_count: 1,
        status_code: 200,
        created_at: "2026-09-11T20:02:00Z".into(),
    }).unwrap();

    let stats = ledger.get_summary_stats().expect("Failed to calculate stats");
    assert_eq!(stats.total_ai_sessions, 2);
    assert_eq!(stats.total_prompt_tokens, 30);
    assert_eq!(stats.total_completion_tokens, 300);
    assert_eq!(stats.avg_ttft_ms, 25.0);
    assert_eq!(stats.avg_tps, 55.0);
    assert_eq!(stats.total_webhooks, 1);
    assert_eq!(stats.valid_webhooks, 1);
    assert_eq!(stats.total_replays, 1);
}

#[test]
fn test_ec_disk_persistence_across_reopens() {
    let tmp = NamedTempFile::new().unwrap();
    let db_path = tmp.path().to_path_buf();

    {
        let ledger = ForgeLedger::open(&db_path).expect("Open 1 failed");
        ledger.record_ai_session(&AiSessionRecord {
            session_id: "persist_01".into(),
            model: "mistral".into(),
            prompt_preview: "test persistence".into(),
            prompt_tokens: 15,
            completion_tokens: 45,
            ttft_ms: 18.0,
            tps: 40.0,
            duration_ms: 1100.0,
            status: "COMPLETED".into(),
            created_at: "2026-09-11T20:10:00Z".into(),
        }).unwrap();
    }

    // Reopen from disk
    {
        let ledger = ForgeLedger::open(&db_path).expect("Open 2 failed");
        let sessions = ledger.query_ai_sessions(10).expect("Query failed");
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].session_id, "persist_01");
        assert_eq!(sessions[0].model, "mistral");
    }
}
