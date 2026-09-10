use crate::models::GenerateChunk;
use forge_core::{ForgeError, Result};

pub struct NdjsonParser {
    buffer: String,
}

impl NdjsonParser {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    pub fn push_chunk(&mut self, chunk_str: &str) -> Vec<Result<GenerateChunk>> {
        self.buffer.push_str(chunk_str);
        let mut results = Vec::new();

        while let Some(pos) = self.buffer.find('\n') {
            let line = self.buffer[..pos].trim().to_string();
            self.buffer = self.buffer[pos + 1..].to_string();

            if line.is_empty() {
                continue;
            }

            match serde_json::from_str::<GenerateChunk>(&line) {
                Ok(chunk) => results.push(Ok(chunk)),
                Err(e) => results.push(Err(ForgeError::Json(e))),
            }
        }

        results
    }

    pub fn flush_remaining(&mut self) -> Option<Result<GenerateChunk>> {
        let line = self.buffer.trim().to_string();
        self.buffer.clear();
        if line.is_empty() {
            None
        } else {
            Some(serde_json::from_str::<GenerateChunk>(&line).map_err(ForgeError::Json))
        }
    }
}

impl Default for NdjsonParser {
    fn default() -> Self {
        Self::new()
    }
}
