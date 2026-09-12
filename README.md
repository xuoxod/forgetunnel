<div align="center">

# ⚡ FORGETUNNEL
### Sovereign Local-AI & Webhook Edge Gateway

*The Open-Source Sovereign Alternative to Ngrok & Cloudflare Tunnels for Local LLMs (Ollama/vLLM) and Webhooks.*

[![CI Test Suite](https://img.shields.io/badge/tests-61%2F61%20passing-brightgreen.svg?style=flat-square)](#)
[![Static Musl Binary](https://img.shields.io/badge/musl-100%25%20standalone%20pie-blue.svg?style=flat-square)](#)
[![Blockchain Audit](https://img.shields.io/badge/ledger-SHA--256%20verified-purple.svg?style=flat-square)](#)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-orange.svg?style=flat-square)](#)

</div>

---

## 🌟 Executive Summary

**ForgeTunnel** is an enterprise-grade reverse-tunnel gateway built in Rust. It enables developers to expose local AI models (such as Ollama, vLLM, and llama.cpp) and internal webhook receivers to remote consumers over secure outbound-only TLS WebSocket connections—without opening firewall ports or relying on proprietary SaaS intermediaries.

Built with a strict **One-Job-Philosophy (OJP)**, cryptographic non-repudiation, and sub-nanosecond execution tracking, ForgeTunnel sets a new standard for sovereign infrastructure.

---

## 🏗️ 8-Crate Decoupled Architecture

| Crate | Responsibility |
| :--- | :--- |
| [`crates/forge-core`](crates/forge-core) | 4-tier action safety gatekeeper, configuration framing, network structures. |
| [`crates/forge-telemetry`](crates/forge-telemetry) | 5W1H micro-provenance schema, picosecond timer (`HighPrecisionTimer`), rolling SHA-256 blockchain hash-chain. |
| [`crates/forge-ledger`](crates/forge-ledger) | SQLite WAL persistent sink, token velocity accounting, webhook replay buffers. |
| [`crates/forge-report`](crates/forge-report) | Decoupled Single Source of Truth (SST) generator producing 6 synchronized formats (HTML5 Dashboard, ASCII, Markdown, CSV, JSON, JSONL). |
| [`crates/forge-ai`](crates/forge-ai) | Real-time SSE/NDJSON streaming chunk parser, Time-To-First-Token (TTFT) and Tokens-Per-Second (TPS) metering. |
| [`crates/forge-webhook`](crates/forge-webhook) | Constant-time multi-provider HMAC-SHA256 signature verifier (GitHub, Stripe, Slack, Shopify) and circular ring buffer. |
| [`crates/forge-bridge`](crates/forge-bridge) | Outbound TLS WebSocket reverse tunnel bridge and virtual stream multiplexer. |
| [`crates/forge-cli`](crates/forge-cli) | Tactical operator CLI binary with colorized status HUDs and ANSI tables. |

---

## 🚀 Quick Start

### 1. Build & Install Standalone Musl Binary
```bash
# Compile 100% static release binary
cargo build --release --target x86_64-unknown-linux-musl

# Install globally
sudo cp target/x86_64-unknown-linux-musl/release/forgetunnel /usr/local/bin/
```

### 2. Generate Configuration Template
```bash
forgetunnel --generate-config > forgetunnel.toml
```

### 3. Expose Local Ollama & Webhook Endpoints
```bash
forgetunnel expose --config forgetunnel.toml
```

### 4. Stream Chat Completion with Live Token Metering
```bash
forgetunnel chat --model llama3.2 --prompt "Explain Rust memory safety"
```

---

## ⏱️ Universal Chrono & Search Utilities

ForgeTunnel includes enterprise-grade utilities in the universals toolchain:

### Picosecond & Timestamp Converter (`sovereign-chrono.sh`)
```bash
# Convert raw picoseconds to scaled human format & breakdown
/home/emhcet/private/projects/universals/scripts/sovereign-chrono.sh ps 1450000000000

# Convert ISO-8601 UTC timestamp to Local, UTC, Relative & Epoch
/home/emhcet/private/projects/universals/scripts/sovereign-chrono.sh ts 2026-09-11T20:16:46Z
```

### Multi-Criteria Log & Ledger Search (`forgetunnel-search.sh`)
```bash
# Filter AI sessions from today
/home/emhcet/private/projects/universals/scripts/forgetunnel-search.sh --date today

# Filter AI sessions taking longer than 1s in the last 2 hours
/home/emhcet/private/projects/universals/scripts/forgetunnel-search.sh --since "2h ago" --min-dur 1s --model llama3.2

# Query webhook replay logs
/home/emhcet/private/projects/universals/scripts/forgetunnel-search.sh --type webhook -k Stripe --format json
```

---

## 📊 Decoupled 6-Format Enterprise Reports

Generated automatically via `forgetunnel report` or the orchestrator:
1. **Interactive HTML5 Dashboard**: Zero-CDN responsive dashboard with real-time client-side search, zero inline CSS/JS, and strict Content Security Policy (CSP).
2. **ASCII Terminal Manual**: Formatted tables for standard POSIX consoles.
3. **GitHub Flavored Markdown**: Formatted documentation for repository wikis.
4. **RFC 4180 CSV**: Sanitized tabular data with CSV formula injection defense.
5. **Pretty JSON**: Canonical Single Source of Truth document.
6. **Streaming JSONL**: Line-delimited event stream for log aggregators.

---

## 📚 Documentation Field Manuals

- 📖 [Operator Guide](docs/OPERATOR_GUIDE.md) (`FORGE-DOC-OP-01`)
- 🛠️ [Installation & Uninstallation Field Manual](docs/INSTALLATION_AND_UNINSTALLATION.md) (`FORGE-DOC-INST-01`)
- 🛡️ [Architecture & Security Field Manual](docs/ARCHITECTURE_AND_SECURITY.md) (`FORGE-DOC-ARCH-01`)

---

## ⚖️ License

Dual-licensed under MIT OR Apache-2.0.
