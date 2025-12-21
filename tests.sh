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

echo "Running latests tests. Chapter $CHAPTER"
# ./test_compiler "$FCC_PATH" -v --chapter $CHAPTER --bitwise --latest-only 
./test_compiler "$FCC_PATH" -v --chapter $CHAPTER --stage parse --bitwise
# todo i should add this feature to the cli app

# echo "Running all tests"
# ./test_compiler "$FCC_PATH" -v --chapter $CHAPTER --bitwise

echo "Run all tests successfully!!"
