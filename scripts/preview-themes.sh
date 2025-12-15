#!/usr/bin/env bash
# Preview phos themes
# Uses release build - debug is ~6x slower due to regex compilation overhead
set -e
cargo build --release --quiet 2>/dev/null || cargo build --release
P="./target/release/phos"

S="INFO slot=12345 Synced | WARN timeout | ERROR 0x4f6a8b2c1d"

echo "phos themes:"
for t in $($P list-themes 2>/dev/null | awk '/^  /{print $1}'); do
    printf "%-12s" "$t:"
    echo "$S" | $P -c geth -t "$t"
done
