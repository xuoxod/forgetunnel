use forge_telemetry::{HighPrecisionTimer, TelemetryMinutiae, TelemetryProvenance};
use std::collections::HashMap;

#[test]
fn test_rw_provenance_5w1h_serialization_roundtrip() {
    let mut extra = HashMap::new();
    extra.insert("model_vram_mb".to_string(), "4096".to_string());
    extra.insert("gpu_device".to_string(), "RTX_4090".to_string());

    let minutiae = TelemetryMinutiae {
        duration_ps: 45_200_000_000,
        prompt_tokens: 128,
        completion_tokens: 512,
        ttft_ms: 34.82,
        tps: 64.5,
        payload_bytes: 4096,
        safety_tier: "Tier2".to_string(),
        dry_run: false,
        status_code: 200,
        extra,
    };

    let provenance = TelemetryProvenance::new(
        "tenant:dev-admin",
        "tcp://192.168.1.160:54321",
        "http://127.0.0.1:11434/api/generate",
        "ollama:generate:llama3",
        "TLS_WS_CONDUIT_V1",
        minutiae.clone(),
    );

    let json_str = serde_json::to_string(&provenance).expect("Serialization failed");
    let recovered: TelemetryProvenance = serde_json::from_str(&json_str).expect("Deserialization failed");

    assert_eq!(recovered.who, "tenant:dev-admin");
    assert_eq!(recovered.minutiae.prompt_tokens, 128);
    assert_eq!(recovered.minutiae.completion_tokens, 512);
    assert_eq!(recovered.minutiae.ttft_ms, 34.82);
    assert_eq!(recovered.minutiae.extra.get("gpu_device").unwrap(), "RTX_4090");
}

#[test]
fn test_ec_empty_minutiae_and_extra_fields() {
    let minutiae = TelemetryMinutiae::default();
    let provenance = TelemetryProvenance::new(
        "anonymous",
        "127.0.0.1:0",
        "http://127.0.0.1:3000/webhook",
        "webhook:ping",
        "HTTP_REST",
        minutiae,
    );

    let digest1 = provenance.compute_payload_digest();
    let digest2 = provenance.compute_payload_digest();
    assert_eq!(digest1, digest2);
    assert_eq!(digest1.len(), 64);
}

#[test]
fn test_rw_high_precision_timer_sub_nanos() {
    let timer = HighPrecisionTimer::start();
    std::thread::sleep(std::time::Duration::from_millis(10));
    let elapsed_ps = timer.elapsed_picoseconds();
    let elapsed_ms = timer.elapsed_millis();

    assert!(elapsed_ps >= 10_000_000_000, "Picoseconds must be >= 10,000,000,000 ps");
    assert!(elapsed_ms >= 9.0, "Millis must be >= 9.0 ms");
}
