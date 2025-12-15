//! Alert condition evaluation.

use super::condition::{AlertCondition, AlertSeverity};
use super::formatter::AlertPayload;
use regex::Regex;

/// State for evaluating alert conditions.
pub struct ConditionEvaluator {
    /// Pattern for detecting ERROR log level.
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
            error_pattern: Regex::new(r"(?i)\b(ERROR|ERR|CRIT|CRITICAL|FATAL|PANIC)\b").unwrap(),
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
            AlertCondition::Pattern { regex } => self.evaluate_pattern(line, regex, program),
        }
    }

    /// Update internal state with new metrics (call once per line).
    pub fn update_state(&mut self, peer_count: Option<usize>, slot: Option<u64>) {
        // Update peer count tracking
        if let Some(peers) = peer_count {
            self.last_peer_count = Some(peers);
        }

        // Update slot tracking
        if let Some(new_slot) = slot {
            if self.last_slot != Some(new_slot) {
                self.last_slot = Some(new_slot);
                self.lines_since_slot_change = 0;
            } else {
                self.lines_since_slot_change += 1;
            }
        } else {
            self.lines_since_slot_change += 1;
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
            let payload = AlertPayload::new("Error Detected", line)
                .with_severity(AlertSeverity::Error);
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
            let payload = AlertPayload::new(
                format!("Error Threshold Reached: {threshold} errors"),
                line,
            )
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
                    let payload = AlertPayload::new(
                        format!("Peer Count Dropped Below {threshold}"),
                        line,
                    )
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
        if self.lines_since_slot_change > 100 && self.last_slot.is_some() {
            self.lines_since_slot_change = 0; // Reset to avoid repeated alerts
            let payload = AlertPayload::new("Sync Stall Detected", line)
                .with_severity(AlertSeverity::Warning)
                .with_field("last_slot", self.last_slot.unwrap().to_string());
            Some(add_program(payload, program))
        } else {
            None
        }
    }

    fn evaluate_pattern(
        &self,
        line: &str,
        regex: &Regex,
        program: Option<&str>,
    ) -> Option<AlertPayload> {
        if regex.is_match(line) {
            let payload = AlertPayload::new("Pattern Match", line)
                .with_severity(AlertSeverity::Warning)
                .with_field("pattern", regex.as_str().to_string());
            Some(add_program(payload, program))
        } else {
            None
        }
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
        let result =
            evaluator.evaluate(&condition, "ERROR: another failure", 1, None, None, None);
        assert!(result.is_none());
    }

    #[test]
    fn test_evaluate_error_threshold() {
        let mut evaluator = ConditionEvaluator::new();
        let condition = AlertCondition::ErrorThreshold { count: 5 };

        // Below threshold
        let result =
            evaluator.evaluate(&condition, "ERROR: test", 4, None, None, Some("lodestar"));
        assert!(result.is_none());

        // At threshold
        let result =
            evaluator.evaluate(&condition, "ERROR: test", 5, None, None, Some("lodestar"));
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
        let result =
            evaluator.evaluate(&condition, "peers=15", 0, Some(15), None, None);
        assert!(result.is_none());

        // Peers drop below threshold - should alert
        let result =
            evaluator.evaluate(&condition, "peers=5", 0, Some(5), None, Some("geth"));
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
        let result =
            evaluator.evaluate(&condition, "INFO: normal log", 0, None, None, None);
        assert!(result.is_none());

        // Match
        let result =
            evaluator.evaluate(&condition, "FATAL: out of memory", 0, None, None, None);
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
}
