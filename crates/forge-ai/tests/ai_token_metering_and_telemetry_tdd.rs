use forge_ai::{NdjsonParser, OllamaClient};
use forge_core::TokenMeter;

#[tokio::test]
async fn test_rw_ollama_mock_stream_token_metering() {
    let (_chunks, meter) = OllamaClient::mock_stream_response("llama3:latest", "Tell me a joke");

    let snapshot = meter.snapshot();
    assert!(snapshot.prompt_tokens > 0);
    assert!(snapshot.completion_tokens > 0);
    assert!(snapshot.ttft_ms >= 0.0);
    assert!(snapshot.tokens_per_second >= 0.0);
}

#[tokio::test]
async fn test_ec_empty_prompt_and_single_chunk() {
    let mut meter = TokenMeter::new();
    meter.set_prompt_tokens(0);
    meter.record_chunk(1, 10);

    let snap = meter.snapshot();
    assert_eq!(snap.prompt_tokens, 0);
    assert_eq!(snap.completion_tokens, 1);
}

#[test]
fn test_attack_malformed_ndjson_recovery() {
    let mut parser = NdjsonParser::new();
    let malformed = "{\"model\":\"llama3\",\"response\":\"Valid part \",\"done\":false}\n{corrupted_json_chunk\n{\"model\":\"llama3\",\"response\":\"recovered\",\"done\":true}\n";
    let results = parser.push_chunk(malformed);

    let valid_chunks: Vec<_> = results.into_iter().filter_map(|r| r.ok()).collect();
    assert_eq!(valid_chunks.len(), 2, "Parser must recover and yield valid chunks while discarding malformed lines");
    assert_eq!(valid_chunks[0].response, "Valid part ");
    assert_eq!(valid_chunks[1].response, "recovered");
    assert!(valid_chunks[1].done);
}
