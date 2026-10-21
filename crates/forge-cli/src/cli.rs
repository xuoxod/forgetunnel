//! CLI argument definitions and subcommands for Forgetunnel.

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "forgetunnel",
    author = "Rick Walker <rick@rmediatech.com>",
    version = "0.1.0",
    about = "⚡ Sovereign Local-AI & Webhook Edge Gateway (The Open-Source Ngrok + Ollama Bridge)",
    long_about = "Forgetunnel exposes local AI models (Ollama, vLLM) and dev webhooks to the world over zero-open-inbound-port Conduit tunnels with token-gating, streaming LLM proxying, and 4-tier safety gates."
)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE", help = "Path to TOML configuration file")]
    pub config: Option<PathBuf>,

    #[arg(long, help = "Generate default forgetunnel.toml configuration template")]
    pub generate_config: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum OutputFormat {
    Html,
    Ascii,
    Markdown,
    Csv,
    Json,
    Jsonl,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Expose a local service port through the reverse tunnel")]
    Expose {
        #[arg(help = "Local port to expose (e.g. 11434 for Ollama, 3000 for Webhooks)")]
        port: u16,

        #[arg(short, long, default_value = "ollama", help = "Service type: ollama, webhook, http")]
        r#as: String,

        #[arg(short, long, help = "Custom name for this endpoint")]
        name: Option<String>,
    },

    #[command(about = "List local Ollama models, quantization levels, and memory footprint")]
    Models {
        #[arg(long, default_value = "http://127.0.0.1:11434", help = "Ollama host base URL")]
        host: String,
    },

    #[command(about = "Start an interactive streaming chat with a local Ollama model")]
    Chat {
        #[arg(short, long, default_value = "llama3:latest", help = "Model identifier")]
        model: String,

        #[arg(long, default_value = "http://127.0.0.1:11434", help = "Ollama host base URL")]
        host: String,
    },

    #[command(about = "Inspect and replay received webhooks")]
    Webhook {
        #[arg(short, long, help = "Replay specific webhook ID")]
        replay: Option<String>,

        #[arg(short, long, default_value = "10", help = "Number of recent webhooks to list")]
        limit: usize,

        #[arg(long, default_value = "forgetunnel.db", help = "Path to SQLite ledger database")]
        db: PathBuf,
    },

    #[command(about = "Display live ASCII throughput & token streaming telemetry")]
    Monitor,

    #[command(about = "Generate multi-format operational & forensic reports from ledger")]
    Report {
        #[arg(short, long, value_enum, default_value_t = OutputFormat::Ascii, help = "Report format")]
        format: OutputFormat,

        #[arg(short, long, help = "Output file path (default: stdout)")]
        output: Option<PathBuf>,

        #[arg(long, default_value = "forgetunnel.db", help = "Path to SQLite ledger database")]
        db: PathBuf,
    },

    #[command(about = "Audit cryptographic SHA-256 blockchain ledger integrity")]
    Audit {
        #[arg(long, default_value = "forgetunnel.db", help = "Path to SQLite ledger database")]
        db: PathBuf,
    },

    #[command(about = "View verified blockchain audit blocks")]
    Logs {
        #[arg(short, long, default_value = "20", help = "Number of blocks to inspect")]
        limit: usize,

        #[arg(long, default_value = "forgetunnel.db", help = "Path to SQLite ledger database")]
        db: PathBuf,
    },
}
