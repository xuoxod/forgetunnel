# 📖 ForgeTunnel Enterprise Operator Guide (`FORGE-DOC-OP-01`)

> **Sovereign Local-AI & Webhook Edge Gateway**  
> *The Decoupled Open-Source Alternative to Ngrok & Cloudflare Tunnels for Local LLMs (Ollama/vLLM) and Webhooks.*

---

## 📑 Table of Contents
1. [Overview & Architectural Philosophy](#1-overview--architectural-philosophy)
2. [CLI Command Reference](#2-cli-command-reference)
3. [Reverse Proxying Local AI Models](#3-reverse-proxying-local-ai-models)
4. [Webhook Ingestion & Cryptographic Replay](#4-webhook-ingestion--cryptographic-replay)
5. [4-Tier Safety Matrix & Destructive Action Authorization](#5-4-tier-safety-matrix--destructive-action-authorization)
6. [Picosecond Telemetry & Sovereign Chrono Tools](#6-picosecond-telemetry--sovereign-chrono-tools)
7. [Enterprise Search & Date/Time Filtering](#7-enterprise-search--datetime-filtering)
8. [Blockchain Ledger Auditing & Non-Repudiation](#8-blockchain-ledger-auditing--non-repudiation)

---

## 1. Overview & Architectural Philosophy

ForgeTunnel is engineered around the **One-Job-Philosophy (OJP)**. It decouples the outbound reverse-tunnel gateway, AI token metering, webhook inspection, tamper-proof blockchain ledgering, and multi-format report generation into strictly separated crates:

- **`forge-core`**: Core configuration, TLS framing, and 4-tier action safety matrix.
- **`forge-telemetry`**: 5W1H micro-provenance schema, picosecond timer (`HighPrecisionTimer`), and SHA-256 rolling blockchain hash-chain engine.
- **`forge-ledger`**: High-performance SQLite WAL persistent sink for AI invocations, token accounting, and webhook replay buffers.
- **`forge-report`**: Decoupled Single Source of Truth (SST) report orchestrator generating 6 synchronized formats (Interactive HTML5 Dashboard, ASCII, Markdown, RFC 4180 CSV, Pretty JSON, Streaming JSONL).
- **`forge-ai`**: Real-time SSE/NDJSON streaming chunk parser, Time-To-First-Token (TTFT), and Tokens-Per-Second (TPS) metering.
- **`forge-webhook`**: Multi-provider HMAC-SHA256 signature verification (Stripe, GitHub, Slack, Shopify) and circular ring buffer.
- **`forge-bridge`**: Outbound TLS WebSocket reverse tunnel bridge and virtual stream multiplexer.
- **`forge-cli`**: High-performance operator CLI with ANSI terminal formatting.

---

## 2. CLI Command Reference

The ForgeTunnel CLI binary (`forgetunnel`) provides comprehensive management commands:

| Command | Description | Safety Tier |
| :--- | :--- | :--- |
| `forgetunnel expose` | Launch reverse tunnel to expose local Ollama/HTTP services | Tier 2 |
| `forgetunnel models` | Query and list local Ollama models with parameter sizes and VRAM | Tier 1 |
| `forgetunnel chat` | Stream chat completion through tunnel with live TTFT & TPS metering | Tier 2 |
| `forgetunnel webhook` | Inspect captured webhooks and replay specific payloads with HMAC validation | Tier 2 |
| `forgetunnel monitor` | Launch live TUI dashboard tracking active connections, token velocity, and health | Tier 1 |
| `forgetunnel report` | Generate decoupled 6-format enterprise reports from SQLite WAL ledger | Tier 1 |
| `forgetunnel audit` | Verify SHA-256 cryptographic blockchain ledger against SQL injection or tampering | Tier 1 |
| `forgetunnel logs` | Stream live structured telemetry events in JSON or colorized ANSI | Tier 1 |
| `forgetunnel --generate-config` | Emit fully documented TOML configuration template | Tier 1 |

---

## 3. Reverse Proxying Local AI Models

ForgeTunnel allows developers to expose local Ollama or vLLM inference instances to remote endpoints over an encrypted, outbound-only TLS WebSocket bridge.

### Launching the Tunnel
```bash
forgetunnel expose --config forgetunnel.toml --node hyperion-prime
```

### Querying Local Models
```bash
forgetunnel models --host 127.0.0.1 --port 11434
```

### Streaming Chat Invocations
```bash
forgetunnel chat --model llama3.2 --prompt "Explain Rust async runtimes" --temperature 0.7
```
*Live output provides continuous token-by-token emission, measuring Time-To-First-Token (TTFT) and real-time Tokens-Per-Second (TPS).*

---

## 4. Webhook Ingestion & Cryptographic Replay

ForgeTunnel captures inbound webhooks into an in-memory circular ring buffer and persists them to the SQLite WAL ledger.

### Supported HMAC Signatures
- **GitHub**: `X-Hub-Signature-256` (`sha256=<digest>`)
- **Stripe**: `Stripe-Signature` (`t=<timestamp>,v1=<digest>`)
- **Slack**: `X-Slack-Signature` (`v0=<digest>`)
- **Shopify**: `X-Shopify-Hmac-Sha256` (Base64 digest)

### Webhook Inspection & Replay
```bash
# List recent webhook events
forgetunnel webhook --list --limit 20

# Replay an event to local consumer
forgetunnel webhook --replay wh_evt_99812 --target http://127.0.0.1:3000/api/webhook
```

---

## 5. 4-Tier Safety Matrix & Destructive Action Authorization

All operations pass through ForgeTunnel's cryptographic safety gatekeeper:

| Tier | Category | Description | Authentication Required |
| :--- | :--- | :--- | :--- |
| **Tier 1** | Read-Only | Status checks, model listings, telemetry queries, report generation | No |
| **Tier 2** | Safe Invocations | Ollama prompt streaming, webhook replay, tunnel proxying | Bearer Auth Token |
| **Tier 3** | State Modification | Config reloading, log purging, webhook clearing | Bearer Token + Dry-Run Mode |
| **Tier 4** | Destructive Actions | Model deletion, ledger wipe, node deregistration | Bearer Token + Explicit `danger_code` |

---

## 6. Picosecond Telemetry & Sovereign Chrono Tools

ForgeTunnel measures micro-provenance timings using picosecond resolution ($10^{-12}\text{ s}$).

To convert and inspect timestamps and picoseconds in human-friendly formats, use the universal namespaced tool:

```bash
# Convert raw picoseconds to scaled human format & unit breakdown
/home/emhcet/private/projects/universals/scripts/sovereign-chrono.sh ps 1450000000000

# Convert ISO-8601 UTC timestamp to Local, UTC, Relative & Epoch
/home/emhcet/private/projects/universals/scripts/sovereign-chrono.sh ts 2026-09-11T20:16:46Z

# Parse human duration into picoseconds
/home/emhcet/private/projects/universals/scripts/sovereign-chrono.sh parse 1.5s

# Live time inspection across all representations
/home/emhcet/private/projects/universals/scripts/sovereign-chrono.sh now
```

---

## 7. Enterprise Search & Date/Time Filtering

Filter Forgetunnel SQLite ledgers, reports, and logs by exact dates, relative times, daily windows, AI models, durations, or keywords:

```bash
# Search AI sessions from today
/home/emhcet/private/projects/universals/scripts/forgetunnel-search.sh --date today

# Search AI sessions in the last 2 hours taking longer than 500ms
/home/emhcet/private/projects/universals/scripts/forgetunnel-search.sh --since "2h ago" --min-dur 500ms --model llama3.2

# Search webhooks by source keyword in JSON format
/home/emhcet/private/projects/universals/scripts/forgetunnel-search.sh --type webhook -k Stripe --format json

# Search blockchain telemetry blocks with bullet summary
/home/emhcet/private/projects/universals/scripts/forgetunnel-search.sh --type block --safety-tier Tier2 --format summary
```

---

## 8. Blockchain Ledger Auditing & Non-Repudiation

Every telemetry event generates a sequential block cryptographically chained to previous blocks using SHA-256:

$$\text{Block Hash} = \text{SHA256}(\text{Index} \parallel \text{Prev Hash} \parallel \text{Payload Digest} \parallel \text{Timestamp})$$

To verify ledger integrity:
```bash
forgetunnel audit --db /path/to/forgetunnel_telemetry.db
```
If an unauthorized out-of-band SQL update modifies any payload, hash, or sequence index, `forgetunnel audit` detects the break and flags non-repudiation failure.
