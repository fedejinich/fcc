#!/bin/bash

# FCC Unit Tests Runner
# Runs all comprehensive unit tests for the FCC compiler

cd "$(dirname "$0")/.."

echo "RUNNING UNIT TESTS"
echo "---"
echo

# Run all our unit tests
cargo test --test lexer_tests --test util_tests --test ast_tests --test driver_tests --test tacky_tests --test folder_tests --test parser_tests --test semantic_tests --test tacky_gen_tests --test codegen_tests

echo ""

echo "UNIT TESTS COMPLETED SUCCESSFULLY!"
