#!/bin/bash

USER="void" # todo(fede) this is ugly
FCC_PATH="/Users/$USER/Projects/fcc/target/debug/fcc"

set -e

echo "Testing FCC"

cd writing-a-c-compiler-tests/

echo "Cleaning previous build"
cargo clean

echo "Building fcc"
cargo build

echo "Testing lexer"
./test_compiler $FCC_PATH --chapter 1 --stage lex

echo "Testing parser"
./test_compiler $FCC_PATH --chapter 1 --stage parse

echo "Testing codegen"
./test_compiler $FCC_PATH --chapter 1 --stage codegen

