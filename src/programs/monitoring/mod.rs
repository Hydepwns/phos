//! Monitoring and observability programs.
//!
//! Provides Program implementations for Prometheus, etc.

use std::sync::Arc;

use super::common;
use crate::colors::SemanticColor;
use crate::program::{ProgramRegistry, SimpleProgram};
use crate::rule::Rule;

/// Register all Monitoring programs with the registry.
pub fn register_all(registry: &mut ProgramRegistry) {
    registry.register(Arc::new(prometheus_program()));
    registry.register(Arc::new(grafana_program()));
    registry.register(Arc::new(datadog_program()));
    registry.register(Arc::new(signoz_program()));
}

// =============================================================================
// PROMETHEUS
// =============================================================================

fn prometheus_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // Structured key=value log format
    // ts=2024-12-05T00:12:36.123Z level=info component=tsdb msg="compaction"
    rules.extend([
        Rule::new(r"\bts=\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d+Z")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
        Rule::new(r"\blevel=error\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\blevel=warn\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\blevel=info\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\blevel=debug\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]);

    // Component names
    rules.push(
        Rule::new(r"\bcomponent=[\w\-]+")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    );

    // Message strings
    rules.push(
        Rule::new(r#"\bmsg="[^"]+""#)
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    );

    // TSDB operations
    rules.extend([
        Rule::new(r"\bcompaction\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bcheckpoint\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bhead\s+GC\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
        Rule::new(r"\bWAL\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    ]);

    // Scraping
    rules.extend([
        Rule::new(r"\bScrape\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\btarget=[\w\-\.:]+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\bjob=[\w\-]+")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bscrape_pool=[\w\-]+")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    ]);

    // Service discovery
    rules.extend([
        Rule::new(r"\bdiscovery\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bkubernetes_sd\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        Rule::new(r"\bfile_sd\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    ]);

    // Alerting
    rules.extend([
        Rule::new(r"\balertmanager\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\brule_group=[\w\-]+")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        Rule::new(r"\balert=[\w\-]+")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
    ]);

    // Server lifecycle
    rules.extend([
        Rule::new(r"\bServer is ready to receive web requests\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .bold()
            .build(),
        Rule::new(r"\bLoading configuration file\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bCompleted loading of configuration file\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
    ]);

    // Errors and warnings
    rules.extend([
        Rule::new(r#"\berr="[^"]+""#)
            .unwrap()
            .semantic(SemanticColor::Error)
            .build(),
        Rule::new(r"\bfailed\b")
            .unwrap()
            .semantic(SemanticColor::Failure)
            .build(),
    ]);

    // General key=value pattern (should be after specific ones)
    rules.push(common::key_value_rule());

    rules.push(common::ipv4_rule());
    rules.push(common::duration_rule());
    rules.push(common::size_rule());
    rules.push(common::number_rule());

    rules
}

fn prometheus_program() -> SimpleProgram {
    SimpleProgram::new(
        "monitoring.prometheus",
        "prometheus",
        "Prometheus server logs",
        "monitoring",
        prometheus_rules(),
    )
    .with_detect_patterns(vec!["prometheus", "promtool"])
}

// =============================================================================
// GRAFANA
// =============================================================================

fn grafana_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // Grafana log format: t=timestamp level=info msg="message" logger=component
    rules.push(
        Rule::new(r"t=\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}[^\s]*")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
    );

    // Log levels
    rules.extend([
        Rule::new(r"\blevel=error\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\blevel=warn\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\blevel=info\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\blevel=debug\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]);

    // Logger components
    rules.push(
        Rule::new(r"\blogger=[\w\.\-]+")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    );

    // Message strings
    rules.push(
        Rule::new(r#"\bmsg="[^"]*""#)
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    );

    // Dashboard and panel operations
    rules.extend([
        Rule::new(r"\bdashboard\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bpanel\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\balert\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\bdatasource\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
    ]);

    // User/org info
    rules.extend([
        Rule::new(r"\buser=[\w@\.\-]+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\borgId=\d+")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
        Rule::new(r"\buId=\d+")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
    ]);

    // HTTP request info
    rules.extend([
        Rule::new(r"\bmethod=(GET|POST|PUT|DELETE|PATCH)")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bstatus=\d+")
            .unwrap()
            .semantic(SemanticColor::Number)
            .build(),
        Rule::new(r"\bpath=/[^\s]+")
            .unwrap()
            .semantic(SemanticColor::Value)
            .build(),
    ]);

    // Errors
    rules.push(
        Rule::new(r#"\berror="[^"]*""#)
            .unwrap()
            .semantic(SemanticColor::Error)
            .build(),
    );

    rules.push(common::key_value_rule());
    rules.push(common::ipv4_rule());
    rules.push(common::duration_rule());
    rules.push(common::number_rule());

    rules
}

fn grafana_program() -> SimpleProgram {
    SimpleProgram::new(
        "monitoring.grafana",
        "grafana",
        "Grafana server logs",
        "monitoring",
        grafana_rules(),
    )
    .with_detect_patterns(vec!["grafana", "grafana-server"])
}

// =============================================================================
// DATADOG
// =============================================================================

fn datadog_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // Datadog agent timestamp format
    rules.push(
        Rule::new(r"\d{4}-\d{2}-\d{2}\s+\d{2}:\d{2}:\d{2}\s+\w+")
            .unwrap()
            .semantic(SemanticColor::Timestamp)
            .build(),
    );

    // Log levels (Datadog format)
    rules.extend([
        Rule::new(r"\bERROR\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\bWARN\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\bINFO\b")
            .unwrap()
            .semantic(SemanticColor::Info)
            .build(),
        Rule::new(r"\bDEBUG\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]);

    // Component identifiers
    rules.extend([
        Rule::new(r"\([\w\.\-]+\)")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bcheck=[\w\.\-]+")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    ]);

    // Metrics and checks
    rules.extend([
        Rule::new(r"\bmetric\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bservice_check\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bhost=[\w\.\-]+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    ]);

    // Status indicators
    rules.extend([
        Rule::new(r"\bOK\b")
            .unwrap()
            .semantic(SemanticColor::Success)
            .build(),
        Rule::new(r"\bCRITICAL\b")
            .unwrap()
            .semantic(SemanticColor::Error)
            .bold()
            .build(),
        Rule::new(r"\bWARNING\b")
            .unwrap()
            .semantic(SemanticColor::Warn)
            .build(),
        Rule::new(r"\bUNKNOWN\b")
            .unwrap()
            .semantic(SemanticColor::Debug)
            .build(),
    ]);

    // API key (masked)
    rules.push(
        Rule::new(r"\bapi_key=\*+")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    );

    rules.push(common::key_value_rule());
    rules.push(common::ipv4_rule());
    rules.push(common::duration_rule());
    rules.push(common::size_rule());
    rules.push(common::number_rule());

    rules
}

fn datadog_program() -> SimpleProgram {
    SimpleProgram::new(
        "monitoring.datadog",
        "datadog",
        "Datadog agent logs",
        "monitoring",
        datadog_rules(),
    )
    .with_detect_patterns(vec!["datadog", "datadog-agent", "dd-agent"])
}

// =============================================================================
// SIGNOZ
// =============================================================================

fn signoz_rules() -> Vec<Rule> {
    let mut rules = vec![];

    // SigNoz uses structured JSON-like logging
    rules.push(common::iso_timestamp_rule());

    // Log levels
    rules.extend(common::log_level_rules());

    // Component names
    rules.extend([
        Rule::new(r#""caller":"[^"]+""#)
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r#""component":"[^"]+""#)
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    ]);

    // Traces and spans
    rules.extend([
        Rule::new(r"\btrace_id\b")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\bspan_id\b")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
        Rule::new(r"\b[a-f0-9]{32}\b")
            .unwrap()
            .semantic(SemanticColor::Identifier)
            .build(),
    ]);

    // Query engine
    rules.extend([
        Rule::new(r"\bClickHouse\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bquery\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r#""query":"[^"]*""#)
            .unwrap()
            .semantic(SemanticColor::Value)
            .build(),
    ]);

    // OpenTelemetry
    rules.extend([
        Rule::new(r"\bOTLP\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bOpenTelemetry\b")
            .unwrap()
            .semantic(SemanticColor::Key)
            .build(),
        Rule::new(r"\bexporter\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
        Rule::new(r"\breceiver\b")
            .unwrap()
            .semantic(SemanticColor::Label)
            .build(),
    ]);

    rules.push(common::key_value_rule());
    rules.push(common::ipv4_rule());
    rules.push(common::duration_rule());
    rules.push(common::number_rule());

    rules
}

fn signoz_program() -> SimpleProgram {
    SimpleProgram::new(
        "monitoring.signoz",
        "signoz",
        "SigNoz observability platform logs",
        "monitoring",
        signoz_rules(),
    )
    .with_detect_patterns(vec!["signoz", "signoz-otel-collector"])
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
