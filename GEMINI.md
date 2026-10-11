# 🏛️ SOVEREIGN FLAGSHIP ARCHITECTURAL & ENGINEERING STANDARD

> **Document ID:** `AGY-RULE-SOVEREIGN-FLAGSHIP-01`  
> **Status:** Permanent Global & Workspace Rulebook  
> **Target Ecosystem:** RMediaTech Sovereign Infrastructure, Flagship Web Platform, & Conduit Suite  

---

## 🎯 1. Ground-Floor Sovereign Re-Architecture Principle (No Legacy Copy-Paste)

* **Problem-Intent Extraction Over Code Lifting**:
  * Never blindly copy-paste or lift legacy code or experimental prototypes.
  * Extract the hard-won *domain requirements*, *real-world failure modes*, and *edge cases* discovered through past troubleshooting.
  * Re-engineer each utility from scratch using modern, idiomatic, enterprise-grade, zero-allocation Rust.
* **Radical OJP (One-Job-Principle)**:
  * Every crate, module, struct, and function must have a single, strictly bounded responsibility.
  * Zero monolithic sprawl, zero overlapping concern boundaries, and zero hidden side-effects.

---

## 🧼 2. The Pristine Single-Schema & Anti-Patching Principle (Zero Migration Hell)

* **Zero Incremental DB Migrations**:
  * Chained migration files (`001_initial.sql`, `002_patch.sql`, `003_hotfix.sql`) are strictly forbidden.
  * The entire database topology must derive from a single, authoritative, pristine `schema.sql` executed in one atomic transaction.
* **Tear-Down & Rebuild Over Band-Aid Patching**:
  * Never duct-tape or patch flawed, naive, or failing code.
  * If a design weakness or better enterprise awareness is discovered, tear down the module and rebuild it cleanly the way it should have been from line one.
* **Deterministic Single-Command Rebuilds**:
  * Database state must be 100% rebuildable in sub-second time via atomic, namespaced OJP bash tools (`scripts/db/rmt_db_*.sh`).

---

## ⚡ 3. Pure-Rust Systems Engineering & Runtime Standards

* **Static Musl Single-Binary Distribution**:
  * Target `x86_64-unknown-linux-musl` and `aarch64-unknown-linux-musl` with 100% static linking.
  * Zero dynamic `glibc` runtime dependencies or shared object mismatches.
* **Panic Immunity & Strict Typed Errors**:
  * `unwrap()` and `expect()` are strictly forbidden in production runtime code paths.
  * All errors must be modeled as domain-specific `enum` types using `thiserror`.
  * Public API responses must never leak internal stack traces, database schema details, or raw server paths.
* **Async Hygiene & Resource Safety**:
  * All Tokio channels must be explicitly bounded (`tokio::sync::mpsc::channel(N)`) to guarantee zero memory exhaustion under load.
  * Explicit timeouts on all network I/O (`tokio::time::timeout`).
  * Graceful shutdown orchestration using `tokio_util::sync::CancellationToken`.
  * Zero task leaks, zero unmonitored background spawned tasks, zero blocking operations on async runtime threads.

---

## 💎 4. Flagship UI & Frontend Security Standards

* **Viewport-Native Modern Aesthetics**:
  * 100% responsive and natural across Mobile, Tablet, Desktop, and Ultra-Wide displays.
  * Glassmorphic, dark-mode primary, sovereign minimalist design.
* **Strict Zero-Inline CSP (Content Security Policy)**:
  * **0% Inline CSS** (`style="..."` prohibited) and **0% Inline JavaScript** (`<script>...</script>` prohibited).
  * 100% self-hosted static assets (CSS, JS, SVG icons, fonts).
  * **0% External CDN Dependencies**: Zero calls to Google Fonts, Cloudflare CDN, or third-party trackers. All assets served directly from the sovereign server.
* **Sovereign Cryptographic Identity**:
  * Password hashing via Argon2id with memory-hardened parameters.
  * Token generation and asymmetric licensing via Ed25519 signatures.
  * Native carrier email/SMS verification with zero Twilio markups or proprietary vendor lock-in.

---

## 🛡️ 5. The 6-Tier Sovereign Verification & Adversarial Testing Shield

Every crate, module, and API route must satisfy all 6 testing tiers:

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                    THE 6-TIER SOVEREIGN VERIFICATION MATRIX                     │
├─────────────────────────────────────────────────────────────────────────────────┤
│ Tier 1: POC TDD        │ Algorithmic correctness verified test-first before code│
│ Tier 2: RW (Real-World)│ Production payloads, valid browser headers & devices   │
│ Tier 3: EC (Edge-Case) │ Zero-byte files, integer limits, malformed UTF-8/JSON  │
│ Tier 4: SIM (Chaos)    │ Packet drops, simulated jitter, sudden socket closure  │
│ Tier 5: TDD CLI Tools  │ Dedicated standalone diagnostic & test runner CLI tools│
│ Tier 6: RED-TEAM ATTACK│ Deliberate self-fuzzing & exploitation test harness    │
└─────────────────────────────────────────────────────────────────────────────────┘
```

* **Adversarial Self-Attacking**: Before deployment, we actively pen-test and fuzz our own rate limiters, token parsers, reverse proxies, and API gates with exploit payloads.

---

## 🌐 6. Decoupled Multi-Linode Topology & Zero-Waste Rule

* **Linode 1 (`rmediatech.com`) — Storefront & Control Plane**:
  * Customer registration, sovereign identity, Stripe recurring billing, tenant API key provisioning, and interactive live demo rooms.
* **Linode 2 (`sfu.rmediatech.com`) — Private Node**:
  * Untouched, dedicated to private internal WebRTC and Matrix communications.
* **Linode 3 (`livekit.rmediatech.com`) — Commercial SFU & Ledger**:
  * Pure high-speed LiveKit media fanout and SovereignLedger append log. 100% autonomous video forwarding with zero web application overhead.
* **The Working-Class Zero-Waste Rule**:
  * Never purchase or deploy servers ahead of revenue. Maximize existing hardware capacity and only scale nodes when paying customer revenue funds it.
