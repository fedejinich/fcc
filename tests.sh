#!/bin/bash

FCC_PATH="$HOME/Projects/fcc/target/debug/fcc"

set -e

echo "Testing FCC"

cd writing-a-c-compiler-tests/

echo "Cleaning previous build"
cargo clean

echo "Building fcc"
cargo build

CHAPTER=6

echo "Running latests tests"
./test_compiler "$FCC_PATH" -v --chapter $CHAPTER --bitwise --latest-only --stage parse

echo "Running all tests"
./test_compiler "$FCC_PATH" -v --chapter $CHAPTER --bitwise --stage parse
