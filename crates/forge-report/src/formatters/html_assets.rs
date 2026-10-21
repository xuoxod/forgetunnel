//! CSS and JavaScript assets for ForgeTunnel HTML5 Executive Dashboard.

pub const FORGE_HTML_CSS: &str = r#"
        :root {
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
        }
        * { box-sizing: border-box; margin: 0; padding: 0; }
        body {
            background-color: var(--bg-primary);
            color: var(--text-primary);
            font-family: var(--font-sans);
            line-height: 1.5;
            padding: 1rem;
            -webkit-font-smoothing: antialiased;
        }
        .container { max-width: 1280px; margin: 0 auto; }
        header {
            display: flex; flex-wrap: wrap; justify-content: space-between; align-items: center;
            gap: 1rem; padding-bottom: 1.5rem; border-bottom: 1px solid var(--border-color); margin-bottom: 1.5rem;
        }
        .brand-title { display: flex; align-items: center; gap: 0.75rem; font-size: 1.5rem; font-weight: 700; color: var(--text-primary); }
        .brand-badge {
            background: var(--bg-tertiary); border: 1px solid var(--border-color); color: var(--accent-blue);
            font-size: 0.75rem; font-family: var(--font-mono); padding: 0.2rem 0.6rem; border-radius: 9999px;
        }
        .grid-stats { display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: 1rem; margin-bottom: 1.5rem; }
        .stat-card {
            background: var(--bg-secondary); border: 1px solid var(--border-color); border-radius: 8px;
            padding: 1.25rem; display: flex; flex-direction: column; gap: 0.25rem;
        }
        .stat-label { font-size: 0.8rem; color: var(--text-secondary); text-transform: uppercase; letter-spacing: 0.05em; }
        .stat-value { font-size: 1.75rem; font-weight: 700; font-family: var(--font-mono); color: var(--text-primary); }
        .narrative-card {
            background: var(--bg-secondary); border: 1px solid var(--border-color); border-left: 4px solid var(--accent-blue);
            border-radius: 8px; padding: 1.25rem; margin-bottom: 1.5rem; white-space: pre-line; color: var(--text-primary); font-size: 0.95rem;
        }
        .toolbar { display: flex; flex-wrap: wrap; justify-content: space-between; align-items: center; gap: 1rem; margin-bottom: 1rem; }
        .filter-tabs { display: flex; gap: 0.5rem; overflow-x: auto; }
        .tab-btn {
            background: var(--bg-secondary); border: 1px solid var(--border-color); color: var(--text-secondary);
            padding: 0.5rem 1rem; border-radius: 6px; cursor: pointer; font-size: 0.85rem; font-weight: 600; transition: all 0.2s;
        }
        .tab-btn.active { background: var(--accent-blue); color: #000; border-color: var(--accent-blue); }
        .search-box {
            background: var(--bg-secondary); border: 1px solid var(--border-color); color: var(--text-primary);
            padding: 0.5rem 1rem; border-radius: 6px; font-size: 0.85rem; width: 100%; max-width: 320px;
        }
        .table-container {
            background: var(--bg-secondary); border: 1px solid var(--border-color); border-radius: 8px;
            overflow-x: auto; margin-bottom: 2rem; -webkit-overflow-scrolling: touch;
        }
        table { width: 100%; border-collapse: collapse; font-size: 0.875rem; text-align: left; }
        th { background: var(--bg-tertiary); color: var(--text-secondary); font-weight: 600; padding: 0.75rem 1rem; border-bottom: 1px solid var(--border-color); }
        td { padding: 0.75rem 1rem; border-bottom: 1px solid var(--border-color); color: var(--text-primary); }
        tr:last-child td { border-bottom: none; }
        .badge { display: inline-block; padding: 0.2rem 0.5rem; border-radius: 4px; font-size: 0.75rem; font-weight: 600; font-family: var(--font-mono); }
        .badge-success { background: rgba(63, 185, 80, 0.15); color: var(--accent-green); border: 1px solid var(--accent-green); }
        .badge-danger { background: rgba(248, 81, 73, 0.15); color: var(--accent-red); border: 1px solid var(--accent-red); }
        .badge-model { background: rgba(88, 166, 255, 0.15); color: var(--accent-blue); border: 1px solid var(--accent-blue); }
        .badge-provider { background: rgba(188, 140, 255, 0.15); color: var(--accent-purple); border: 1px solid var(--accent-purple); }
        .badge-tier { background: rgba(210, 153, 34, 0.15); color: var(--accent-yellow); border: 1px solid var(--accent-yellow); }
        .btn {
            background: var(--bg-tertiary); border: 1px solid var(--border-color); color: var(--text-primary);
            padding: 0.4rem 0.8rem; border-radius: 4px; cursor: pointer; font-size: 0.75rem; font-weight: 600;
        }
        .btn:hover { background: var(--border-color); }
        .font-mono { font-family: var(--font-mono); }
        .text-cyan { color: var(--accent-blue); }
        .text-green { color: var(--accent-green); }
        .text-yellow { color: var(--accent-yellow); }
        .text-muted { color: var(--text-muted); }
        .text-right { text-align: right; }
        .text-center { text-align: center; }
        .hidden { display: none; }
        footer { text-align: center; padding: 2rem 0; color: var(--text-muted); font-size: 0.8rem; border-top: 1px solid var(--border-color); }
        @media (max-width: 768px) {
            body { padding: 0.5rem; }
            .brand-title { font-size: 1.25rem; }
            .stat-value { font-size: 1.4rem; }
            .search-box { max-width: 100%; }
        }
"#;

pub const FORGE_HTML_JS: &str = r#"
        document.addEventListener('DOMContentLoaded', function() {
            const searchInput = document.getElementById('filter-search');
            const tabButtons = document.querySelectorAll('.tab-btn');
            const aiSection = document.getElementById('ai-section');
            const webhookSection = document.getElementById('webhook-section');
            const ledgerSection = document.getElementById('ledger-section');

            tabButtons.forEach(btn => {
                btn.addEventListener('click', function() {
                    tabButtons.forEach(b => b.classList.remove('active'));
                    this.classList.add('active');
                    const filter = this.getAttribute('data-filter');

                    if (filter === 'all') {
                        aiSection.classList.remove('hidden');
                        webhookSection.classList.remove('hidden');
                        ledgerSection.classList.remove('hidden');
                    } else if (filter === 'ai') {
                        aiSection.classList.remove('hidden');
                        webhookSection.classList.add('hidden');
                        ledgerSection.classList.add('hidden');
                    } else if (filter === 'webhook') {
                        aiSection.classList.add('hidden');
                        webhookSection.classList.remove('hidden');
                        ledgerSection.classList.add('hidden');
                    } else if (filter === 'ledger') {
                        aiSection.classList.add('hidden');
                        webhookSection.classList.add('hidden');
                        ledgerSection.classList.remove('hidden');
                    }
                });
            });

            searchInput.addEventListener('input', function() {
                const query = this.value.toLowerCase();
                const rows = document.querySelectorAll('.data-row');
                rows.forEach(row => {
                    const text = row.textContent.toLowerCase();
                    if (text.includes(query)) {
                        row.style.display = '';
                    } else {
                        row.style.display = 'none';
                    }
                });
            });

            document.querySelectorAll('.action-replay').forEach(btn => {
                btn.addEventListener('click', function() {
                    const id = this.getAttribute('data-id');
                    navigator.clipboard.writeText('forgetunnel webhook --replay ' + id).then(() => {
                        const original = this.textContent;
                        this.textContent = 'Copied!';
                        setTimeout(() => { this.textContent = original; }, 1500);
                    });
                });
            });

            const exportBtn = document.getElementById('btn-export-json');
            if (exportBtn) {
                exportBtn.addEventListener('click', function() {
                    window.open(window.location.pathname.replace('.html', '.json'), '_blank');
                });
            }
        });
"#;
