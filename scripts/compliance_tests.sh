#!/bin/bash

FCC_PATH="$HOME/Projects/fcc/target/debug/fcc"

set -e

echo "Running compliance tests"

cd "$(dirname "$0")/.."

echo "Cleaning previous build"
cargo clean

echo "Building fcc"
cargo build

cd writing-a-c-compiler-tests/

CHAPTER=6

echo "Running latests tests. Chapter $CHAPTER"
./test_compiler "$FCC_PATH" -v --chapter $CHAPTER --bitwise --latest-only

# echo "Running all tests"
./test_compiler "$FCC_PATH" -v --chapter $CHAPTER --bitwise

echo "Compliance tests completed successfully!!"
