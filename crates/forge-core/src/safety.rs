use serde::{Deserialize, Serialize};
use crate::error::{ForgeError, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SafetyTier {
    Tier1ReadOnly = 1,
    Tier2SafeInvocation = 2,
    Tier3StateChange = 3,
    Tier4DangerDestructive = 4,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SafetyDecision {
    Allowed,
    DryRunOnly { plan: String },
    Blocked { reason: String },
}

pub struct SafetyEvaluator;

impl SafetyEvaluator {
    pub fn classify_path(path: &str, method: &str) -> SafetyTier {
        let path = path.to_lowercase();
        let method = method.to_uppercase();

        if method == "GET" || method == "HEAD" {
            if path.contains("/delete") || path.contains("/rm") {
                return SafetyTier::Tier4DangerDestructive;
            }
            return SafetyTier::Tier1ReadOnly;
        }

        if path.contains("/api/delete") || path.contains("/api/rm") || path.contains("/delete") || path.contains("/system/exec") {
            return SafetyTier::Tier4DangerDestructive;
        }

        if path.contains("/api/pull") || path.contains("/api/create") || path.contains("/api/copy") {
            return SafetyTier::Tier3StateChange;
        }

        if path.contains("/api/generate") || path.contains("/api/chat") || path.contains("/v1/chat/completions") || path.contains("/webhook") {
            return SafetyTier::Tier2SafeInvocation;
        }

        SafetyTier::Tier2SafeInvocation
    }

    pub fn evaluate(
        tier: SafetyTier,
        action: &str,
        has_auth_token: bool,
        allow_state_changes: bool,
        danger_confirmation_code: Option<&str>,
        expected_danger_code: &str,
    ) -> Result<SafetyDecision> {
        match tier {
            SafetyTier::Tier1ReadOnly => Ok(SafetyDecision::Allowed),

            SafetyTier::Tier2SafeInvocation => {
                if has_auth_token {
                    Ok(SafetyDecision::Allowed)
                } else {
                    Err(ForgeError::Auth("Authentication token required for invocation".into()))
                }
            }

            SafetyTier::Tier3StateChange => {
                if !has_auth_token {
                    return Err(ForgeError::Auth("Authentication token required for state changes".into()));
                }
                if allow_state_changes {
                    Ok(SafetyDecision::Allowed)
                } else {
                    Ok(SafetyDecision::DryRunOnly {
                        plan: format!("DRY-RUN: Action '{}' requires --allow-state-changes flag or UI confirmation.", action),
                    })
                }
            }

            SafetyTier::Tier4DangerDestructive => {
                if !has_auth_token {
                    return Err(ForgeError::Auth("Authentication token required for destructive operations".into()));
                }
                if let Some(code) = danger_confirmation_code {
                    if code == expected_danger_code && !code.is_empty() {
                        return Ok(SafetyDecision::Allowed);
                    }
                }
                Err(ForgeError::SafetyViolation {
                    tier: format!("{:?}", tier),
                    action: action.to_string(),
                    reason: "Destructive operation strictly blocked without valid danger confirmation key.".into(),
                })
            }
        }
    }
}
