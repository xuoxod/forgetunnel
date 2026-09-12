# 🛡️ ForgeTunnel Architecture & Cryptographic Security Manual (`FORGE-DOC-ARCH-01`)

> **Sovereign Local-AI & Webhook Edge Gateway**  
> *Cryptographic Proofs, Blockchain Non-Repudiation, 5W1H Micro-Provenance, 4-Tier Gatekeeper, and Injection Defenses.*

---

## 📑 Table of Contents
1. [Architectural Overview & Crate Isolation](#1-architectural-overview--crate-isolation)
2. [Cryptographic Telemetry & SHA-256 Hash-Chain Blockchain](#2-cryptographic-telemetry--sha-256-hash-chain-blockchain)
3. [5W1H Micro-Provenance Schema](#3-5w1h-micro-provenance-schema)
4. [4-Tier Action Safety Gatekeeper](#4-4-tier-action-safety-gatekeeper)
5. [Multi-Provider HMAC-SHA256 Webhook Verification](#5-multi-provider-hmac-sha256-webhook-verification)
6. [Enterprise Report Generation & Security Sanitization](#6-enterprise-report-generation--security-sanitization)
7. [Zero-CDN Native UI & Content Security Policy (CSP)](#7-zero-cdn-native-ui--content-security-policy-csp)

---

## 1. Architectural Overview & Crate Isolation

ForgeTunnel enforces absolute **One-Job-Philosophy (OJP)** across 8 decoupled crates:

```
                      ┌────────────────────────────────────────┐
                      │          forge-cli (Operator)          │
                      └───────────────────┬────────────────────┘
                                          │
    ┌───────────────────────────┬─────────┴─────────┬───────────────────────────┐
    │                           │                   │                           │
┌───▼──────────────┐   ┌────────▼────────┐ ┌────────▼────────┐   ┌──────────────▼───┐
│     forge-ai     │   │  forge-webhook  │ │  forge-report   │   │   forge-bridge   │
│ (Streaming/TTFT) │   │ (HMAC/Replay)   │ │ (Decoupled SST) │   │ (TLS WS Bridge)  │
└───┬──────────────┘   └────────┬────────┘ └────────┬────────┘   └──────────────┬───┘
    │                           │                   │                           │
    └───────────────────────────┼───────────────────┘                           │
                                │                                               │
                       ┌────────▼────────┐                                      │
                       │  forge-ledger   │                                      │
                       │ (SQLite WAL DB) │                                      │
                       └────────┬────────┘                                      │
                                │                                               │
                       ┌────────▼────────┐                                      │
                       │ forge-telemetry │                                      │
                       │ (Blockchain/5W) │                                      │
                       └────────┬────────┘                                      │
                                │                                               │
                       ┌────────▼────────┐                                      │
                       │   forge-core    │◄─────────────────────────────────────┘
                       │ (Matrix/Frames) │
                       └─────────────────┘
```

---

## 2. Cryptographic Telemetry & SHA-256 Hash-Chain Blockchain

Every execution event generates a cryptographic block linked sequentially:

$$\text{Genesis Block: } H_0 = \text{SHA256}(0 \parallel \text{"GENESIS"} \parallel \text{Payload Digest} \parallel T_0)$$
$$\text{Sequential Block: } H_n = \text{SHA256}(n \parallel H_{n-1} \parallel \text{Payload Digest} \parallel T_n)$$

### Tamper-Proof Audit
`ForgeLedger::verify_stored_chain_integrity` audits the entire database by recomputing all rolling SHA-256 hashes against stored raw JSON blocks, detecting:
- Out-of-band SQL updates to token counts or execution times.
- Sequence reordering or deletion of events.
- Modified payload digests.

---

## 3. 5W1H Micro-Provenance Schema

Every invocation is cataloged with 6 core dimensions:
- **Who**: Cryptographic node ID or user entity (`who`).
- **From**: Source socket / IP address (`from`).
- **To**: Target local service endpoint (`to`).
- **What**: Action / Prompt payload / API route (`what`).
- **How**: Transport protocol (`TLS-WS`, `HTTP/1.1`, `SSE`, `NDJSON`).
- **Minutiae**: Picosecond duration ($10^{-12}\text{ s}$), prompt/completion tokens, TTFT (ms), TPS, and safety tier.

---

## 4. 4-Tier Action Safety Gatekeeper

Every command passes through a 4-tier safety matrix:
- **Tier 1 (Read-Only)**: Zero credentials needed.
- **Tier 2 (Safe Invocations)**: Requires valid `auth_token`.
- **Tier 3 (State Modification)**: Requires valid `auth_token` and triggers dry-run validation by default.
- **Tier 4 (Destructive Actions)**: Requires valid `auth_token` and an explicit match against `danger_code`.

---

## 5. Multi-Provider HMAC-SHA256 Webhook Verification

ForgeTunnel verifies webhooks in constant time to prevent timing side-channel attacks:

$$\text{HMAC} = \text{HMAC-SHA256}(\text{Secret}, \text{Raw Payload})$$

### Constant-Time Comparison
Uses `subtle::ConstantTimeEq` to eliminate execution-time discrepancies between valid and invalid signatures.

---

## 6. Enterprise Report Generation & Security Sanitization

### CSV Formula Injection Defense
All generated CSV fields sanitize dangerous spreadsheet prefixes:
$$[=, +, -, @, \backslash t, \backslash r] \longrightarrow \text{Prepended with single quote } (')$$

---

## 7. Zero-CDN Native UI & Content Security Policy (CSP)

The generated interactive HTML5 dashboard (`forge_report.html`) strictly adheres to zero-trust design:
- **Zero Remote CDN Requests**: 100% self-contained styles, fonts, and inline SVG data URIs.
- **Zero Inline Code**: ZERO `style="..."` attributes and ZERO `onclick="..."` event handlers.
- **Strict CSP Header**:
  ```http
  default-src 'none'; style-src 'unsafe-inline'; script-src 'unsafe-inline'; img-src data:;
  ```
