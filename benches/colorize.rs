//! Benchmarks for phos colorization performance.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use phos::{programs, Colorizer, Theme};

// Sample log lines of varying complexity
const SIMPLE_LINE: &str = "INFO: Application started successfully";
const MEDIUM_LINE: &str =
    "2024-01-15T10:30:45.123Z INFO [main] Starting server on 192.168.1.100:8080 - memory: 256MB";
const COMPLEX_LINE: &str = "Dec 05 00:12:36.557 INFO Synced slot: 12345678, epoch: 385802, peers: 47, head: 0x4f6a8b2c1d3e5f7a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2a";

fn bench_colorize_line_sizes(c: &mut Criterion) {
    let registry = programs::default_registry();
    let rules = registry.get("ethereum.lighthouse").unwrap().rules();
    let theme = Theme::default_dark();

    let mut group = c.benchmark_group("colorize_line_size");

    for (name, line) in [
        ("simple_50b", SIMPLE_LINE),
        ("medium_100b", MEDIUM_LINE),
        ("complex_150b", COMPLEX_LINE),
    ] {
        group.throughput(Throughput::Bytes(line.len() as u64));
        group.bench_with_input(BenchmarkId::new("lighthouse", name), line, |b, line| {
            let mut colorizer = Colorizer::new(rules.clone())
                .with_theme(theme.clone())
                .with_color_enabled(true);
            b.iter(|| colorizer.colorize(black_box(line)))
        });
    }
    group.finish();
}

fn bench_rule_counts(c: &mut Criterion) {
    let registry = programs::default_registry();
    let theme = Theme::default_dark();
    let line = COMPLEX_LINE;

    let mut group = c.benchmark_group("colorize_rule_count");

    for (name, program_id) in [
        ("5_rules", "network.ping"),
        ("20_rules", "dev.cargo"),
        ("50_rules", "ethereum.lighthouse"),
    ] {
        let rules = registry.get(program_id).unwrap().rules();
        group.bench_with_input(BenchmarkId::new("rules", name), &rules, |b, rules| {
            let mut colorizer = Colorizer::new(rules.clone())
                .with_theme(theme.clone())
                .with_color_enabled(true);
            b.iter(|| colorizer.colorize(black_box(line)))
        });
    }
    group.finish();
}

fn bench_themes(c: &mut Criterion) {
    let registry = programs::default_registry();
    let rules = registry.get("dev.cargo").unwrap().rules();
    let line = "error[E0382]: borrow of moved value: `x`";

    let mut group = c.benchmark_group("colorize_themes");

    for theme_name in ["default-dark", "dracula", "nord", "gruvbox"] {
        let theme = Theme::builtin(theme_name).unwrap();
        group.bench_with_input(BenchmarkId::new("theme", theme_name), &theme, |b, theme| {
            let mut colorizer = Colorizer::new(rules.clone())
                .with_theme(theme.clone())
                .with_color_enabled(true);
            b.iter(|| colorizer.colorize(black_box(line)))
        });
    }
    group.finish();
}

fn bench_batch_processing(c: &mut Criterion) {
    let registry = programs::default_registry();
    let rules = registry.get("devops.docker").unwrap().rules();
    let theme = Theme::default_dark();

    // Generate batch of lines
    let lines: Vec<&str> = vec![MEDIUM_LINE; 100];

    c.bench_function("batch_100_lines", |b| {
        let mut colorizer = Colorizer::new(rules.clone())
            .with_theme(theme.clone())
            .with_color_enabled(true);
        b.iter(|| {
            for line in &lines {
                black_box(colorizer.colorize(black_box(line)));
            }
        })
    });
}

criterion_group!(
    benches,
    bench_colorize_line_sizes,
    bench_rule_counts,
    bench_themes,
    bench_batch_processing,
);
criterion_main!(benches);
