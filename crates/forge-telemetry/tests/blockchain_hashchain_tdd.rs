use forge_telemetry::{HashChain, TelemetryMinutiae, TelemetryProvenance};

#[test]
fn test_rw_hashchain_sequential_block_generation() {
    let mut chain = HashChain::new("cluster-alpha-seed");
    assert_eq!(chain.len(), 0);
    assert_eq!(chain.is_empty(), true);

    for i in 0..10 {
        let entry = TelemetryProvenance::new(
            format!("client_{}", i),
            "127.0.0.1:8080",
            "http://127.0.0.1:11434/api/generate",
            format!("ollama:model_{}", i),
            "TLS_WS_CONDUIT_V1",
            TelemetryMinutiae {
                duration_ps: 1_000_000 * (i + 1),
                prompt_tokens: 10 + i,
                completion_tokens: 50 + i,
                ttft_ms: 12.5,
                tps: 45.0,
                payload_bytes: 1024,
                safety_tier: "Tier2".to_string(),
                dry_run: false,
                status_code: 200,
                extra: Default::default(),
            },
        );

        let block = chain.append(entry);
        assert_eq!(block.index, i);
        assert_eq!(block.block_hash.len(), 64);
    }

    assert_eq!(chain.len(), 10);
    assert!(chain.verify_integrity().is_ok(), "Chain integrity verification must succeed");
}

#[test]
fn test_attack_hashchain_payload_tamper_detected() {
    let mut chain = HashChain::new("security-seed");

    for i in 0..5 {
        chain.append(TelemetryProvenance::new(
            format!("user_{}", i),
            "10.0.0.1",
            "http://127.0.0.1:11434",
            "prompt:test",
            "TLS_WS",
            TelemetryMinutiae::default(),
        ));
    }

    // Tamper with block 2's internal payload (change who field)
    let mut tampered_blocks = chain.blocks().to_vec();
    tampered_blocks[2].entry.who = "malicious_attacker".to_string();

    // Reconstruct chain structure with tampered block
    let tampered_chain_json = serde_json::json!({
        "genesis_hash": chain.genesis_hash(),
        "blocks": tampered_blocks
    });

    let reconstructed: HashChain = serde_json::from_value(tampered_chain_json).unwrap();
    let err = reconstructed.verify_integrity().unwrap_err();
    let err_str = err.to_string();
    assert!(err_str.contains("Tamper detected at block index 2"), "Must catch payload tamper: {}", err_str);
}

#[test]
fn test_attack_hashchain_prev_hash_tamper_detected() {
    let mut chain = HashChain::new("security-seed-2");

    for i in 0..5 {
        chain.append(TelemetryProvenance::new(
            format!("user_{}", i),
            "10.0.0.1",
            "http://127.0.0.1:11434",
            "prompt:test",
            "TLS_WS",
            TelemetryMinutiae::default(),
        ));
    }

    let mut tampered_blocks = chain.blocks().to_vec();
    tampered_blocks[3].prev_hash = "0000000000000000000000000000000000000000000000000000000000000000".to_string();

    let tampered_chain_json = serde_json::json!({
        "genesis_hash": chain.genesis_hash(),
        "blocks": tampered_blocks
    });

    let reconstructed: HashChain = serde_json::from_value(tampered_chain_json).unwrap();
    let err = reconstructed.verify_integrity().unwrap_err();
    let err_str = err.to_string();
    assert!(err_str.contains("Invalid previous hash at block index 3"), "Must catch broken hash linkage: {}", err_str);
}

#[test]
fn test_attack_hashchain_sequence_replay_detected() {
    let mut chain = HashChain::new("security-seed-3");

    for i in 0..4 {
        chain.append(TelemetryProvenance::new(
            format!("user_{}", i),
            "10.0.0.1",
            "http://127.0.0.1:11434",
            "prompt:test",
            "TLS_WS",
            TelemetryMinutiae::default(),
        ));
    }

    let mut tampered_blocks = chain.blocks().to_vec();
    // Swap block 1 and block 2
    tampered_blocks.swap(1, 2);

    let tampered_chain_json = serde_json::json!({
        "genesis_hash": chain.genesis_hash(),
        "blocks": tampered_blocks
    });

    let reconstructed: HashChain = serde_json::from_value(tampered_chain_json).unwrap();
    let err = reconstructed.verify_integrity().unwrap_err();
    assert!(err.to_string().contains("Invalid block sequence") || err.to_string().contains("Invalid previous hash"));
}
