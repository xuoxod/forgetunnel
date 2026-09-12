/// Sanitize CSV cell against Formula Injection / CSV Injection attacks (CWE-1236)
/// Neutralizes leading '=', '+', '-', '@', '\t', '\r' by prepending a single quote
pub fn sanitize_csv_cell(input: &str) -> String {
    let trimmed = input.trim_start();
    if trimmed.starts_with('=')
        || trimmed.starts_with('+')
        || trimmed.starts_with('-')
        || trimmed.starts_with('@')
        || input.starts_with('\t')
        || input.starts_with('\r')
    {
        format!("'{}", input)
    } else {
        input.to_string()
    }
}

/// Entity-escape HTML characters to prevent XSS attacks
pub fn escape_html(input: &str) -> String {
    let mut output = String::with_capacity(input.len() * 2);
    for c in input.chars() {
        match c {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            '\'' => output.push_str("&#39;"),
            _ => output.push(c),
        }
    }
    output
}
