#!/bin/bash

# FCC Unit Tests Runner
# Runs all comprehensive unit tests for the FCC compiler

echo "Running FCC Unit Tests"
echo "========================"
echo

# Run all our unit tests
cargo test --test lexer_tests --test util_tests --test ast_tests --test driver_tests --test tacky_tests

echo
echo "Test Summary:"
echo "  • lexer_tests.rs  - 19 tests (Tokenization & lexical analysis)"
echo "  • util_tests.rs   - 15 tests (Utility functions)"
echo "  • ast_tests.rs    - 16 tests (Abstract syntax tree)"
echo "  • driver_tests.rs - 16 tests (Command-line interface)"
echo "  • tacky_tests.rs  - 16 tests (Intermediate representation)"
echo "  Total: 82 unit tests"

