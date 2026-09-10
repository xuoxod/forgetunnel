use forge_ai::OllamaClient;
use forge_bridge::TunnelSession;
use forge_core::{ForgeConfig, ForgePacket, PacketFlag, SafetyDecision, SafetyEvaluator, SafetyTier};

#[test]
fn test_poc_end_to_end_local_ai_tunnel_pipeline() {
    // 1. Setup configuration
    let config = ForgeConfig::default();
    let session = TunnelSession::new(config.clone());
    let stream_id = "stream-poc-ai-999";

    // 2. Client initiates tunnel stream with SYN
    let syn = ForgePacket::new(
        "remote-mobile-client",
        stream_id,
        0,
        PacketFlag::Syn,
        b"".to_vec(),
        &config.auth_token,
    );
    let syn_resp = session.handle_inbound_packet(&syn).expect("SYN should succeed").expect("Must return ACK");
    assert_eq!(syn_resp.flag, PacketFlag::Ack);

    // 3. Client sends prompt request
    let prompt_payload = b"/api/generate?prompt=Explain+Conduit";
    let data = ForgePacket::new(
        "remote-mobile-client",
        stream_id,
        1,
        PacketFlag::Data,
        prompt_payload.to_vec(),
        &config.auth_token,
    );

    // 4. Safety evaluation
    let tier = SafetyEvaluator::classify_path("/api/generate", "POST");
    assert_eq!(tier, SafetyTier::Tier2SafeInvocation);
    let decision = SafetyEvaluator::evaluate(tier, "generate", true, false, None, &config.danger_code).unwrap();
    assert_eq!(decision, SafetyDecision::Allowed);

    // 5. Ollama stream execution & token metering
    let (chunks, meter) = OllamaClient::mock_stream_response("llama3:8b", "Explain Conduit");
    assert!(!chunks.is_empty());
    assert!(chunks.last().unwrap().done);

    let metrics = meter.snapshot();
    assert_eq!(metrics.prompt_tokens, 2);
    assert!(metrics.completion_tokens > 0);

    // 6. Packet dispatch through session
    let data_resp = session.handle_inbound_packet(&data).expect("DATA should succeed").expect("Must return data");
    assert_eq!(data_resp.flag, PacketFlag::Data);
    assert!(data_resp.verify_signature(&config.auth_token).is_ok());

    // 7. Client closes stream with FIN
    let fin = ForgePacket::new(
        "remote-mobile-client",
        stream_id,
        2,
        PacketFlag::Fin,
        b"".to_vec(),
        &config.auth_token,
    );
    let fin_resp = session.handle_inbound_packet(&fin).expect("FIN should succeed").expect("Must return ACK");
    assert_eq!(fin_resp.flag, PacketFlag::Ack);
}
