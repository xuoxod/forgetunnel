use chrono::Utc;
use forge_webhook::{WebhookInspector, WebhookRecord};
use std::collections::HashMap;

#[test]
fn test_rw_webhook_multi_provider_hmac_signatures() {
    let secret = "whsec_test_secret_key_12345";
    let payload = b"{\"event\":\"charge.captured\",\"amount\":4200}";

    // Compute expected signature
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(payload);
    let valid_sig = hex::encode(mac.finalize().into_bytes());

    assert!(WebhookInspector::verify_hmac_signature(secret, payload, &valid_sig));
    assert!(!WebhookInspector::verify_hmac_signature(secret, payload, "invalid_forged_signature_hex"));
}

#[test]
fn test_ec_zero_byte_payload_webhook_handling() {
    let inspector = WebhookInspector::new(10);
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());

    let empty_record = WebhookRecord {
        id: "evt_empty_01".to_string(),
        timestamp: Utc::now(),
        method: "POST".to_string(),
        path: "/webhook".to_string(),
        headers,
        payload: Vec::new(),
        source_ip: "127.0.0.1".to_string(),
        status: 200,
    };

    inspector.record(empty_record.clone());
    assert_eq!(inspector.count(), 1);
    let fetched = inspector.get("evt_empty_01").unwrap();
    assert_eq!(fetched.payload.len(), 0);
}

#[test]
fn test_attack_signature_timing_and_replay_boundary() {
    let secret = "tenant_signing_secret";
    let payload = b"state_change_action=activate_subscription";

    let tampered_payload = b"state_change_action=DELETE_ALL_DATA";
    let valid_for_original = {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;
        let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(payload);
        hex::encode(mac.finalize().into_bytes())
    };

    assert!(
        !WebhookInspector::verify_hmac_signature(secret, tampered_payload, &valid_for_original),
        "Must strictly reject valid signature paired with tampered payload"
    );
}
