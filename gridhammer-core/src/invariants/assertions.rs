use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViolationLevel {
    Warning, // Logged but simulation continues
    Error,   // Simulation should probably stop or strictly flagged as failure
    Panic,   // Internal inconsistency, crash immediately
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Violation {
    pub rule_id: String,
    pub message: String,
    pub level: ViolationLevel,
    pub tick: u64,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
}

impl Violation {
    pub fn new(
        rule_id: impl Into<String>,
        message: impl Into<String>,
        level: ViolationLevel,
        tick: u64,
        source: impl Into<String>,
    ) -> Self {
        Self {
            rule_id: rule_id.into(),
            message: message.into(),
            level,
            tick,
            source: source.into(),
            context: None,
        }
    }

    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }
}

/// A registry for collecting assertions violations during simulation
#[derive(Debug, Default)]
pub struct AssertionRegistry {
    violations: Vec<Violation>,
}

impl AssertionRegistry {
    pub fn new() -> Self {
        Self {
            violations: Vec::new(),
        }
    }

    pub fn record(&mut self, violation: Violation) {
        self.violations.push(violation);
    }

    pub fn violations(&self) -> &[Violation] {
        &self.violations
    }

    pub fn has_errors(&self) -> bool {
        self.violations
            .iter()
            .any(|v| matches!(v.level, ViolationLevel::Error | ViolationLevel::Panic))
    }

    pub fn clear(&mut self) {
        self.violations.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_violation_creation() {
        let v = Violation::new(
            "RULE-001",
            "Something bad",
            ViolationLevel::Error,
            10,
            "FSM",
        );
        assert_eq!(v.rule_id, "RULE-001");
        assert_eq!(v.level, ViolationLevel::Error);
        assert_eq!(v.tick, 10);
    }

    #[test]
    fn test_registry() {
        let mut registry = AssertionRegistry::new();
        assert!(!registry.has_errors());

        registry.record(Violation::new(
            "WARN-01",
            "Warning",
            ViolationLevel::Warning,
            1,
            "Test",
        ));
        assert!(!registry.has_errors());

        registry.record(Violation::new(
            "ERR-01",
            "Error",
            ViolationLevel::Error,
            2,
            "Test",
        ));
        assert!(registry.has_errors());
        assert_eq!(registry.violations().len(), 2);
    }
}
