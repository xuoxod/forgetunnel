use crate::error::LedgerError;
use crate::models::{AiSessionRecord, BlockRecord, LedgerSummaryStats, WebhookRecord};
use forge_telemetry::Block;
use rusqlite::{params, Connection};
use std::path::Path;
use std::sync::Mutex;

/// Thread-safe SQLite WAL Ledger Repository
pub struct ForgeLedger {
    conn: Mutex<Connection>,
}

impl ForgeLedger {
    /// Open or create an SQLite ledger database at the given path with WAL mode
    pub fn open(path: impl AsRef<Path>) -> Result<Self, LedgerError> {
        let conn = Connection::open(path)?;
        Self::init_connection(conn)
    }

    /// Open an in-memory SQLite ledger database (ideal for testing)
    pub fn open_in_memory() -> Result<Self, LedgerError> {
        let conn = Connection::open_in_memory()?;
        Self::init_connection(conn)
    }

    fn init_connection(conn: Connection) -> Result<Self, LedgerError> {
        // Enforce high-performance WAL journaling & foreign keys
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA busy_timeout = 5000;
             PRAGMA foreign_keys = ON;"
        )?;

        // Initialize schema tables
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS telemetry_blocks (
                block_index INTEGER PRIMARY KEY,
                block_hash TEXT NOT NULL UNIQUE,
                prev_hash TEXT NOT NULL,
                payload_digest TEXT NOT NULL,
                who TEXT NOT NULL,
                source_addr TEXT NOT NULL,
                dest_addr TEXT NOT NULL,
                action TEXT NOT NULL,
                transport TEXT NOT NULL,
                safety_tier TEXT NOT NULL,
                prompt_tokens INTEGER NOT NULL,
                completion_tokens INTEGER NOT NULL,
                ttft_ms REAL NOT NULL,
                tps REAL NOT NULL,
                duration_ps INTEGER NOT NULL,
                timestamp_utc TEXT NOT NULL,
                raw_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS ai_sessions (
                session_id TEXT PRIMARY KEY,
                model TEXT NOT NULL,
                prompt_preview TEXT NOT NULL,
                prompt_tokens INTEGER NOT NULL,
                completion_tokens INTEGER NOT NULL,
                ttft_ms REAL NOT NULL,
                tps REAL NOT NULL,
                duration_ms REAL NOT NULL,
                status TEXT NOT NULL,
                created_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS webhook_events (
                event_id TEXT PRIMARY KEY,
                provider TEXT NOT NULL,
                event_type TEXT NOT NULL,
                signature_header TEXT NOT NULL,
                signature_valid INTEGER NOT NULL,
                payload_bytes INTEGER NOT NULL,
                payload_body TEXT NOT NULL,
                replay_count INTEGER NOT NULL DEFAULT 0,
                status_code INTEGER NOT NULL,
                created_at TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_blocks_timestamp ON telemetry_blocks(timestamp_utc);
            CREATE INDEX IF NOT EXISTS idx_ai_created ON ai_sessions(created_at);
            CREATE INDEX IF NOT EXISTS idx_webhook_created ON webhook_events(created_at);"
        )?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /// Record a verified blockchain block to the ledger
    pub fn record_block(&self, block: &Block) -> Result<(), LedgerError> {
        let conn = self.conn.lock().map_err(|e| LedgerError::Lock(e.to_string()))?;
        let raw_json = serde_json::to_string(block)?;

        conn.execute(
            "INSERT INTO telemetry_blocks (
                block_index, block_hash, prev_hash, payload_digest,
                who, source_addr, dest_addr, action, transport, safety_tier,
                prompt_tokens, completion_tokens, ttft_ms, tps, duration_ps,
                timestamp_utc, raw_json
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)",
            params![
                block.index,
                block.block_hash,
                block.prev_hash,
                block.payload_digest,
                block.entry.who,
                block.entry.from,
                block.entry.to,
                block.entry.what,
                block.entry.how,
                block.entry.minutiae.safety_tier,
                block.entry.minutiae.prompt_tokens,
                block.entry.minutiae.completion_tokens,
                block.entry.minutiae.ttft_ms,
                block.entry.minutiae.tps,
                block.entry.minutiae.duration_ps,
                block.timestamp_utc,
                raw_json,
            ],
        )?;

        Ok(())
    }

    /// Record an AI session metrics entry
    pub fn record_ai_session(&self, record: &AiSessionRecord) -> Result<(), LedgerError> {
        let conn = self.conn.lock().map_err(|e| LedgerError::Lock(e.to_string()))?;

        conn.execute(
            "INSERT OR REPLACE INTO ai_sessions (
                session_id, model, prompt_preview, prompt_tokens,
                completion_tokens, ttft_ms, tps, duration_ms, status, created_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                record.session_id,
                record.model,
                record.prompt_preview,
                record.prompt_tokens,
                record.completion_tokens,
                record.ttft_ms,
                record.tps,
                record.duration_ms,
                record.status,
                record.created_at,
            ],
        )?;

        Ok(())
    }

    /// Record a webhook interception event
    pub fn record_webhook(&self, record: &WebhookRecord) -> Result<(), LedgerError> {
        let conn = self.conn.lock().map_err(|e| LedgerError::Lock(e.to_string()))?;

        conn.execute(
            "INSERT OR REPLACE INTO webhook_events (
                event_id, provider, event_type, signature_header,
                signature_valid, payload_bytes, payload_body, replay_count,
                status_code, created_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                record.event_id,
                record.provider,
                record.event_type,
                record.signature_header,
                if record.signature_valid { 1 } else { 0 },
                record.payload_bytes,
                record.payload_body,
                record.replay_count,
                record.status_code,
                record.created_at,
            ],
        )?;

        Ok(())
    }

    /// Increment replay counter for a webhook event and return new count
    pub fn increment_webhook_replay(&self, event_id: &str) -> Result<u32, LedgerError> {
        let conn = self.conn.lock().map_err(|e| LedgerError::Lock(e.to_string()))?;

        conn.execute(
            "UPDATE webhook_events SET replay_count = replay_count + 1 WHERE event_id = ?1",
            params![event_id],
        )?;

        let mut stmt = conn.prepare("SELECT replay_count FROM webhook_events WHERE event_id = ?1")?;
        let count: u32 = stmt.query_row(params![event_id], |row| row.get(0))
            .map_err(|_| LedgerError::NotFound(format!("Webhook event not found: {}", event_id)))?;

        Ok(count)
    }

    /// Query recent AI sessions
    pub fn query_ai_sessions(&self, limit: usize) -> Result<Vec<AiSessionRecord>, LedgerError> {
        let conn = self.conn.lock().map_err(|e| LedgerError::Lock(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT session_id, model, prompt_preview, prompt_tokens,
                    completion_tokens, ttft_ms, tps, duration_ms, status, created_at
             FROM ai_sessions ORDER BY created_at DESC LIMIT ?1",
        )?;

        let rows = stmt.query_map(params![limit as i64], |row| {
            Ok(AiSessionRecord {
                session_id: row.get(0)?,
                model: row.get(1)?,
                prompt_preview: row.get(2)?,
                prompt_tokens: row.get(3)?,
                completion_tokens: row.get(4)?,
                ttft_ms: row.get(5)?,
                tps: row.get(6)?,
                duration_ms: row.get(7)?,
                status: row.get(8)?,
                created_at: row.get(9)?,
            })
        })?;

        let mut results = Vec::new();
        for r in rows {
            results.push(r?);
        }
        Ok(results)
    }

    /// Query recent Webhook events
    pub fn query_webhooks(&self, limit: usize) -> Result<Vec<WebhookRecord>, LedgerError> {
        let conn = self.conn.lock().map_err(|e| LedgerError::Lock(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT event_id, provider, event_type, signature_header,
                    signature_valid, payload_bytes, payload_body, replay_count,
                    status_code, created_at
             FROM webhook_events ORDER BY created_at DESC LIMIT ?1",
        )?;

        let rows = stmt.query_map(params![limit as i64], |row| {
            let valid_int: i32 = row.get(4)?;
            Ok(WebhookRecord {
                event_id: row.get(0)?,
                provider: row.get(1)?,
                event_type: row.get(2)?,
                signature_header: row.get(3)?,
                signature_valid: valid_int == 1,
                payload_bytes: row.get(5)?,
                payload_body: row.get(6)?,
                replay_count: row.get(7)?,
                status_code: row.get(8)?,
                created_at: row.get(9)?,
            })
        })?;

        let mut results = Vec::new();
        for r in rows {
            results.push(r?);
        }
        Ok(results)
    }

    /// Query confirmed blockchain blocks from storage
    pub fn query_blocks(&self, limit: usize) -> Result<Vec<BlockRecord>, LedgerError> {
        let conn = self.conn.lock().map_err(|e| LedgerError::Lock(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT block_index, block_hash, prev_hash, payload_digest,
                    who, source_addr, dest_addr, action, transport, safety_tier,
                    prompt_tokens, completion_tokens, ttft_ms, tps, duration_ps,
                    timestamp_utc, raw_json
             FROM telemetry_blocks ORDER BY block_index ASC LIMIT ?1",
        )?;

        let rows = stmt.query_map(params![limit as i64], |row| {
            Ok(BlockRecord {
                index: row.get(0)?,
                block_hash: row.get(1)?,
                prev_hash: row.get(2)?,
                payload_digest: row.get(3)?,
                who: row.get(4)?,
                source_addr: row.get(5)?,
                dest_addr: row.get(6)?,
                action: row.get(7)?,
                transport: row.get(8)?,
                safety_tier: row.get(9)?,
                prompt_tokens: row.get(10)?,
                completion_tokens: row.get(11)?,
                ttft_ms: row.get(12)?,
                tps: row.get(13)?,
                duration_ps: row.get(14)?,
                timestamp_utc: row.get(15)?,
                raw_json: row.get(16)?,
            })
        })?;

        let mut results = Vec::new();
        for r in rows {
            results.push(r?);
        }
        Ok(results)
    }

    /// Verify stored blockchain blocks directly from database and detect any SQL tampering
    pub fn verify_stored_chain_integrity(&self) -> Result<usize, LedgerError> {
        let blocks = self.query_blocks(usize::MAX)?;
        if blocks.is_empty() {
            return Ok(0);
        }

        let mut prev_hash = String::new();
        for (i, block_rec) in blocks.iter().enumerate() {
            if block_rec.index != i as u64 {
                return Err(LedgerError::TamperDetected(format!(
                    "Invalid index sequence: expected {}, got {}",
                    i, block_rec.index
                )));
            }

            if i > 0 && block_rec.prev_hash != prev_hash {
                return Err(LedgerError::TamperDetected(format!(
                    "Broken hash chain at block {}: prev_hash '{}' != predecessor hash '{}'",
                    i, block_rec.prev_hash, prev_hash
                )));
            }

            // Verify SQL column fields match JSON and verify internal crypto signature
            let block: Block = serde_json::from_str(&block_rec.raw_json)?;
            if block_rec.block_hash != block.block_hash 
                || block_rec.payload_digest != block.payload_digest 
                || block_rec.prev_hash != block.prev_hash 
            {
                return Err(LedgerError::TamperDetected(format!(
                    "SQL column tamper detected at block index {}: column digest '{}' != json digest '{}'",
                    i, block_rec.payload_digest, block.payload_digest
                )));
            }

            block.verify().map_err(|e| LedgerError::TamperDetected(format!("Block {} verification failed: {}", i, e)))?;

            prev_hash = block_rec.block_hash.clone();
        }

        Ok(blocks.len())
    }

    /// Calculate summary statistics aggregate
    pub fn get_summary_stats(&self) -> Result<LedgerSummaryStats, LedgerError> {
        let conn = self.conn.lock().map_err(|e| LedgerError::Lock(e.to_string()))?;

        let (total_ai, total_prompt, total_completion, avg_ttft, avg_tps): (u64, u64, u64, f64, f64) = conn.query_row(
            "SELECT COUNT(*), COALESCE(SUM(prompt_tokens), 0), COALESCE(SUM(completion_tokens), 0),
                    COALESCE(AVG(ttft_ms), 0.0), COALESCE(AVG(tps), 0.0)
             FROM ai_sessions",
            [],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?)),
        )?;

        let (total_webhooks, valid_webhooks, total_replays): (u64, u64, u64) = conn.query_row(
            "SELECT COUNT(*), COALESCE(SUM(signature_valid), 0), COALESCE(SUM(replay_count), 0)
             FROM webhook_events",
            [],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
        )?;

        let total_blocks: u64 = conn.query_row(
            "SELECT COUNT(*) FROM telemetry_blocks",
            [],
            |r| r.get(0),
        )?;

        Ok(LedgerSummaryStats {
            total_ai_sessions: total_ai,
            total_prompt_tokens: total_prompt,
            total_completion_tokens: total_completion,
            avg_ttft_ms: avg_ttft,
            avg_tps: avg_tps,
            total_webhooks,
            valid_webhooks,
            total_replays,
            total_blocks,
        })
    }
}
