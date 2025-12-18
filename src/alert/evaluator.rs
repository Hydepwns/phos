//! Alert condition evaluation.

use super::condition::{AlertCondition, AlertSeverity};
use super::formatter::AlertPayload;
use crate::programs::common::log_levels::ERROR_LEVEL_PATTERN;
use regex::Regex;

/// State for evaluating alert conditions.
pub struct ConditionEvaluator {
    /// Pattern for detecting ERROR log level.
    /// Uses shared pattern from `common::log_levels`.
    error_pattern: Regex,
    /// Last known peer count for detecting drops.
    last_peer_count: Option<usize>,
    /// Last known slot for detecting sync stalls.
    last_slot: Option<u64>,
    /// Number of lines since last slot change.
    lines_since_slot_change: usize,
    /// Whether error condition has fired (fire-once).
    error_fired: bool,
}

impl ConditionEvaluator {
    /// Create a new condition evaluator.
    pub fn new() -> Self {
        Self {
            error_pattern: ERROR_LEVEL_PATTERN.clone(),
            last_peer_count: None,
            last_slot: None,
            lines_since_slot_change: 0,
            error_fired: false,
        }
    }

    /// Evaluate a condition against the current line and stats.
    ///
    /// Returns Some(AlertPayload) if the condition fires, None otherwise.
    pub fn evaluate(
        &mut self,
        condition: &AlertCondition,
        line: &str,
        error_count: usize,
        peer_count: Option<usize>,
        slot: Option<u64>,
        program: Option<&str>,
    ) -> Option<AlertPayload> {
        match condition {
            AlertCondition::Error => self.evaluate_error(line, program),
            AlertCondition::ErrorThreshold { count } => {
                self.evaluate_error_threshold(line, error_count, *count, program)
            }
            AlertCondition::PeerDrop { threshold } => {
                self.evaluate_peer_drop(line, peer_count, *threshold, program)
            }
            AlertCondition::SyncStall => self.evaluate_sync_stall(line, slot, program),
            AlertCondition::Pattern { regex } => evaluate_pattern(line, regex, program),
        }
    }

    /// Update internal state with new metrics (call once per line).
    pub fn update_state(&mut self, peer_count: Option<usize>, slot: Option<u64>) {
        // Update peer count tracking
        if let Some(peers) = peer_count {
            self.last_peer_count = Some(peers);
        }

        // Update slot tracking
        match (slot, self.last_slot) {
            (Some(new_slot), Some(last)) if new_slot == last => {
                self.lines_since_slot_change += 1;
            }
            (Some(new_slot), _) => {
                self.last_slot = Some(new_slot);
                self.lines_since_slot_change = 0;
            }
            (None, _) => {
                self.lines_since_slot_change += 1;
            }
        }
    }

    /// Reset the evaluator state.
    pub fn reset(&mut self) {
        self.last_peer_count = None;
        self.last_slot = None;
        self.lines_since_slot_change = 0;
        self.error_fired = false;
    }

    fn evaluate_error(&mut self, line: &str, program: Option<&str>) -> Option<AlertPayload> {
        if self.error_fired {
            return None;
        }

        if self.error_pattern.is_match(line) {
            self.error_fired = true;
            let payload =
                AlertPayload::new("Error Detected", line).with_severity(AlertSeverity::Error);
            Some(add_program(payload, program))
        } else {
            None
        }
    }

    fn evaluate_error_threshold(
        &mut self,
        line: &str,
        error_count: usize,
        threshold: usize,
        program: Option<&str>,
    ) -> Option<AlertPayload> {
        // Fire when error count first reaches threshold
        if error_count == threshold && self.error_pattern.is_match(line) {
            let payload =
                AlertPayload::new(format!("Error Threshold Reached: {threshold} errors"), line)
                    .with_severity(AlertSeverity::Error)
                    .with_field("error_count", error_count.to_string());
            Some(add_program(payload, program))
        } else {
            None
        }
    }

    fn evaluate_peer_drop(
        &mut self,
        line: &str,
        peer_count: Option<usize>,
        threshold: usize,
        program: Option<&str>,
    ) -> Option<AlertPayload> {
        let current = peer_count?;

        // Fire when peers drop below threshold
        if current < threshold {
            if let Some(last) = self.last_peer_count {
                if last >= threshold {
                    // Just crossed below threshold
                    let payload =
                        AlertPayload::new(format!("Peer Count Dropped Below {threshold}"), line)
                            .with_severity(AlertSeverity::Warning)
                            .with_field("previous", last.to_string())
                            .with_field("current", current.to_string());
                    return Some(add_program(payload, program));
                }
            }
        }
        None
    }

    fn evaluate_sync_stall(
        &mut self,
        line: &str,
        _slot: Option<u64>,
        program: Option<&str>,
    ) -> Option<AlertPayload> {
        // Fire after 100 lines with no slot change (rough heuristic)
        // In practice, would use time-based detection
        if self.lines_since_slot_change > 100 {
            if let Some(last_slot) = self.last_slot {
                self.lines_since_slot_change = 0; // Reset to avoid repeated alerts
                let payload = AlertPayload::new("Sync Stall Detected", line)
                    .with_severity(AlertSeverity::Warning)
                    .with_field("last_slot", last_slot.to_string());
                return Some(add_program(payload, program));
            }
        }
        None
    }
}

fn evaluate_pattern(line: &str, regex: &Regex, program: Option<&str>) -> Option<AlertPayload> {
    if regex.is_match(line) {
        let payload = AlertPayload::new("Pattern Match", line)
            .with_severity(AlertSeverity::Warning)
            .with_field("pattern", regex.as_str().to_string());
        Some(add_program(payload, program))
    } else {
        None
    }
}

impl Default for ConditionEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

fn add_program(payload: AlertPayload, program: Option<&str>) -> AlertPayload {
    match program {
        Some(p) => payload.with_program(p),
        None => payload,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_error_fires_once() {
        let mut evaluator = ConditionEvaluator::new();
        let condition = AlertCondition::Error;

        // First error should fire
        let result = evaluator.evaluate(&condition, "ERROR: something failed", 0, None, None, None);
        assert!(result.is_some());

        // Second error should not fire (fire-once)
        let result = evaluator.evaluate(&condition, "ERROR: another failure", 1, None, None, None);
        assert!(result.is_none());
    }

    #[test]
    fn test_evaluate_error_threshold() {
        let mut evaluator = ConditionEvaluator::new();
        let condition = AlertCondition::ErrorThreshold { count: 5 };

        // Below threshold
        let result = evaluator.evaluate(&condition, "ERROR: test", 4, None, None, Some("lodestar"));
        assert!(result.is_none());

        // At threshold
        let result = evaluator.evaluate(&condition, "ERROR: test", 5, None, None, Some("lodestar"));
        assert!(result.is_some());
        let payload = result.unwrap();
        assert!(payload.title.contains("5 errors"));
        assert_eq!(payload.program, Some("lodestar".to_string()));
    }

    #[test]
    fn test_evaluate_peer_drop() {
        let mut evaluator = ConditionEvaluator::new();
        let condition = AlertCondition::PeerDrop { threshold: 10 };

        // Set initial peer count above threshold
        evaluator.update_state(Some(15), None);

        // Peers above threshold - no alert
        let result = evaluator.evaluate(&condition, "peers=15", 0, Some(15), None, None);
        assert!(result.is_none());

        // Peers drop below threshold - should alert
        let result = evaluator.evaluate(&condition, "peers=5", 0, Some(5), None, Some("geth"));
        assert!(result.is_some());
        let payload = result.unwrap();
        assert!(payload.title.contains("Below 10"));
        assert_eq!(payload.fields.get("previous"), Some(&"15".to_string()));
        assert_eq!(payload.fields.get("current"), Some(&"5".to_string()));
    }

    #[test]
    fn test_evaluate_pattern() {
        let mut evaluator = ConditionEvaluator::new();
        let regex = Regex::new(r"FATAL|OOM").unwrap();
        let condition = AlertCondition::Pattern { regex };

        // No match
        let result = evaluator.evaluate(&condition, "INFO: normal log", 0, None, None, None);
        assert!(result.is_none());

        // Match
        let result = evaluator.evaluate(&condition, "FATAL: out of memory", 0, None, None, None);
        assert!(result.is_some());
    }

    #[test]
    fn test_update_state() {
        let mut evaluator = ConditionEvaluator::new();

        evaluator.update_state(Some(50), Some(12345));
        assert_eq!(evaluator.last_peer_count, Some(50));
        assert_eq!(evaluator.last_slot, Some(12345));
        assert_eq!(evaluator.lines_since_slot_change, 0);

        // Same slot, lines counter should increase
        evaluator.update_state(None, Some(12345));
        assert_eq!(evaluator.lines_since_slot_change, 1);

        // New slot, counter should reset
        evaluator.update_state(None, Some(12346));
        assert_eq!(evaluator.lines_since_slot_change, 0);
    }

    #[test]
    fn test_reset() {
        let mut evaluator = ConditionEvaluator::new();

        // Set some state
        evaluator.update_state(Some(50), Some(12345));
        evaluator.error_fired = true;

        // Reset
        evaluator.reset();

        assert_eq!(evaluator.last_peer_count, None);
        assert_eq!(evaluator.last_slot, None);
        assert!(!evaluator.error_fired);
    }

    // =========================================================================
    // EDGE CASE TESTS
    // =========================================================================

    #[test]
    fn test_error_fires_after_reset() {
        let mut evaluator = ConditionEvaluator::new();
        let condition = AlertCondition::Error;

        // First error fires
        let result = evaluator.evaluate(&condition, "ERROR: first", 0, None, None, None);
        assert!(result.is_some());

        // Second error blocked
        let result = evaluator.evaluate(&condition, "ERROR: second", 1, None, None, None);
        assert!(result.is_none());

        // Reset and error fires again
        evaluator.reset();
        let result = evaluator.evaluate(&condition, "ERROR: third", 0, None, None, None);
        assert!(result.is_some());
    }

    #[test]
    fn test_error_requires_pattern_match() {
        let mut evaluator = ConditionEvaluator::new();
        let condition = AlertCondition::Error;

        // INFO line should not fire error condition
        let result = evaluator.evaluate(&condition, "INFO: all good", 0, None, None, None);
        assert!(result.is_none());

        // WARN line should not fire error condition
        let result = evaluator.evaluate(&condition, "WARN: be careful", 0, None, None, None);
        assert!(result.is_none());
    }

    #[test]
    fn test_error_threshold_fires_exactly_at_count() {
        let mut evaluator = ConditionEvaluator::new();
        let condition = AlertCondition::ErrorThreshold { count: 3 };

        // Below threshold
        let result = evaluator.evaluate(&condition, "ERROR: 1", 1, None, None, None);
        assert!(result.is_none());

        let result = evaluator.evaluate(&condition, "ERROR: 2", 2, None, None, None);
        assert!(result.is_none());

        // Exactly at threshold - should fire
        let result = evaluator.evaluate(&condition, "ERROR: 3", 3, None, None, None);
        assert!(result.is_some());

        // Above threshold - should not fire again
        let result = evaluator.evaluate(&condition, "ERROR: 4", 4, None, None, None);
        assert!(result.is_none());
    }

    #[test]
    fn test_error_threshold_zero() {
        let mut evaluator = ConditionEvaluator::new();
        let condition = AlertCondition::ErrorThreshold { count: 0 };

        // Zero threshold fires on first error (count=0)
        let result = evaluator.evaluate(&condition, "ERROR: first", 0, None, None, None);
        assert!(result.is_some());
    }

    #[test]
    fn test_peer_drop_requires_crossing_threshold() {
        let mut evaluator = ConditionEvaluator::new();
        let condition = AlertCondition::PeerDrop { threshold: 10 };

        // Start below threshold - should not alert (no crossing)
        evaluator.update_state(Some(5), None);
        let result = evaluator.evaluate(&condition, "peers=5", 0, Some(5), None, None);
        assert!(result.is_none());

        // Still below - no crossing
        evaluator.update_state(Some(3), None);
        let result = evaluator.evaluate(&condition, "peers=3", 0, Some(3), None, None);
        assert!(result.is_none());
    }

    #[test]
    fn test_peer_drop_no_alert_without_peer_count() {
        let mut evaluator = ConditionEvaluator::new();
        let condition = AlertCondition::PeerDrop { threshold: 10 };

        // No peer count provided - should not fire
        let result = evaluator.evaluate(&condition, "some log", 0, None, None, None);
        assert!(result.is_none());
    }

    #[test]
    fn test_sync_stall_requires_slot_history() {
        let mut evaluator = ConditionEvaluator::new();
        let condition = AlertCondition::SyncStall;

        // No slot history - should not fire even after many lines
        for _ in 0..150 {
            evaluator.update_state(None, None);
        }
        let result = evaluator.evaluate(&condition, "some log", 0, None, None, None);
        assert!(result.is_none());
    }

    #[test]
    fn test_sync_stall_fires_after_threshold() {
        let mut evaluator = ConditionEvaluator::new();
        let condition = AlertCondition::SyncStall;

        // Set initial slot
        evaluator.update_state(None, Some(12345));

        // Simulate many lines without slot change
        for _ in 0..101 {
            evaluator.update_state(None, Some(12345)); // Same slot
        }

        // Should fire now
        let result = evaluator.evaluate(&condition, "some log", 0, None, Some(12345), None);
        assert!(result.is_some());
    }

    #[test]
    fn test_sync_stall_resets_after_firing() {
        let mut evaluator = ConditionEvaluator::new();
        let condition = AlertCondition::SyncStall;

        // Set initial slot and exceed threshold
        evaluator.update_state(None, Some(12345));
        for _ in 0..101 {
            evaluator.update_state(None, Some(12345));
        }

        // Fire once
        let result = evaluator.evaluate(&condition, "stalled", 0, None, Some(12345), None);
        assert!(result.is_some());

        // Immediately after, should not fire (counter reset)
        let result = evaluator.evaluate(&condition, "still stalled", 0, None, Some(12345), None);
        assert!(result.is_none());
    }

    #[test]
    fn test_pattern_complex_regex() {
        let mut evaluator = ConditionEvaluator::new();
        let regex = Regex::new(r"error\[E\d{4}\]").unwrap(); // Rust compiler error pattern
        let condition = AlertCondition::Pattern { regex };

        assert!(
            evaluator
                .evaluate(
                    &condition,
                    "error[E0382]: borrow of moved value",
                    0,
                    None,
                    None,
                    None
                )
                .is_some()
        );

        assert!(
            evaluator
                .evaluate(&condition, "warning: unused variable", 0, None, None, None)
                .is_none()
        );
    }

    #[test]
    fn test_evaluate_with_program() {
        let mut evaluator = ConditionEvaluator::new();
        let condition = AlertCondition::Error;

        let result =
            evaluator.evaluate(&condition, "ERROR: test", 0, None, None, Some("lighthouse"));

        assert!(result.is_some());
        let payload = result.unwrap();
        assert_eq!(payload.program, Some("lighthouse".to_string()));
    }

    #[test]
    fn test_default_impl() {
        let evaluator = ConditionEvaluator::default();
        assert!(evaluator.last_peer_count.is_none());
        assert!(evaluator.last_slot.is_none());
        assert!(!evaluator.error_fired);
    }
}
