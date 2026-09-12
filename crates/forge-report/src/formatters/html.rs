use crate::error::ReportError;
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
    <style>
        :root {{
            --bg-primary: #0d1117;
            --bg-secondary: #161b22;
            --bg-tertiary: #21262d;
            --border-color: #30363d;
            --text-primary: #f0f6fc;
            --text-secondary: #8b949e;
            --text-muted: #6e7681;
            --accent-blue: #58a6ff;
            --accent-green: #3fb950;
            --accent-yellow: #d29922;
            --accent-red: #f85149;
            --accent-purple: #bc8cff;
            --font-sans: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
            --font-mono: ui-monospace, SFMono-Regular, SF Mono, Menlo, Consolas, Liberation Mono, monospace;
        }}
        * {{ box-sizing: border-box; margin: 0; padding: 0; }}
        body {{
            background-color: var(--bg-primary);
            color: var(--text-primary);
            font-family: var(--font-sans);
            line-height: 1.5;
            padding: 1rem;
            -webkit-font-smoothing: antialiased;
        }}
        .container {{
            max-width: 1280px;
            margin: 0 auto;
        }}
        header {{
            display: flex;
            flex-wrap: wrap;
            justify-content: space-between;
            align-items: center;
            gap: 1rem;
            padding-bottom: 1.5rem;
            border-bottom: 1px solid var(--border-color);
            margin-bottom: 1.5rem;
        }}
        .brand-title {{
            display: flex;
            align-items: center;
            gap: 0.75rem;
            font-size: 1.5rem;
            font-weight: 700;
            color: var(--text-primary);
        }}
        .brand-badge {{
            background: var(--bg-tertiary);
            border: 1px solid var(--border-color);
            color: var(--accent-blue);
            font-size: 0.75rem;
            font-family: var(--font-mono);
            padding: 0.2rem 0.6rem;
            border-radius: 9999px;
        }}
        .grid-stats {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
            gap: 1rem;
            margin-bottom: 1.5rem;
        }}
        .stat-card {{
            background: var(--bg-secondary);
            border: 1px solid var(--border-color);
            border-radius: 8px;
            padding: 1.25rem;
            display: flex;
            flex-direction: column;
            gap: 0.25rem;
        }}
        .stat-label {{
            font-size: 0.8rem;
            color: var(--text-secondary);
            text-transform: uppercase;
            letter-spacing: 0.05em;
        }}
        .stat-value {{
            font-size: 1.75rem;
            font-weight: 700;
            font-family: var(--font-mono);
            color: var(--text-primary);
        }}
        .narrative-card {{
            background: var(--bg-secondary);
            border: 1px solid var(--border-color);
            border-left: 4px solid var(--accent-blue);
            border-radius: 8px;
            padding: 1.25rem;
            margin-bottom: 1.5rem;
            white-space: pre-line;
            color: var(--text-primary);
            font-size: 0.95rem;
        }}
        .toolbar {{
            display: flex;
            flex-wrap: wrap;
            justify-content: space-between;
            align-items: center;
            gap: 1rem;
            margin-bottom: 1rem;
        }}
        .filter-tabs {{
            display: flex;
            gap: 0.5rem;
            overflow-x: auto;
        }}
        .tab-btn {{
            background: var(--bg-secondary);
            border: 1px solid var(--border-color);
            color: var(--text-secondary);
            padding: 0.5rem 1rem;
            border-radius: 6px;
            cursor: pointer;
            font-size: 0.85rem;
            font-weight: 600;
            transition: all 0.2s;
        }}
        .tab-btn.active {{
            background: var(--accent-blue);
            color: #000;
            border-color: var(--accent-blue);
        }}
        .search-box {{
            background: var(--bg-secondary);
            border: 1px solid var(--border-color);
            color: var(--text-primary);
            padding: 0.5rem 1rem;
            border-radius: 6px;
            font-size: 0.85rem;
            width: 100%;
            max-width: 320px;
        }}
        .table-container {{
            background: var(--bg-secondary);
            border: 1px solid var(--border-color);
            border-radius: 8px;
            overflow-x: auto;
            margin-bottom: 2rem;
            -webkit-overflow-scrolling: touch;
        }}
        table {{
            width: 100%;
            border-collapse: collapse;
            font-size: 0.875rem;
            text-align: left;
        }}
        th {{
            background: var(--bg-tertiary);
            color: var(--text-secondary);
            font-weight: 600;
            padding: 0.75rem 1rem;
            border-bottom: 1px solid var(--border-color);
        }}
        td {{
            padding: 0.75rem 1rem;
            border-bottom: 1px solid var(--border-color);
            color: var(--text-primary);
        }}
        tr:last-child td {{ border-bottom: none; }}
        .badge {{
            display: inline-block;
            padding: 0.2rem 0.5rem;
            border-radius: 4px;
            font-size: 0.75rem;
            font-weight: 600;
            font-family: var(--font-mono);
        }}
        .badge-success {{ background: rgba(63, 185, 80, 0.15); color: var(--accent-green); border: 1px solid var(--accent-green); }}
        .badge-danger {{ background: rgba(248, 81, 73, 0.15); color: var(--accent-red); border: 1px solid var(--accent-red); }}
        .badge-model {{ background: rgba(88, 166, 255, 0.15); color: var(--accent-blue); border: 1px solid var(--accent-blue); }}
        .badge-provider {{ background: rgba(188, 140, 255, 0.15); color: var(--accent-purple); border: 1px solid var(--accent-purple); }}
        .badge-tier {{ background: rgba(210, 153, 34, 0.15); color: var(--accent-yellow); border: 1px solid var(--accent-yellow); }}
        .btn {{
            background: var(--bg-tertiary);
            border: 1px solid var(--border-color);
            color: var(--text-primary);
            padding: 0.4rem 0.8rem;
            border-radius: 4px;
            cursor: pointer;
            font-size: 0.75rem;
            font-weight: 600;
        }}
        .btn:hover {{ background: var(--border-color); }}
        .font-mono {{ font-family: var(--font-mono); }}
        .text-cyan {{ color: var(--accent-blue); }}
        .text-green {{ color: var(--accent-green); }}
        .text-yellow {{ color: var(--accent-yellow); }}
        .text-muted {{ color: var(--text-muted); }}
        .text-right {{ text-align: right; }}
        .text-center {{ text-align: center; }}
        .hidden {{ display: none; }}
        footer {{
            text-align: center;
            padding: 2rem 0;
            color: var(--text-muted);
            font-size: 0.8rem;
            border-top: 1px solid var(--border-color);
        }}
        @media (max-width: 768px) {{
            body {{ padding: 0.5rem; }}
            .brand-title {{ font-size: 1.25rem; }}
            .stat-value {{ font-size: 1.4rem; }}
            .search-box {{ max-width: 100%; }}
        }}
    </style>
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

    <script>
        document.addEventListener('DOMContentLoaded', function() {{
            const searchInput = document.getElementById('filter-search');
            const tabButtons = document.querySelectorAll('.tab-btn');
            const aiSection = document.getElementById('ai-section');
            const webhookSection = document.getElementById('webhook-section');
            const ledgerSection = document.getElementById('ledger-section');

            // Tab filtering
            tabButtons.forEach(btn => {{
                btn.addEventListener('click', function() {{
                    tabButtons.forEach(b => b.classList.remove('active'));
                    this.classList.add('active');
                    const filter = this.getAttribute('data-filter');

                    if (filter === 'all') {{
                        aiSection.classList.remove('hidden');
                        webhookSection.classList.remove('hidden');
                        ledgerSection.classList.remove('hidden');
                    }} else if (filter === 'ai') {{
                        aiSection.classList.remove('hidden');
                        webhookSection.classList.add('hidden');
                        ledgerSection.classList.add('hidden');
                    }} else if (filter === 'webhook') {{
                        aiSection.classList.add('hidden');
                        webhookSection.classList.remove('hidden');
                        ledgerSection.classList.add('hidden');
                    }} else if (filter === 'ledger') {{
                        aiSection.classList.add('hidden');
                        webhookSection.classList.add('hidden');
                        ledgerSection.classList.remove('hidden');
                    }}
                }});
            }});

            // Real-time search filtering
            searchInput.addEventListener('input', function() {{
                const query = this.value.toLowerCase();
                const rows = document.querySelectorAll('.data-row');
                rows.forEach(row => {{
                    const text = row.textContent.toLowerCase();
                    if (text.includes(query)) {{
                        row.style.display = '';
                    }} else {{
                        row.style.display = 'none';
                    }}
                }});
            }});

            // Replay action buttons
            document.querySelectorAll('.action-replay').forEach(btn => {{
                btn.addEventListener('click', function() {{
                    const id = this.getAttribute('data-id');
                    navigator.clipboard.writeText('forgetunnel webhook --replay ' + id).then(() => {{
                        const original = this.textContent;
                        this.textContent = 'Copied!';
                        setTimeout(() => {{ this.textContent = original; }}, 1500);
                    }});
                }});
            }});

            // Export JSON
            const exportBtn = document.getElementById('btn-export-json');
            if (exportBtn) {{
                exportBtn.addEventListener('click', function() {{
                    window.open(window.location.pathname.replace('.html', '.json'), '_blank');
                }});
            }}
        }});
    </script>
</body>
</html>"#,
            title = title,
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
