use forge_core::{ForgePacket, PacketFlag, TokenMeter};
use std::thread::sleep;
use std::time::Duration;

#[test]
fn test_rw_packet_hmac_sha256_verification() {
    let secret = "master-secret-key-12345678";
    let packet = ForgePacket::new(
        "endpoint-ollama-01",
        "stream-99",
        1,
        PacketFlag::Data,
        b"{\"model\":\"llama3\",\"response\":\"hello sovereign world\"}".to_vec(),
        secret,
    );

    assert!(packet.verify_signature(secret).is_ok());
    assert!(!packet.sha256_payload_hash().is_empty());
}

#[test]
fn test_attack_packet_payload_tamper_detected() {
    let secret = "master-secret-key-12345678";
    let mut packet = ForgePacket::new(
        "endpoint-ollama-01",
        "stream-99",
        1,
        PacketFlag::Data,
        b"{\"model\":\"llama3\",\"response\":\"legit output\"}".to_vec(),
        secret,
    );

    // Tamper with payload
    packet.payload = b"{\"model\":\"llama3\",\"response\":\"INJECTED EXPLOIT\"}".to_vec();

    assert!(
        packet.verify_signature(secret).is_err(),
        "Tampered payload must cause signature verification failure"
    );
}

#[test]
fn test_attack_packet_sequence_replay_tamper() {
    let secret = "master-secret-key-12345678";
    let mut packet = ForgePacket::new(
        "endpoint-ollama-01",
        "stream-99",
        1,
        PacketFlag::Data,
        b"payload data".to_vec(),
        secret,
    );

    // Tamper with sequence number (replay attack attempt)
    packet.sequence = 999;

    assert!(
        packet.verify_signature(secret).is_err(),
        "Tampered sequence number must cause signature verification failure"
    );
}

#[test]
fn test_rw_token_meter_ttft_and_tps_calculation() {
    let mut meter = TokenMeter::new();
    meter.set_prompt_tokens(45);

    // Simulate TTFT delay
    sleep(Duration::from_millis(20));
    meter.record_chunk(10, 50);

    // Simulate subsequent chunks
    sleep(Duration::from_millis(20));
    meter.record_chunk(15, 75);

    let snapshot = meter.snapshot();
    assert_eq!(snapshot.prompt_tokens, 45);
    assert_eq!(snapshot.completion_tokens, 25);
    assert_eq!(snapshot.total_tokens, 70);
    assert_eq!(snapshot.total_bytes, 125);
    assert!(snapshot.ttft_ms >= 15.0);
    assert!(snapshot.tokens_per_second > 0.0);
}

#[test]
fn test_ec_token_meter_zero_tokens_and_instant_finish() {
    let meter = TokenMeter::new();
    let snapshot = meter.snapshot();
    assert_eq!(snapshot.prompt_tokens, 0);
    assert_eq!(snapshot.completion_tokens, 0);
    assert_eq!(snapshot.total_tokens, 0);
    assert_eq!(snapshot.total_bytes, 0);
}
