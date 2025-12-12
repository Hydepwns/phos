//! Build script for phos
//!
//! Captures git information at compile time for version output.

use std::process::Command;

fn main() {
    // Tell Cargo to re-run this if git HEAD changes
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs/heads/");

    // Get git hash
    let git_hash = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    // Check if working directory is dirty
    let is_dirty = Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| !o.stdout.is_empty())
        .unwrap_or(false);

    let dirty_suffix = if is_dirty { "-dirty" } else { "" };

    // Get build date
    let build_date = chrono_lite_date();

    println!("cargo:rustc-env=PHOS_GIT_HASH={git_hash}{dirty_suffix}");
    println!("cargo:rustc-env=PHOS_BUILD_DATE={build_date}");
}

/// Get current date in YYYY-MM-DD format without external dependencies
fn chrono_lite_date() -> String {
    Command::new("date")
        .args(["+%Y-%m-%d"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}
