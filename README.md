# ⚡ Forgetunnel (`forgetunnel-rs`)
### *Sovereign Local-AI & Webhook Edge Gateway (The Open-Source Ngrok + Ollama Bridge)*

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Rust: 1.75+](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)
[![Security: 4-Tier Guard](https://img.shields.io/badge/security-4--Tier%20Guard-brightgreen.svg)]()
[![Zero Port Forwarding](https://img.shields.io/badge/firewall-0%20Open%20Ports-success.svg)]()

---

## 🧭 Overview

**Forgetunnel** is an autonomous reverse-proxy and edge gateway designed for developers running GPU-accelerated local AI models (such as **Ollama**, **vLLM**, or **LocalAI**) and webhook services (such as **Stripe**, **GitHub**, or **Shopify**).

By utilizing **Conduit** sovereign edge tunneling, Forgetunnel eliminates the need for third-party SaaS subscriptions, port forwarding, or complex VPN overlays.

```
┌──────────────────────────────┐                ┌──────────────────────────────┐
│  Remote Client / Phone       │                │  Local AI Workstation        │
│  (Mobile Safari / Chrome)    │                │  (Ollama / GPU Rig)          │
└──────────────┬───────────────┘                └──────────────▲───────────────┘
               │                                               │
               │ HTTPS / WSS                                   │ Streamed NDJSON / SSE
               ▼                                               │
┌──────────────────────────────┐                ┌──────────────┴───────────────┐
│   Master Conduit Gateway     │ ══════════════ │   Forgetunnel Outpost        │
│   (Auth Gate & Multiplexer)  │   TLS Tunnel   │   - 4-Tier Safety Matrix     │
└──────────────────────────────┘ (Zero Ports)   │   - TTFT & Token Meter       │
                                                │   - Webhook Ring-Inspector   │
                                                └──────────────────────────────┘
```

---

## ✨ Key Features

1. **One-Command Local AI Tunneling**: Expose local LLMs (`:11434`) securely with token-gated streaming proxying.
2. **4-Tier Safety Matrix**:
   - **Tier 1 (Read-Only)**: Model tags, health checks, latency queries.
   - **Tier 2 (Safe Invocation)**: Chat completions, text generation with Bearer token authentication.
   - **Tier 3 (State Change)**: Model pulling (`ollama pull`) protected by dry-run verification.
   - **Tier 4 (Danger/Destructive)**: Model deletion (`ollama rm`) and host execution strictly blocked without danger confirmation keys.
3. **TTFT & Token Metering**: Real-time profiler calculating Time-To-First-Token (TTFT), tokens/second throughput, and payload volume.
4. **Webhook Ring-Buffer Inspector & Replay**: Inspect incoming webhooks with one-click payload replay into local dev ports.
5. **Persistent TOML Discovery Hierarchy**: Automatically searches `./forgetunnel.toml`, `~/.config/forgetunnel/forgetunnel.toml`, and `/etc/forgetunnel/forgetunnel.toml`.

---

## 🚀 Quickstart

### 1. Build & Run Tests
```bash
cargo test --workspace
```

### 2. Generate Default Configuration
```bash
forgetunnel --generate-config > forgetunnel.toml
```

### 3. Expose Local Ollama Instance
```bash
forgetunnel expose 11434 --as ollama --name my-llama3
```

### 4. Interactive Terminal Chat
```bash
forgetunnel chat --model llama3:8b
```

### 5. Inspect Webhooks
```bash
forgetunnel webhook --limit 10
```

---

## 🛡️ License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE).
