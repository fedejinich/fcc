#!/bin/bash

FCC_PATH="$HOME/Projects/fcc/target/debug/fcc"

set -e

echo "RUNNING COMPLIANCE TESTS"

cd "$(dirname "$0")/.."

cd writing-a-c-compiler-tests/

CHAPTER=6

echo ""

echo "Running latests tests. Chapter $CHAPTER"
./test_compiler "$FCC_PATH" -v --chapter $CHAPTER --bitwise --latest-only

echo ""

echo "Running all tests"
./test_compiler "$FCC_PATH" -v --chapter $CHAPTER --bitwise

echo "COMPLIANCE TESTS COMPLETED SUCCESSFULLY!"
