#!/usr/bin/env bash
#
# Rust CLI Mastery - Exercise Verification Script
# Tests compilation of all exercises to ensure repository health
#

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Counters
TOTAL=0
PASSED=0
FAILED=0
SKIPPED=0

# Arrays to track results
FAILED_EXERCISES=()
SKIPPED_EXERCISES=()

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}Rust CLI Mastery - Exercise Verification${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

# Find the exercises directory
if [ ! -d "exercises" ]; then
    echo -e "${RED}Error: exercises directory not found${NC}"
    echo "Please run this script from the repository root"
    exit 1
fi

cd exercises

# Count total exercises
TOTAL=$(find . -maxdepth 1 -type d -name "ex*" | wc -l)
echo -e "Found ${BLUE}${TOTAL}${NC} exercise directories"
echo ""

# Test each exercise
for dir in ex*/; do
    # Remove trailing slash
    exercise="${dir%/}"
    
    # Check if Cargo.toml exists
    if [ ! -f "${exercise}/Cargo.toml" ]; then
        echo -e "${YELLOW}⊘${NC} ${exercise} - No Cargo.toml (setup exercise)"
        ((SKIPPED++))
        SKIPPED_EXERCISES+=("$exercise")
        continue
    fi
    
    # Try to build
    printf "Testing ${BLUE}%-40s${NC} ... " "$exercise"
    
    if cargo build --manifest-path "${exercise}/Cargo.toml" --quiet 2>/dev/null; then
        echo -e "${GREEN}✓ PASS${NC}"
        ((PASSED++))
    else
        echo -e "${RED}✗ FAIL${NC}"
        ((FAILED++))
        FAILED_EXERCISES+=("$exercise")
    fi
done

# Summary
echo ""
echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}Test Summary${NC}"
echo -e "${BLUE}========================================${NC}"
echo -e "Total Exercises:   ${BLUE}${TOTAL}${NC}"
echo -e "Passed:            ${GREEN}${PASSED}${NC}"
echo -e "Failed:            ${RED}${FAILED}${NC}"
echo -e "Skipped:           ${YELLOW}${SKIPPED}${NC} (setup exercises)"
echo ""

# Show failed exercises
if [ ${FAILED} -gt 0 ]; then
    echo -e "${RED}Failed Exercises:${NC}"
    for ex in "${FAILED_EXERCISES[@]}"; do
        echo -e "  ${RED}✗${NC} $ex"
    done
    echo ""
fi

# Show skipped exercises
if [ ${SKIPPED} -gt 0 ]; then
    echo -e "${YELLOW}Skipped Exercises (no Cargo.toml):${NC}"
    for ex in "${SKIPPED_EXERCISES[@]}"; do
        echo -e "  ${YELLOW}⊘${NC} $ex"
    done
    echo ""
fi

# Calculate success rate
if [ $((PASSED + FAILED)) -gt 0 ]; then
    SUCCESS_RATE=$((PASSED * 100 / (PASSED + FAILED)))
    echo -e "Success Rate: ${GREEN}${SUCCESS_RATE}%${NC}"
fi

# Exit code
if [ ${FAILED} -gt 0 ]; then
    echo ""
    echo -e "${RED}Some tests failed!${NC}"
    exit 1
else
    echo ""
    echo -e "${GREEN}All tests passed! ✨${NC}"
    exit 0
fi
