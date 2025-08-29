#!/bin/bash

FCC_PATH="$HOME/Projects/fcc/target/debug/fcc"

set -e

echo "Testing FCC"

cd writing-a-c-compiler-tests/

echo "Cleaning previous build"
cargo clean

echo "Building fcc"
cargo build

echo "Integration tests"
./test_compiler "$FCC_PATH" --chapter 3 --bitwise
