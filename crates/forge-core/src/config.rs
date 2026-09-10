use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use crate::error::{ForgeError, Result};
use crate::models::TunnelType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalServiceConfig {
    pub name: String,
    pub local_host: String,
    pub local_port: u16,
    pub tunnel_type: TunnelType,
    #[serde(default = "default_max_rps")]
    pub max_rps: u32,
}

fn default_max_rps() -> u32 {
    100
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeConfig {
    #[serde(default = "default_node_label")]
    pub node_label: String,

    #[serde(default = "default_relay_url")]
    pub relay_url: String,

    #[serde(default = "default_auth_token")]
    pub auth_token: String,

    #[serde(default = "default_danger_code")]
    pub danger_code: String,

    #[serde(default = "default_webhook_buffer_size")]
    pub webhook_buffer_size: usize,

    #[serde(default = "default_services")]
    pub services: Vec<LocalServiceConfig>,
}

fn default_node_label() -> String {
    "hyperion-prime".to_string()
}

fn default_relay_url() -> String {
    "wss://relay.example.com:8084/ws/tunnel".to_string()
}

fn default_auth_token() -> String {
    "forge-dev-token-99x".to_string()
}

fn default_danger_code() -> String {
    "SAFETY-DANGER-CONFIRM-99".to_string()
}

fn default_webhook_buffer_size() -> usize {
    50
}

fn default_services() -> Vec<LocalServiceConfig> {
    vec![
        LocalServiceConfig {
            name: "ollama".to_string(),
            local_host: "127.0.0.1".to_string(),
            local_port: 11434,
            tunnel_type: TunnelType::Ollama,
            max_rps: 120,
        },
        LocalServiceConfig {
            name: "webhooks".to_string(),
            local_host: "127.0.0.1".to_string(),
            local_port: 3000,
            tunnel_type: TunnelType::Webhook,
            max_rps: 300,
        },
    ]
}

impl Default for ForgeConfig {
    fn default() -> Self {
        Self {
            node_label: default_node_label(),
            relay_url: default_relay_url(),
            auth_token: default_auth_token(),
            danger_code: default_danger_code(),
            webhook_buffer_size: default_webhook_buffer_size(),
            services: default_services(),
        }
    }
}

impl ForgeConfig {
    pub fn validate(&self) -> Result<()> {
        if self.node_label.trim().is_empty() {
            return Err(ForgeError::Validation("node_label cannot be empty".into()));
        }

        if !self.relay_url.starts_with("ws://")
            && !self.relay_url.starts_with("wss://")
            && !self.relay_url.starts_with("http://")
            && !self.relay_url.starts_with("https://")
        {
            return Err(ForgeError::Validation(format!(
                "relay_url must use ws://, wss://, http://, or https:// protocol (got: {})",
                self.relay_url
            )));
        }

        if self.auth_token.len() < 8 {
            return Err(ForgeError::Validation(
                "auth_token must be at least 8 characters in length".into(),
            ));
        }

        if self.webhook_buffer_size == 0 || self.webhook_buffer_size > 10000 {
            return Err(ForgeError::Validation(
                "webhook_buffer_size must be between 1 and 10000".into(),
            ));
        }

        for service in &self.services {
            if service.name.trim().is_empty() {
                return Err(ForgeError::Validation("Service name cannot be empty".into()));
            }
            if service.local_port == 0 {
                return Err(ForgeError::Validation(format!(
                    "Service '{}' local_port cannot be 0",
                    service.name
                )));
            }
            if service.name.contains("..") || service.local_host.contains("..") {
                return Err(ForgeError::Validation(
                    "Path traversal characters '..' forbidden in service definitions".into(),
                ));
            }
        }

        Ok(())
    }

    pub fn load_from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)?;
        config.validate()?;
        Ok(config)
    }

    pub fn discover_or_default(explicit_path: Option<&Path>) -> Result<(Self, PathBuf)> {
        if let Some(path) = explicit_path {
            if path.exists() {
                let config = Self::load_from_file(path)?;
                return Ok((config, path.to_path_buf()));
            } else {
                return Err(ForgeError::Config(format!(
                    "Explicit config path does not exist: {}",
                    path.display()
                )));
            }
        }

        let candidates = vec![
            PathBuf::from("forgetunnel.toml"),
            dirs_next_config_path(),
            PathBuf::from("/etc/forgetunnel/forgetunnel.toml"),
        ];

        for candidate in candidates {
            if candidate.exists() {
                if let Ok(config) = Self::load_from_file(&candidate) {
                    return Ok((config, candidate));
                }
            }
        }

        let default_config = Self::default();
        Ok((default_config, PathBuf::from("in-memory-default")))
    }

    pub fn generate_default_template() -> String {
        r#"# ====================================================================
# ⚡ FORGETUNNEL SOVEREIGN LOCAL-AI & WEBHOOK GATEWAY CONFIGURATION
# Open-Source Ngrok + Ollama Bridge
# ====================================================================

node_label = "hyperion-prime"
relay_url = "wss://relay.example.com:8084/ws/tunnel"
auth_token = "forge-dev-token-99x"
danger_code = "SAFETY-DANGER-CONFIRM-99"
webhook_buffer_size = 50

# --------------------------------------------------------------------
# Local Services Matrix (Reverse Tunnel Exposures)
# --------------------------------------------------------------------
[[services]]
name = "ollama"
local_host = "127.0.0.1"
local_port = 11434
tunnel_type = "Ollama"
max_rps = 120

[[services]]
name = "webhooks"
local_host = "127.0.0.1"
local_port = 3000
tunnel_type = "Webhook"
max_rps = 300
"#
        .to_string()
    }
}

fn dirs_next_config_path() -> PathBuf {
    if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home).join(".config/forgetunnel/forgetunnel.toml")
    } else {
        PathBuf::from("forgetunnel.toml")
    }
}
