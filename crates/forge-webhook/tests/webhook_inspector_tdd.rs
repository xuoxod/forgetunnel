use chrono::Utc;
use forge_webhook::{WebhookInspector, WebhookRecord};
use std::collections::HashMap;

#[test]
fn test_rw_webhook_inspector_record_and_retrieve() {
    let inspector = WebhookInspector::new(10);
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());
    headers.insert("X-Stripe-Signature".to_string(), "sig-12345".to_string());

    let record = WebhookRecord {
        id: "wh-evt-001".to_string(),
        timestamp: Utc::now(),
        method: "POST".to_string(),
        path: "/webhook/stripe".to_string(),
        headers,
        payload: b"{\"event\":\"payment_intent.succeeded\",\"amount\":2000}".to_vec(),
        source_ip: "192.0.2.1".to_string(),
        status: 200,
    };

    inspector.record(record.clone());
    assert_eq!(inspector.count(), 1);

    let recent = inspector.list_recent(5);
    assert_eq!(recent.len(), 1);
    assert_eq!(recent[0].id, "wh-evt-001");
    assert!(recent[0].payload_utf8_lossy().contains("payment_intent.succeeded"));

    let fetched = inspector.get("wh-evt-001").expect("Should find record by ID");
    assert_eq!(fetched.path, "/webhook/stripe");
}

#[test]
fn test_ec_webhook_ring_buffer_capacity_rollover() {
    let inspector = WebhookInspector::new(3);

    for i in 1..=5 {
        inspector.record(WebhookRecord {
            id: format!("wh-evt-00{}", i),
            timestamp: Utc::now(),
            method: "POST".to_string(),
            path: "/hook".to_string(),
            headers: HashMap::new(),
            payload: format!("payload-{}", i).into_bytes(),
            source_ip: "127.0.0.1".to_string(),
            status: 200,
        });
    }

    assert_eq!(inspector.count(), 3);
    let recent = inspector.list_recent(10);
    assert_eq!(recent.len(), 3);
    // Newest first: 5, 4, 3
    assert_eq!(recent[0].id, "wh-evt-005");
    assert_eq!(recent[1].id, "wh-evt-004");
    assert_eq!(recent[2].id, "wh-evt-003");
    // 1 and 2 should have been evicted
    assert!(inspector.get("wh-evt-001").is_none());
    assert!(inspector.get("wh-evt-002").is_none());
}

#[test]
fn test_attack_webhook_hmac_signature_validation() {
    let secret = "whsec_test_secret_key_8899aabb";
    let payload = b"{\"event\":\"order_created\",\"total\":99.99}";

    // Compute legitimate HMAC
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(payload);
    let valid_sig = hex::encode(mac.finalize().into_bytes());

    // Legitimate signature verification
    assert!(WebhookInspector::verify_hmac_signature(secret, payload, &valid_sig));

    // Tampered payload with valid signature -> must reject
    let tampered_payload = b"{\"event\":\"order_created\",\"total\":0.01}";
    assert!(!WebhookInspector::verify_hmac_signature(secret, tampered_payload, &valid_sig));

    // Valid payload with forged signature -> must reject
    assert!(!WebhookInspector::verify_hmac_signature(secret, payload, "deadbeefcafebabe1234"));
}
