#!/usr/bin/env bash
# PTY Compatibility Test Suite
# Tests phos across different execution modes

set -uo pipefail

PHOS="${PHOS:-./target/release/phos}"
PASS=0
FAIL=0

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

log_pass() { echo -e "${GREEN}[PASS]${NC} $1"; ((PASS++)) || true; }
log_fail() { echo -e "${RED}[FAIL]${NC} $1"; ((FAIL++)) || true; }
log_section() { echo -e "\n${BLUE}=== $1 ===${NC}"; }

# Check binary exists
if [[ ! -x "$PHOS" ]]; then
    echo "Error: phos not found at $PHOS"
    exit 1
fi

echo "Testing: $PHOS"
echo "Version: $($PHOS --version 2>&1 | head -1)"

# ============================================================================
log_section "Output Completeness"

# git status
expected=$(git status 2>/dev/null | wc -l)
actual=$($PHOS -p git -- git status 2>/dev/null | wc -l)
[[ "$expected" -eq "$actual" ]] && log_pass "git status complete ($actual lines)" || log_fail "git status truncated ($expected vs $actual)"

# git log
expected=$(git log --oneline -20 2>/dev/null | wc -l)
actual=$($PHOS -p git -- git log --oneline -20 2>/dev/null | wc -l)
[[ "$expected" -eq "$actual" ]] && log_pass "git log complete ($actual lines)" || log_fail "git log truncated ($expected vs $actual)"

# seq
expected=10
actual=$($PHOS -p cargo -- seq 1 10 2>/dev/null | wc -l)
[[ "$expected" -eq "$actual" ]] && log_pass "seq complete ($actual lines)" || log_fail "seq truncated ($expected vs $actual)"

# Large output
expected=1000
actual=$($PHOS -p cargo -- seq 1 1000 2>/dev/null | wc -l)
[[ "$expected" -eq "$actual" ]] && log_pass "large output complete ($actual lines)" || log_fail "large output truncated ($expected vs $actual)"

# ============================================================================
log_section "Execution Modes"

# Pipe mode
output=$($PHOS -p git -- git --version 2>/dev/null)
[[ "$output" == *"git version"* ]] && log_pass "pipe mode works" || log_fail "pipe mode failed"

# --no-pty flag
output=$($PHOS --no-pty -p git -- git --version 2>/dev/null)
[[ "$output" == *"git version"* ]] && log_pass "--no-pty flag works" || log_fail "--no-pty flag failed"

# --pty flag
output=$($PHOS --pty -p git -- git --version 2>/dev/null)
[[ "$output" == *"git version"* ]] && log_pass "--pty flag works" || log_fail "--pty flag failed"

# stdin pipe
output=$(echo "ERROR: test" | $PHOS -p cargo 2>/dev/null)
[[ "$output" == *"ERROR"* ]] && log_pass "stdin pipe works" || log_fail "stdin pipe failed"

# ============================================================================
log_section "Exit Code Preservation"

# Exit code 0
$PHOS -p cargo -- true 2>/dev/null
[[ $? -eq 0 ]] && log_pass "exit code 0 preserved" || log_fail "exit code 0 not preserved"

# Exit code 1
$PHOS -p cargo -- false 2>/dev/null || true
$PHOS -p cargo -- false 2>/dev/null; code=$?
[[ $code -eq 1 ]] && log_pass "exit code 1 preserved" || log_fail "exit code 1 not preserved (got $code)"

# Exit code 42
$PHOS -p cargo -- sh -c 'exit 42' 2>/dev/null; code=$?
[[ $code -eq 42 ]] && log_pass "exit code 42 preserved" || log_fail "exit code 42 not preserved (got $code)"

# ============================================================================
log_section "Stderr Handling"

# stderr capture
output=$($PHOS -p cargo -- sh -c 'echo "stderr" >&2' 2>&1)
[[ "$output" == *"stderr"* ]] && log_pass "stderr captured" || log_fail "stderr not captured"

# mixed output
output=$($PHOS -p cargo -- sh -c 'echo "out"; echo "err" >&2' 2>&1)
[[ "$output" == *"out"* ]] && [[ "$output" == *"err"* ]] && log_pass "mixed output captured" || log_fail "mixed output failed"

# ============================================================================
log_section "Auto-Detection"

# git auto-detect
output=$($PHOS -- git --version 2>/dev/null)
[[ "$output" == *"git"* ]] && log_pass "git auto-detected" || log_fail "git not auto-detected"

# cargo auto-detect
output=$($PHOS -- cargo --version 2>/dev/null)
[[ "$output" == *"cargo"* ]] && log_pass "cargo auto-detected" || log_fail "cargo not auto-detected"

# ============================================================================
log_section "Summary"
echo "Passed: $PASS"
echo "Failed: $FAIL"

[[ $FAIL -eq 0 ]] && echo -e "${GREEN}All tests passed!${NC}" && exit 0
echo -e "${RED}Some tests failed.${NC}" && exit 1
