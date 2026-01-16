//! Rate limiting for webhook alerts.

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Rate limiter using cooldowns and hourly limits.
pub struct RateLimiter {
    /// Global cooldown between any alerts.
    global_cooldown: Duration,
    /// Per-condition cooldown.
    per_condition_cooldown: Duration,
    /// Maximum alerts per hour (0 = unlimited).
    max_per_hour: usize,

    /// Last alert timestamp for global limiting.
    last_alert: Option<Instant>,
    /// Last alert per condition type.
    condition_timestamps: HashMap<String, Instant>,
    /// Alert count in current hour window.
    hourly_count: usize,
    /// Hour window start.
    hour_start: Instant,
}

impl RateLimiter {
    /// Create a new rate limiter with default settings.
    #[must_use]
    pub fn new() -> Self {
        Self {
            global_cooldown: Duration::from_secs(30),
            per_condition_cooldown: Duration::from_secs(60),
            max_per_hour: 50,
            last_alert: None,
            condition_timestamps: HashMap::new(),
            hourly_count: 0,
            hour_start: Instant::now(),
        }
    }

    /// Set global cooldown between any alerts.
    #[must_use]
    pub fn with_global_cooldown(mut self, cooldown: Duration) -> Self {
        self.global_cooldown = cooldown;
        self
    }

    /// Set per-condition cooldown.
    #[must_use]
    pub fn with_per_condition_cooldown(mut self, cooldown: Duration) -> Self {
        self.per_condition_cooldown = cooldown;
        self
    }

    /// Set maximum alerts per hour (0 = unlimited).
    #[must_use]
    pub fn with_max_per_hour(mut self, max: usize) -> Self {
        self.max_per_hour = max;
        self
    }

    /// Check if an alert can be sent for the given condition.
    #[must_use]
    pub fn can_alert(&self, condition: &str) -> RateLimitResult {
        let now = Instant::now();

        // Check global cooldown
        if let Some(last) = self.last_alert {
            let elapsed = now.duration_since(last);
            if elapsed < self.global_cooldown {
                return RateLimitResult::GlobalCooldown {
                    remaining: self.global_cooldown - elapsed,
                };
            }
        }

        // Check hourly limit
        if self.max_per_hour > 0 {
            let hour_elapsed = now.duration_since(self.hour_start);
            if hour_elapsed < Duration::from_secs(3600) && self.hourly_count >= self.max_per_hour {
                return RateLimitResult::HourlyLimitReached {
                    remaining: Duration::from_secs(3600) - hour_elapsed,
                };
            }
        }

        // Check per-condition cooldown
        if let Some(last) = self.condition_timestamps.get(condition) {
            let elapsed = now.duration_since(*last);
            if elapsed < self.per_condition_cooldown {
                return RateLimitResult::ConditionCooldown {
                    condition: condition.to_string(),
                    remaining: self.per_condition_cooldown - elapsed,
                };
            }
        }

        RateLimitResult::Allowed
    }

    /// Record that an alert was sent.
    pub fn record_alert(&mut self, condition: &str) {
        let now = Instant::now();

        self.last_alert = Some(now);
        self.condition_timestamps.insert(condition.to_string(), now);

        // Reset hourly counter if needed
        if now.duration_since(self.hour_start) >= Duration::from_secs(3600) {
            self.hour_start = now;
            self.hourly_count = 0;
        }
        self.hourly_count += 1;
    }

    /// Get the current hourly alert count.
    #[must_use]
    pub fn hourly_count(&self) -> usize {
        self.hourly_count
    }

    /// Reset all rate limiting state.
    pub fn reset(&mut self) {
        self.last_alert = None;
        self.condition_timestamps.clear();
        self.hourly_count = 0;
        self.hour_start = Instant::now();
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of a rate limit check.
#[derive(Debug, Clone)]
pub enum RateLimitResult {
    /// Alert is allowed.
    Allowed,
    /// Global cooldown not elapsed.
    GlobalCooldown { remaining: Duration },
    /// Per-condition cooldown not elapsed.
    ConditionCooldown {
        condition: String,
        remaining: Duration,
    },
    /// Hourly limit reached.
    HourlyLimitReached { remaining: Duration },
}

impl RateLimitResult {
    /// Returns true if the alert is allowed.
    #[must_use]
    pub fn is_allowed(&self) -> bool {
        matches!(self, Self::Allowed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn test_first_alert_allowed() {
        let limiter = RateLimiter::new();
        assert!(limiter.can_alert("error").is_allowed());
    }

    #[test]
    fn test_global_cooldown() {
        let mut limiter = RateLimiter::new().with_global_cooldown(Duration::from_millis(50));

        assert!(limiter.can_alert("error").is_allowed());
        limiter.record_alert("error");

        // Immediately after, should be blocked
        let result = limiter.can_alert("other");
        assert!(matches!(result, RateLimitResult::GlobalCooldown { .. }));

        // Wait for cooldown
        sleep(Duration::from_millis(60));
        assert!(limiter.can_alert("other").is_allowed());
    }

    #[test]
    fn test_per_condition_cooldown() {
        let mut limiter = RateLimiter::new()
            .with_global_cooldown(Duration::from_millis(10))
            .with_per_condition_cooldown(Duration::from_millis(50));

        limiter.record_alert("error");
        sleep(Duration::from_millis(15)); // Past global cooldown

        // Same condition should still be blocked
        let result = limiter.can_alert("error");
        assert!(matches!(result, RateLimitResult::ConditionCooldown { .. }));

        // Different condition should be allowed
        assert!(limiter.can_alert("peer_drop").is_allowed());
    }

    #[test]
    fn test_hourly_limit() {
        let mut limiter = RateLimiter::new()
            .with_global_cooldown(Duration::ZERO)
            .with_per_condition_cooldown(Duration::ZERO)
            .with_max_per_hour(3);

        for i in 0..3 {
            assert!(limiter.can_alert(&format!("cond{i}")).is_allowed());
            limiter.record_alert(&format!("cond{i}"));
        }

        // 4th alert should be blocked
        let result = limiter.can_alert("cond4");
        assert!(matches!(result, RateLimitResult::HourlyLimitReached { .. }));
    }

    #[test]
    fn test_unlimited_hourly() {
        let mut limiter = RateLimiter::new()
            .with_global_cooldown(Duration::ZERO)
            .with_per_condition_cooldown(Duration::ZERO)
            .with_max_per_hour(0); // Unlimited

        for i in 0..100 {
            assert!(limiter.can_alert(&format!("cond{i}")).is_allowed());
            limiter.record_alert(&format!("cond{i}"));
        }
    }

    #[test]
    fn test_reset() {
        let mut limiter = RateLimiter::new();
        limiter.record_alert("error");
        limiter.record_alert("warn");

        assert_eq!(limiter.hourly_count(), 2);

        limiter.reset();

        assert_eq!(limiter.hourly_count(), 0);
        assert!(limiter.can_alert("error").is_allowed());
    }

    // =========================================================================
    // EDGE CASE TESTS
    // =========================================================================

    #[test]
    fn test_zero_cooldown() {
        let mut limiter = RateLimiter::new()
            .with_global_cooldown(Duration::ZERO)
            .with_per_condition_cooldown(Duration::ZERO)
            .with_max_per_hour(0);

        // With zero cooldown and unlimited hourly, all alerts should be allowed
        for _ in 0..10 {
            assert!(limiter.can_alert("error").is_allowed());
            limiter.record_alert("error");
        }
    }

    #[test]
    fn test_different_conditions_independent() {
        let mut limiter = RateLimiter::new()
            .with_global_cooldown(Duration::from_millis(10))
            .with_per_condition_cooldown(Duration::from_millis(100))
            .with_max_per_hour(0);

        // Record first condition
        limiter.record_alert("error");
        sleep(Duration::from_millis(15)); // Past global cooldown

        // Different condition should be allowed despite first condition's per-condition cooldown
        assert!(limiter.can_alert("peer_drop").is_allowed());

        // Same condition should be blocked
        assert!(!limiter.can_alert("error").is_allowed());
    }

    #[test]
    fn test_rate_limit_result_is_allowed() {
        assert!(RateLimitResult::Allowed.is_allowed());

        assert!(!RateLimitResult::GlobalCooldown {
            remaining: Duration::from_secs(1)
        }
        .is_allowed());

        assert!(!RateLimitResult::ConditionCooldown {
            condition: "error".to_string(),
            remaining: Duration::from_secs(1)
        }
        .is_allowed());

        assert!(!RateLimitResult::HourlyLimitReached {
            remaining: Duration::from_secs(1)
        }
        .is_allowed());
    }

    #[test]
    fn test_default_impl() {
        let limiter = RateLimiter::default();
        // Default should have reasonable values
        assert!(limiter.can_alert("any").is_allowed());
    }

    #[test]
    fn test_builder_pattern() {
        let limiter = RateLimiter::new()
            .with_global_cooldown(Duration::from_secs(10))
            .with_per_condition_cooldown(Duration::from_secs(30))
            .with_max_per_hour(100);

        // Should compile and work
        assert!(limiter.can_alert("test").is_allowed());
    }

    #[test]
    fn test_hourly_count_getter() {
        let mut limiter = RateLimiter::new()
            .with_global_cooldown(Duration::ZERO)
            .with_per_condition_cooldown(Duration::ZERO);

        assert_eq!(limiter.hourly_count(), 0);

        limiter.record_alert("a");
        assert_eq!(limiter.hourly_count(), 1);

        limiter.record_alert("b");
        assert_eq!(limiter.hourly_count(), 2);
    }

    #[test]
    fn test_condition_cooldown_remaining() {
        let mut limiter = RateLimiter::new()
            .with_global_cooldown(Duration::ZERO)
            .with_per_condition_cooldown(Duration::from_millis(100));

        limiter.record_alert("error");

        let result = limiter.can_alert("error");
        match result {
            RateLimitResult::ConditionCooldown { remaining, .. } => {
                // Remaining should be close to 100ms (minus time elapsed)
                assert!(remaining.as_millis() > 0);
                assert!(remaining.as_millis() <= 100);
            }
            _ => panic!("expected ConditionCooldown"),
        }
    }
}
