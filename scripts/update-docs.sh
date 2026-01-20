#!/bin/bash
# Update dynamic values in documentation
set -e

cd "$(dirname "$0")/.."

# Get actual program count from binary
COUNT=$(cargo run --bin phos --quiet -- list 2>/dev/null | grep -c "^\s\s" || echo "99")

# Update README.md
sed -i '' "s/[0-9]* programs built-in/${COUNT} programs built-in/" README.md
sed -i '' "s/List all [0-9]* programs/List all ${COUNT} programs/" README.md
sed -i '' "s/## Programs ([0-9]*)/## Programs (${COUNT})/" README.md

echo "Updated README.md with ${COUNT} programs"
