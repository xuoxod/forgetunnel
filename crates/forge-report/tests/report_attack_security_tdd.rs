use forge_ledger::AiSessionRecord;
use forge_report::{CsvFormatter, HtmlFormatter, ReportDocument, ReportFormatter};

#[test]
fn test_attack_csv_formula_injection_defense() {
    let mut doc = ReportDocument::default();
    // Inject malicious spreadsheet execution formulas
    doc.ai_sessions.push(AiSessionRecord {
        session_id: "=cmd|'/C calc'!A0".to_string(),
        model: "+SUM(A1:A10)".to_string(),
        prompt_preview: "@HYPERLINK(\"http://evil.com\",\"ClickMe\")".to_string(),
        prompt_tokens: 10,
        completion_tokens: 20,
        ttft_ms: 10.0,
        tps: 30.0,
        duration_ms: 666.0,
        status: "-2+5".to_string(),
        created_at: "\t2026-09-11T20:30:00Z".to_string(),
    });

    let csv = CsvFormatter::render(&doc).expect("CSV render failed");

    // All malicious formula triggers MUST be neutralized with a leading single-quote
    assert!(csv.contains("'\t2026-09-11T20:30:00Z") || csv.contains("'=cmd|'/C calc'!A0"));
    assert!(!csv.contains(",=cmd|'/C calc'!A0"));
    assert!(!csv.contains(",+SUM(A1:A10)"));
    assert!(!csv.contains(",-2+5"));
    assert!(!csv.contains(",@HYPERLINK"));
}

#[test]
fn test_attack_html_xss_injection_sanitization() {
    let mut doc = ReportDocument::default();
    doc.ai_sessions.push(AiSessionRecord {
        session_id: "<script>alert('XSS_SESSION')</script>".to_string(),
        model: "<img src=x onerror=alert('XSS_MODEL')>".to_string(),
        prompt_preview: "\"><script>alert('XSS_PROMPT')</script>".to_string(),
        prompt_tokens: 10,
        completion_tokens: 20,
        ttft_ms: 10.0,
        tps: 30.0,
        duration_ms: 100.0,
        status: "<b>COMPLETED</b>".to_string(),
        created_at: "2026-09-11T20:35:00Z".to_string(),
    });

    let html = HtmlFormatter::render(&doc).expect("HTML render failed");

    // Must NOT contain raw unescaped script or onerror tags in body
    assert!(!html.contains("<script>alert('XSS_SESSION')</script>"));
    assert!(!html.contains("<img src=x onerror="));
    assert!(!html.contains("\"><script>alert("));

    // Must contain entity-escaped strings
    assert!(html.contains("&lt;script&gt;alert(&#39;XSS_SESSION&#39;)&lt;/script&gt;") 
        || html.contains("&lt;script&gt;alert(&#x27;XSS_SESSION&#x27;)&lt;/script&gt;")
        || html.contains("&lt;script&gt;alert('XSS_SESSION')&lt;/script&gt;"));
}

#[test]
fn test_attack_strict_no_inline_css_or_js_attributes() {
    let mut doc = ReportDocument::default();
    doc.ai_sessions.push(AiSessionRecord {
        session_id: "sess_01".to_string(),
        model: "llama3".to_string(),
        prompt_preview: "test".to_string(),
        prompt_tokens: 10,
        completion_tokens: 20,
        ttft_ms: 10.0,
        tps: 30.0,
        duration_ms: 100.0,
        status: "COMPLETED".to_string(),
        created_at: "2026-09-11T20:35:00Z".to_string(),
    });

    let html = HtmlFormatter::render(&doc).expect("HTML render failed");

    // Parse HTML lines and ensure NO inline HTML attributes like style="..." or onclick="..." exist on elements
    // Split into body section
    let body_start = html.find("<body>").expect("Missing <body> tag");
    let body_end = html.find("</body>").expect("Missing </body> tag");
    let body_content = &html[body_start..body_end];

    // Check forbidden inline attribute patterns
    let forbidden_patterns = [
        " style=\"", " style='",
        " onclick=\"", " onclick='",
        " onload=\"", " onload='",
        " onerror=\"", " onerror='",
        " onmouseover=\"", " onmouseover='",
        " onchange=\"", " onchange='",
    ];

    for pattern in forbidden_patterns {
        assert!(
            !body_content.to_lowercase().contains(pattern),
            "HTML body must NOT contain inline attribute '{}'",
            pattern
        );
    }
}

#[test]
fn test_attack_strict_content_security_policy_enforcement() {
    let doc = ReportDocument::default();
    let html = HtmlFormatter::render(&doc).expect("HTML render failed");

    // Must include strict CSP meta header preventing remote CDN exfiltration
    assert!(html.contains("Content-Security-Policy"));
    assert!(html.contains("default-src 'none'"));
}
