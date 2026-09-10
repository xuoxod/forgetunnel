use forge_ai::{NdjsonParser, OllamaClient};

#[test]
fn test_rw_ndjson_parser_chunked_stream() {
    let mut parser = NdjsonParser::new();
    let raw_payload_part1 = "{\"model\":\"llama3\",\"response\":\"Hello \",\"done\":false}\n{\"model\":\"llama3\",\"response\":\"wor";
    let raw_payload_part2 = "ld!\",\"done\":false}\n{\"model\":\"llama3\",\"response\":\"\",\"done\":true}\n";

    let chunks1 = parser.push_chunk(raw_payload_part1);
    assert_eq!(chunks1.len(), 1);
    assert_eq!(chunks1[0].as_ref().unwrap().response, "Hello ");

    let chunks2 = parser.push_chunk(raw_payload_part2);
    assert_eq!(chunks2.len(), 2);
    assert_eq!(chunks2[0].as_ref().unwrap().response, "world!");
    assert_eq!(chunks2[1].as_ref().unwrap().done, true);
}

#[test]
fn test_ec_ndjson_parser_malformed_lines() {
    let mut parser = NdjsonParser::new();
    let raw = "not a json string\n{\"model\":\"llama3\",\"response\":\"ok\",\"done\":false}\n";
    let chunks = parser.push_chunk(raw);
    assert_eq!(chunks.len(), 2);
    assert!(chunks[0].is_err());
    assert!(chunks[1].is_ok());
    assert_eq!(chunks[1].as_ref().unwrap().response, "ok");
}

#[test]
fn test_sim_mock_ollama_stream_generation() {
    let (chunks, meter) = OllamaClient::mock_stream_response("llama3:8b", "What is Conduit?");
    assert!(chunks.len() >= 5);
    assert_eq!(chunks.last().unwrap().done, true);

    let metrics = meter.snapshot();
    assert_eq!(metrics.prompt_tokens, 3);
    assert!(metrics.completion_tokens > 5);
    assert!(metrics.tokens_per_second > 0.0);
}
