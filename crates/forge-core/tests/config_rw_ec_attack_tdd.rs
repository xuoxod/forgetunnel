use forge_core::{ForgeConfig, LocalServiceConfig, TunnelType};
use std::fs;

#[test]
fn test_rw_config_template_roundtrip() {
    let template = ForgeConfig::generate_default_template();
    let config: ForgeConfig = toml::from_str(&template).expect("Default template should parse validly");
    assert_eq!(config.node_label, "hyperion-prime");
    assert_eq!(config.relay_url, "wss://relay.example.com:8084/ws/tunnel");
    assert_eq!(config.services.len(), 2);
    assert_eq!(config.services[0].name, "ollama");
    assert_eq!(config.services[0].local_port, 11434);
    assert_eq!(config.services[0].tunnel_type, TunnelType::Ollama);
    assert!(config.validate().is_ok());
}

#[test]
fn test_rw_config_disk_persistence() {
    let temp_dir = std::env::temp_dir().join(format!("forge_test_conf_{}", std::process::id()));
    fs::create_dir_all(&temp_dir).unwrap();
    let file_path = temp_dir.join("forgetunnel.toml");

    let mut config = ForgeConfig::default();
    config.node_label = "valkyrie-edge-02".to_string();
    config.relay_url = "wss://relay.example.com:8084/ws/tunnel".to_string();
    config.services.push(LocalServiceConfig {
        name: "custom-api".to_string(),
        local_host: "127.0.0.1".to_string(),
        local_port: 8080,
        tunnel_type: TunnelType::Http,
        max_rps: 500,
    });

    let toml_str = toml::to_string_pretty(&config).unwrap();
    fs::write(&file_path, toml_str).unwrap();

    let loaded = ForgeConfig::load_from_file(&file_path).expect("Should load back cleanly");
    assert_eq!(loaded.node_label, "valkyrie-edge-02");
    assert_eq!(loaded.services.len(), 3);
    assert_eq!(loaded.services[2].name, "custom-api");
    assert_eq!(loaded.services[2].local_port, 8080);

    let _ = fs::remove_dir_all(temp_dir);
}

#[test]
fn test_ec_boundary_ports_and_buffer_sizes() {
    let mut config = ForgeConfig::default();
    config.services[0].local_port = 1;
    config.services[1].local_port = 65535;
    config.webhook_buffer_size = 1;
    assert!(config.validate().is_ok());

    config.webhook_buffer_size = 10000;
    assert!(config.validate().is_ok());

    config.webhook_buffer_size = 0;
    assert!(config.validate().is_err());

    config.webhook_buffer_size = 10001;
    assert!(config.validate().is_err());
}

#[test]
fn test_attack_config_malformed_syntax() {
    let malformed_toml = r#"
    node_label = "hyperion-prime"
    relay_url = [unclosed syntax bracket
    "#;
    let res: std::result::Result<ForgeConfig, _> = toml::from_str(malformed_toml);
    assert!(res.is_err(), "Malformed TOML must return an error");
}

#[test]
fn test_attack_config_path_traversal() {
    let mut config = ForgeConfig::default();
    config.services[0].name = "../../etc/shadow".to_string();
    assert!(config.validate().is_err(), "Service name with path traversal must be rejected");

    let mut config2 = ForgeConfig::default();
    config2.services[0].local_host = "../../../dev/null".to_string();
    assert!(config2.validate().is_err(), "Host with path traversal must be rejected");
}

#[test]
fn test_attack_config_invalid_relay_protocol() {
    let mut config = ForgeConfig::default();
    config.relay_url = "ftp://malicious.server:21/exploit".to_string();
    assert!(config.validate().is_err(), "FTP protocol must be rejected");

    config.relay_url = "javascript:alert(1)".to_string();
    assert!(config.validate().is_err(), "Javascript URL must be rejected");
}

#[test]
fn test_attack_config_weak_auth_token() {
    let mut config = ForgeConfig::default();
    config.auth_token = "abc".to_string();
    assert!(config.validate().is_err(), "Token shorter than 8 chars must be rejected");
}
