# 🛠️ ForgeTunnel Installation & Uninstallation Field Manual (`FORGE-DOC-INST-01`)

> **Sovereign Local-AI & Webhook Edge Gateway**  
> *Zero-Touch Deployment, Standalone Static Musl Binary Installation, Service Configuration, and Clean Teardown.*

---

## 📑 Table of Contents
1. [System Requirements](#1-system-requirements)
2. [Building from Source (Standard & Static Musl)](#2-building-from-source-standard--static-musl)
3. [Installing the Standalone Static Binary](#3-installing-the-standalone-static-binary)
4. [Systemd Service Setup](#4-systemd-service-setup)
5. [Configuration Deployment](#5-configuration-deployment)
6. [Complete Uninstallation & Artifact Purge](#6-complete-uninstallation--artifact-purge)

---

## 1. System Requirements

- **Supported OS**: Linux (x86_64, aarch64), macOS (Apple Silicon / Intel).
- **Toolchain (Source Build)**: Rust 1.75+ (Cargo).
- **Runtime Dependencies**:
  - **Static Musl Binary**: Zero external runtime dependencies (100% self-contained, bundled SQLite & Rustls TLS).
  - **Dynamic Gnu Build**: Glibc 2.31+.

---

## 2. Building from Source (Standard & Static Musl)

### Standard Release Build (Gnu)
```bash
cd /path/to/forgetunnel
cargo build --release
# Binary produced at: target/release/forgetunnel
```

### Standalone 100% Static Musl Build
```bash
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
# Binary produced at: target/x86_64-unknown-linux-musl/release/forgetunnel
```

---

## 3. Installing the Standalone Static Binary

To install `forgetunnel` globally to `/usr/local/bin`:

```bash
sudo cp target/x86_64-unknown-linux-musl/release/forgetunnel /usr/local/bin/forgetunnel
sudo chmod +x /usr/local/bin/forgetunnel

# Verify installation
forgetunnel --version
```

---

## 4. Systemd Service Setup

Create the dedicated systemd service file at `/etc/systemd/system/forgetunnel.service`:

```ini
[Unit]
Description=ForgeTunnel Sovereign Local-AI & Webhook Edge Gateway
After=network.target ollama.service
Wants=network.target

[Service]
Type=simple
User=root
WorkingDirectory=/var/lib/forgetunnel
ExecStart=/usr/local/bin/forgetunnel expose --config /etc/forgetunnel/forgetunnel.toml
Restart=always
RestartSec=5s
LimitNOFILE=65535
AmbientCapabilities=CAP_NET_BIND_SERVICE

# Sandboxing & Security
ProtectSystem=strict
ProtectHome=read-only
ReadWritePaths=/var/lib/forgetunnel /var/log/forgetunnel
PrivateTmp=true
NoNewPrivileges=true

[Install]
WantedBy=multi-user.target
```

Enable and start the service:
```bash
sudo mkdir -p /var/lib/forgetunnel /var/log/forgetunnel /etc/forgetunnel
sudo systemctl daemon-reload
sudo systemctl enable --now forgetunnel.service
```

---

## 5. Configuration Deployment

Generate the standard configuration template:
```bash
forgetunnel --generate-config | sudo tee /etc/forgetunnel/forgetunnel.toml > /dev/null
```

Edit `/etc/forgetunnel/forgetunnel.toml` with your node label, relay URL, and security tokens.

---

## 6. Complete Uninstallation & Artifact Purge

To completely remove ForgeTunnel and clean up all state:

```bash
# 1. Stop and disable the systemd service
sudo systemctl stop forgetunnel.service 2>/dev/null || true
sudo systemctl disable forgetunnel.service 2>/dev/null || true
sudo rm -f /etc/systemd/system/forgetunnel.service
sudo systemctl daemon-reload

# 2. Remove binaries and symlinks
sudo rm -f /usr/local/bin/forgetunnel

# 3. Purge configuration and ledger databases
sudo rm -rf /etc/forgetunnel
sudo rm -rf /var/lib/forgetunnel
sudo rm -rf /var/log/forgetunnel

echo "ForgeTunnel has been completely uninstalled and purged."
```
