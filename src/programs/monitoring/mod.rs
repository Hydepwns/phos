//! Monitoring and observability programs.
//!
//! Provides Program implementations for Prometheus, Grafana, etc.

mod datadog;
mod grafana;
mod prometheus;
mod signoz;

use crate::program::ProgramRegistry;

pub use datadog::datadog_program;
pub use grafana::grafana_program;
pub use prometheus::prometheus_program;
pub use signoz::signoz_program;

/// Register all Monitoring programs with the registry.
pub fn register_all(registry: &mut ProgramRegistry) {
    registry.register(prometheus_program());
    registry.register(grafana_program());
    registry.register(datadog_program());
    registry.register(signoz_program());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitoring_programs_registered() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);
        assert_eq!(registry.len(), 4);
    }

    #[test]
    fn test_prometheus_detection() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);

        let detected = registry.detect("prometheus --config.file=/etc/prometheus/prometheus.yml");
        assert!(detected.is_some());
        assert_eq!(detected.unwrap().info().name, "prometheus");
    }

    #[test]
    fn test_grafana_detection() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);

        let detected = registry.detect("grafana-server --config=/etc/grafana/grafana.ini");
        assert!(detected.is_some());
        assert_eq!(detected.unwrap().info().name, "grafana");
    }

    #[test]
    fn test_datadog_detection() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);

        let detected = registry.detect("datadog-agent run");
        assert!(detected.is_some());
        assert_eq!(detected.unwrap().info().name, "datadog");
    }

    #[test]
    fn test_signoz_detection() {
        let mut registry = ProgramRegistry::new();
        register_all(&mut registry);

        let detected = registry.detect("signoz-otel-collector --config=config.yaml");
        assert!(detected.is_some());
        assert_eq!(detected.unwrap().info().name, "signoz");
    }
}
