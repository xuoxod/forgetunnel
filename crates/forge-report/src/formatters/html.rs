use crate::error::ReportError;
use crate::formatters::html_assets::{FORGE_HTML_CSS, FORGE_HTML_JS};
use crate::formatters::ReportFormatter;
use crate::models::ReportDocument;
use crate::sanitizer::escape_html;
use crate::storyteller::NarrativeStoryteller;

pub struct HtmlFormatter;

impl ReportFormatter for HtmlFormatter {
    fn render(doc: &ReportDocument) -> Result<String, ReportError> {
        let title = format!("ForgeTunnel Sovereign Gateway — {}", escape_html(&doc.metadata.host));
        let story = NarrativeStoryteller::generate_summary(doc);
        let story_escaped = escape_html(&story);

        let mut ai_rows = String::new();
        if doc.ai_sessions.is_empty() {
            ai_rows.push_str(r#"<tr class="empty-row"><td colspan="8" class="text-center">No active AI sessions recorded</td></tr>"#);
        } else {
            for sess in &doc.ai_sessions {
                let status_class = if sess.status == "COMPLETED" { "badge-success" } else { "badge-danger" };
                ai_rows.push_str(&format!(
                    r#"<tr class="data-row" data-type="ai">
                        <td class="font-mono">{session_id}</td>
                        <td><span class="badge badge-model">{model}</span></td>
                        <td class="prompt-preview" title="{prompt_full}">{prompt}</td>
                        <td class="text-right font-mono">{prompt_tok} / {comp_tok}</td>
                        <td class="text-right font-mono text-cyan">{ttft:.1} ms</td>
                        <td class="text-right font-mono text-green">{tps:.1}</td>
                        <td class="text-right font-mono">{dur:.0} ms</td>
                        <td><span class="badge {status_class}">{status}</span></td>
                    </tr>"#,
                    session_id = escape_html(&sess.session_id),
                    model = escape_html(&sess.model),
                    prompt_full = escape_html(&sess.prompt_preview),
                    prompt = escape_html(&sess.prompt_preview.chars().take(40).collect::<String>()),
                    prompt_tok = sess.prompt_tokens,
                    comp_tok = sess.completion_tokens,
                    ttft = sess.ttft_ms,
                    tps = sess.tps,
                    dur = sess.duration_ms,
                    status_class = status_class,
                    status = escape_html(&sess.status),
                ));
            }
        }

        let mut webhook_rows = String::new();
        if doc.webhooks.is_empty() {
            webhook_rows.push_str(r#"<tr class="empty-row"><td colspan="7" class="text-center">No webhook payloads intercepted</td></tr>"#);
        } else {
            for wh in &doc.webhooks {
                let sig_badge = if wh.signature_valid { "badge-success" } else { "badge-danger" };
                let sig_label = if wh.signature_valid { "Valid HMAC" } else { "Invalid" };
                webhook_rows.push_str(&format!(
                    r#"<tr class="data-row" data-type="webhook">
                        <td class="font-mono">{event_id}</td>
                        <td><span class="badge badge-provider">{provider}</span></td>
                        <td><span class="font-mono text-yellow">{event_type}</span></td>
                        <td><span class="badge {sig_badge}">{sig_label}</span></td>
                        <td class="text-right font-mono">{bytes} B</td>
                        <td class="text-right font-mono">{replays}</td>
                        <td>
                            <button class="btn btn-sm btn-replay action-replay" data-id="{event_id}">Replay</button>
                        </td>
                    </tr>"#,
                    event_id = escape_html(&wh.event_id),
                    provider = escape_html(&wh.provider),
                    event_type = escape_html(&wh.event_type),
                    sig_badge = sig_badge,
                    sig_label = sig_label,
                    bytes = wh.payload_bytes,
                    replays = wh.replay_count,
                ));
            }
        }

        let mut block_rows = String::new();
        if doc.blocks.is_empty() {
            block_rows.push_str(r#"<tr class="empty-row"><td colspan="6" class="text-center">No ledger blocks stored yet</td></tr>"#);
        } else {
            for b in &doc.blocks {
                block_rows.push_str(&format!(
                    r#"<tr class="data-row" data-type="ledger">
                        <td class="font-mono">#{index}</td>
                        <td class="font-mono text-cyan" title="{block_hash}">{hash_short}</td>
                        <td class="font-mono">{who}</td>
                        <td><span class="badge badge-tier">{tier}</span></td>
                        <td class="font-mono text-muted">{action}</td>
                        <td class="font-mono text-right">{time}</td>
                    </tr>"#,
                    index = b.index,
                    block_hash = escape_html(&b.block_hash),
                    hash_short = escape_html(&b.block_hash.chars().take(12).collect::<String>()),
                    who = escape_html(&b.who),
                    tier = escape_html(&b.safety_tier),
                    action = escape_html(&b.action),
                    time = escape_html(&b.timestamp_utc),
                ));
            }
        }

        let html = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=5.0, user-scalable=yes">
    <meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src 'unsafe-inline'; script-src 'unsafe-inline'; img-src data:;">
    <title>{title}</title>
    <link rel="icon" href="data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='%2358a6ff'%3E%3Cpath d='M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5l-9-4zm0 2.18l7 3.12v4.7c0 4.67-3.13 9.04-7 10.15-3.87-1.11-7-5.48-7-10.15V6.3l7-3.12z'/%3E%3C/svg%3E">
    <style>{css}</style>
</head>
<body>
    <div class="container">
        <header>
            <div class="brand-title">
                <span>🛰️ ForgeTunnel Sovereign Gateway</span>
                <span class="brand-badge">{env}</span>
            </div>
            <div class="header-actions">
                <button id="btn-export-json" class="btn">Export JSON</button>
            </div>
        </header>

        <div class="grid-stats">
            <div class="stat-card">
                <span class="stat-label">AI Invocations</span>
                <span class="stat-value text-cyan">{total_ai}</span>
            </div>
            <div class="stat-card">
                <span class="stat-label">Total Compute Tokens</span>
                <span class="stat-value text-cyan">{total_prompt_tokens} <small>P</small> / {total_comp_tokens} <small>C</small></span>
            </div>
            <div class="stat-card">
                <span class="stat-label">Avg Token Velocity</span>
                <span class="stat-value text-green">{avg_tps:.1} <small>TPS</small></span>
            </div>
            <div class="stat-card">
                <span class="stat-label">Avg Latency (TTFT)</span>
                <span class="stat-value text-yellow">{avg_ttft:.1} <small>ms</small></span>
            </div>
            <div class="stat-card">
                <span class="stat-label">Webhooks / Replays</span>
                <span class="stat-value text-purple">{total_wh} / {total_replays}</span>
            </div>
        </div>

        <div class="narrative-card">
            {narrative}
        </div>

        <div class="toolbar">
            <div class="filter-tabs">
                <button class="tab-btn active" data-filter="all">All Activities</button>
                <button class="tab-btn" data-filter="ai">AI Invocations ({total_ai})</button>
                <button class="tab-btn" data-filter="webhook">Webhooks ({total_wh})</button>
                <button class="tab-btn" data-filter="ledger">Blockchain Ledger ({total_blocks})</button>
            </div>
            <input type="text" id="filter-search" class="search-box" placeholder="Filter events, models, providers...">
        </div>

        <div class="table-container" id="ai-section">
            <table>
                <thead>
                    <tr>
                        <th>Session ID</th>
                        <th>Model</th>
                        <th>Prompt Preview</th>
                        <th class="text-right">Prompt / Completion</th>
                        <th class="text-right">TTFT</th>
                        <th class="text-right">TPS</th>
                        <th class="text-right">Duration</th>
                        <th>Status</th>
                    </tr>
                </thead>
                <tbody id="ai-table-body">
                    {ai_rows}
                </tbody>
            </table>
        </div>

        <div class="table-container" id="webhook-section">
            <table>
                <thead>
                    <tr>
                        <th>Event ID</th>
                        <th>Provider</th>
                        <th>Event Type</th>
                        <th>Signature</th>
                        <th class="text-right">Payload Size</th>
                        <th class="text-right">Replays</th>
                        <th>Action</th>
                    </tr>
                </thead>
                <tbody id="webhook-table-body">
                    {webhook_rows}
                </tbody>
            </table>
        </div>

        <div class="table-container" id="ledger-section">
            <table>
                <thead>
                    <tr>
                        <th>Block</th>
                        <th>Block Hash</th>
                        <th>Principal (Who)</th>
                        <th>Tier</th>
                        <th>Action</th>
                        <th class="text-right">Timestamp (UTC)</th>
                    </tr>
                </thead>
                <tbody id="ledger-table-body">
                    {block_rows}
                </tbody>
            </table>
        </div>

        <footer>
            ForgeTunnel v{version} &bull; Host: {host} &bull; Generated: {gen_time}
        </footer>
    </div>

    <script>{js}</script>
</body>
</html>"#,
            title = title,
            css = FORGE_HTML_CSS,
            js = FORGE_HTML_JS,
            env = escape_html(&doc.metadata.environment),
            total_ai = doc.summary.total_ai_sessions,
            avg_tps = doc.summary.avg_tps,
            avg_ttft = doc.summary.avg_ttft_ms,
            total_prompt_tokens = doc.summary.total_prompt_tokens,
            total_comp_tokens = doc.summary.total_completion_tokens,
            total_wh = doc.summary.total_webhooks,
            total_replays = doc.summary.total_replays,
            total_blocks = doc.summary.total_blocks,
            narrative = story_escaped,
            ai_rows = ai_rows,
            webhook_rows = webhook_rows,
            block_rows = block_rows,
            version = escape_html(&doc.metadata.version),
            host = escape_html(&doc.metadata.host),
            gen_time = escape_html(&doc.metadata.generated_at_utc),
        );

        Ok(html)
    }
}
