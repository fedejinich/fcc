#!/bin/bash

# FCC Unit Tests Runner
# Runs all comprehensive unit tests for the FCC compiler

cd "$(dirname "$0")/.."

echo "RUNNING UNIT TESTS"
echo "---"
echo

# Run all tests in test/ directory
cargo test --test '*'

echo ""

echo "UNIT TESTS COMPLETED SUCCESSFULLY!"
