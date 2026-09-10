use forge_core::{SafetyDecision, SafetyEvaluator, SafetyTier};

#[test]
fn test_rw_tier1_read_only_always_allowed() {
    let tier = SafetyEvaluator::classify_path("/api/tags", "GET");
    assert_eq!(tier, SafetyTier::Tier1ReadOnly);

    let decision = SafetyEvaluator::evaluate(
        tier,
        "list_models",
        false, // no token
        false,
        None,
        "SECRET-CONFIRM",
    )
    .expect("Tier 1 read-only should succeed without token");

    assert_eq!(decision, SafetyDecision::Allowed);
}

#[test]
fn test_rw_tier2_safe_invocation_with_token() {
    let tier = SafetyEvaluator::classify_path("/api/generate", "POST");
    assert_eq!(tier, SafetyTier::Tier2SafeInvocation);

    let decision = SafetyEvaluator::evaluate(
        tier,
        "generate_text",
        true, // valid token
        false,
        None,
        "SECRET-CONFIRM",
    )
    .expect("Tier 2 invocation should succeed with token");

    assert_eq!(decision, SafetyDecision::Allowed);
}

#[test]
fn test_attack_tier2_invocation_unauthenticated_blocked() {
    let tier = SafetyEvaluator::classify_path("/api/generate", "POST");
    let result = SafetyEvaluator::evaluate(
        tier,
        "generate_text",
        false, // unauthenticated
        false,
        None,
        "SECRET-CONFIRM",
    );
    assert!(result.is_err(), "Unauthenticated invocation must fail");
}

#[test]
fn test_rw_tier3_state_change_dry_run_mode() {
    let tier = SafetyEvaluator::classify_path("/api/pull", "POST");
    assert_eq!(tier, SafetyTier::Tier3StateChange);

    let decision = SafetyEvaluator::evaluate(
        tier,
        "pull_llama3",
        true,
        false, // allow_state_changes = false
        None,
        "SECRET-CONFIRM",
    )
    .expect("Evaluation should return dry-run");

    match decision {
        SafetyDecision::DryRunOnly { plan } => {
            assert!(plan.contains("DRY-RUN"));
            assert!(plan.contains("pull_llama3"));
        }
        _ => panic!("Expected DryRunOnly for Tier 3 when allow_state_changes is false"),
    }
}

#[test]
fn test_attack_tier4_destructive_model_deletion_blocked() {
    let tier = SafetyEvaluator::classify_path("/api/delete", "POST");
    assert_eq!(tier, SafetyTier::Tier4DangerDestructive);

    let result = SafetyEvaluator::evaluate(
        tier,
        "delete_model_llama3",
        true,
        true,
        None, // missing danger confirmation code
        "SECRET-CONFIRM-99",
    );
    assert!(result.is_err(), "Destructive deletion without confirmation code must fail");
}

#[test]
fn test_rw_tier4_destructive_with_valid_danger_code() {
    let tier = SafetyEvaluator::classify_path("/api/delete", "POST");
    assert_eq!(tier, SafetyTier::Tier4DangerDestructive);

    let decision = SafetyEvaluator::evaluate(
        tier,
        "delete_model_llama3",
        true,
        true,
        Some("SECRET-CONFIRM-99"),
        "SECRET-CONFIRM-99",
    )
    .expect("Destructive deletion with matching danger confirmation code must succeed");

    assert_eq!(decision, SafetyDecision::Allowed);
}
