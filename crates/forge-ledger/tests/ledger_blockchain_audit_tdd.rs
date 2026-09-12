use forge_ledger::ForgeLedger;
use forge_telemetry::{HashChain, TelemetryMinutiae, TelemetryProvenance};
use tempfile::NamedTempFile;

#[test]
fn test_rw_blockchain_blocks_stored_and_integrity_verified() {
    let ledger = ForgeLedger::open_in_memory().expect("Open ledger failed");
    let mut chain = HashChain::new("audit-seed");

    for i in 0..5 {
        let entry = TelemetryProvenance::new(
            format!("client_{}", i),
            "127.0.0.1:8080",
            "http://127.0.0.1:11434/api/generate",
            format!("ollama:llama3_chunk_{}", i),
            "TLS_WS_CONDUIT_V1",
            TelemetryMinutiae {
                duration_ps: 2_500_000_000,
                prompt_tokens: 20,
                completion_tokens: 150,
                ttft_ms: 22.4,
                tps: 62.1,
                payload_bytes: 2048,
                safety_tier: "Tier2".to_string(),
                dry_run: false,
                status_code: 200,
                extra: Default::default(),
            },
        );

        let block = chain.append(entry);
        ledger.record_block(&block).expect("Failed to record block");
    }

    let stored_blocks = ledger.query_blocks(10).expect("Query blocks failed");
    assert_eq!(stored_blocks.len(), 5);
    assert_eq!(stored_blocks[0].index, 0);
    assert_eq!(stored_blocks[4].index, 4);

    let verified_count = ledger.verify_stored_chain_integrity().expect("Integrity check failed");
    assert_eq!(verified_count, 5);
}

#[test]
fn test_attack_blockchain_tampered_in_sqlite_detected() {
    let tmp = NamedTempFile::new().unwrap();
    let db_path = tmp.path().to_path_buf();

    let mut chain = HashChain::new("attack-audit-seed");
    {
        let ledger = ForgeLedger::open(&db_path).expect("Open failed");
        for i in 0..4 {
            let entry = TelemetryProvenance::new(
                format!("client_{}", i),
                "127.0.0.1",
                "http://127.0.0.1:11434",
                "prompt",
                "TLS_WS",
                TelemetryMinutiae::default(),
            );
            let block = chain.append(entry);
            ledger.record_block(&block).expect("Record block failed");
        }
    }

    // Direct malicious SQLite modification simulating rogue DB tampering
    {
        let conn = rusqlite::Connection::open(&db_path).unwrap();
        conn.execute(
            "UPDATE telemetry_blocks SET payload_digest = '0000000000000000000000000000000000000000000000000000000000000000' WHERE block_index = 2",
            [],
        ).unwrap();
    }

    // Verify ledger detects the rogue tampering
    {
        let ledger = ForgeLedger::open(&db_path).expect("Reopen failed");
        let err = ledger.verify_stored_chain_integrity().unwrap_err();
        let err_msg = err.to_string();
        assert!(err_msg.to_lowercase().contains("tamper detected") || err_msg.contains("verification failed"), "Must catch DB tamper: {}", err_msg);
    }
}
