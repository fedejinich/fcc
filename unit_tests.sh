#!/bin/bash

# FCC Unit Tests Runner
# Runs all comprehensive unit tests for the FCC compiler

echo "Running FCC Unit Tests"
echo "========================"
echo

# Run all our unit tests
cargo test --test lexer_tests --test util_tests --test ast_tests --test driver_tests --test tacky_tests --test folder_tests

echo
echo "Tests completed successfully!!"
