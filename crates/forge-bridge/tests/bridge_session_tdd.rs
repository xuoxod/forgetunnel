use forge_bridge::TunnelSession;
use forge_core::{ForgeConfig, ForgePacket, PacketFlag};

#[test]
fn test_rw_tunnel_session_packet_lifecycle() {
    let config = ForgeConfig::default();
    let session = TunnelSession::new(config.clone());

    // 1. SYN
    let syn = ForgePacket::new(
        "remote-client-01",
        "stream-test-100",
        0,
        PacketFlag::Syn,
        b"".to_vec(),
        &config.auth_token,
    );
    let syn_resp = session.handle_inbound_packet(&syn).unwrap().unwrap();
    assert_eq!(syn_resp.flag, PacketFlag::Ack);
    assert_eq!(syn_resp.payload, b"STREAM_READY");

    // 2. DATA
    let data = ForgePacket::new(
        "remote-client-01",
        "stream-test-100",
        1,
        PacketFlag::Data,
        b"/api/generate".to_vec(),
        &config.auth_token,
    );
    let data_resp = session.handle_inbound_packet(&data).unwrap().unwrap();
    assert_eq!(data_resp.flag, PacketFlag::Data);
    assert!(String::from_utf8_lossy(&data_resp.payload).contains("PROXIED_OK"));

    // 3. HEARTBEAT
    let hb = ForgePacket::new(
        "remote-client-01",
        "stream-test-100",
        2,
        PacketFlag::Heartbeat,
        b"PING".to_vec(),
        &config.auth_token,
    );
    let hb_resp = session.handle_inbound_packet(&hb).unwrap().unwrap();
    assert_eq!(hb_resp.flag, PacketFlag::Heartbeat);
    assert_eq!(hb_resp.payload, b"PONG");

    // 4. FIN
    let fin = ForgePacket::new(
        "remote-client-01",
        "stream-test-100",
        3,
        PacketFlag::Fin,
        b"".to_vec(),
        &config.auth_token,
    );
    let fin_resp = session.handle_inbound_packet(&fin).unwrap().unwrap();
    assert_eq!(fin_resp.flag, PacketFlag::Ack);
    assert_eq!(fin_resp.payload, b"STREAM_CLOSED");
}

#[test]
fn test_attack_bridge_rejects_tampered_packet() {
    let config = ForgeConfig::default();
    let session = TunnelSession::new(config.clone());

    let attack_packet = ForgePacket::new(
        "attacker-node",
        "stream-666",
        0,
        PacketFlag::Data,
        b"/api/generate".to_vec(),
        "wrong-secret-token",
    );

    let result = session.handle_inbound_packet(&attack_packet);
    assert!(result.is_err(), "Bridge must reject packets with invalid signature");
}

#[test]
fn test_sim_bridge_dry_run_state_change() {
    let config = ForgeConfig::default();
    let session = TunnelSession::new(config.clone());

    let pull_packet = ForgePacket::new(
        "client-01",
        "stream-200",
        1,
        PacketFlag::Data,
        b"/api/pull?model=llama3".to_vec(),
        &config.auth_token,
    );

    let resp = session.handle_inbound_packet(&pull_packet).unwrap().unwrap();
    assert_eq!(resp.flag, PacketFlag::Data);
    assert!(String::from_utf8_lossy(&resp.payload).contains("DRY-RUN"));
}
