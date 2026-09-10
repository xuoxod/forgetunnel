use forge_ai::OllamaClient;
use forge_core::{ForgeConfig, ForgePacket, PacketFlag, Result, SafetyDecision, SafetyEvaluator, TokenMeter};
use forge_webhook::WebhookInspector;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::info;

pub struct TunnelSession {
    pub config: ForgeConfig,
    pub webhook_inspector: WebhookInspector,
    pub ollama_client: OllamaClient,
    pub active_streams: Arc<Mutex<HashMap<String, TokenMeter>>>,
}

impl TunnelSession {
    pub fn new(config: ForgeConfig) -> Self {
        let webhook_inspector = WebhookInspector::new(config.webhook_buffer_size);
        let ollama_port = config
            .services
            .iter()
            .find(|s| s.tunnel_type == forge_core::TunnelType::Ollama)
            .map(|s| s.local_port)
            .unwrap_or(11434);

        let ollama_client = OllamaClient::new(&format!("http://127.0.0.1:{}", ollama_port));

        Self {
            config,
            webhook_inspector,
            ollama_client,
            active_streams: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn handle_inbound_packet(&self, packet: &ForgePacket) -> Result<Option<ForgePacket>> {
        // 1. Verify packet signature
        packet.verify_signature(&self.config.auth_token)?;

        // 2. Process based on packet flag
        match packet.flag {
            PacketFlag::Syn => {
                let mut streams = self.active_streams.lock().unwrap();
                streams.insert(packet.stream_id.clone(), TokenMeter::new());
                info!(stream_id = %packet.stream_id, "Initialized new tunnel stream");
                let resp = ForgePacket::new(
                    &self.config.node_label,
                    &packet.stream_id,
                    0,
                    PacketFlag::Ack,
                    b"STREAM_READY".to_vec(),
                    &self.config.auth_token,
                );
                Ok(Some(resp))
            }
            PacketFlag::Data => {
                let path_str = String::from_utf8_lossy(&packet.payload);
                let tier = SafetyEvaluator::classify_path(&path_str, "POST");
                let decision = SafetyEvaluator::evaluate(
                    tier,
                    &path_str,
                    true,
                    false,
                    None,
                    &self.config.danger_code,
                )?;

                match decision {
                    SafetyDecision::Allowed => {
                        let resp = ForgePacket::new(
                            &self.config.node_label,
                            &packet.stream_id,
                            packet.sequence + 1,
                            PacketFlag::Data,
                            format!("PROXIED_OK: {}", path_str).into_bytes(),
                            &self.config.auth_token,
                        );
                        Ok(Some(resp))
                    }
                    SafetyDecision::DryRunOnly { plan } => {
                        let resp = ForgePacket::new(
                            &self.config.node_label,
                            &packet.stream_id,
                            packet.sequence + 1,
                            PacketFlag::Data,
                            plan.into_bytes(),
                            &self.config.auth_token,
                        );
                        Ok(Some(resp))
                    }
                    SafetyDecision::Blocked { reason } => {
                        let resp = ForgePacket::new(
                            &self.config.node_label,
                            &packet.stream_id,
                            packet.sequence + 1,
                            PacketFlag::Rst,
                            reason.into_bytes(),
                            &self.config.auth_token,
                        );
                        Ok(Some(resp))
                    }
                }
            }
            PacketFlag::Heartbeat => {
                let resp = ForgePacket::new(
                    &self.config.node_label,
                    &packet.stream_id,
                    packet.sequence + 1,
                    PacketFlag::Heartbeat,
                    b"PONG".to_vec(),
                    &self.config.auth_token,
                );
                Ok(Some(resp))
            }
            PacketFlag::Fin => {
                let mut streams = self.active_streams.lock().unwrap();
                streams.remove(&packet.stream_id);
                let resp = ForgePacket::new(
                    &self.config.node_label,
                    &packet.stream_id,
                    packet.sequence + 1,
                    PacketFlag::Ack,
                    b"STREAM_CLOSED".to_vec(),
                    &self.config.auth_token,
                );
                Ok(Some(resp))
            }
            _ => Ok(None),
        }
    }
}
