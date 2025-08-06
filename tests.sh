#!/bin/bash

FCC_PATH="/Users/void/Projects/fcc/target/debug/fcc"

set -e

echo "Testing FCC"

cd writing-a-c-compiler-tests/

echo "Testing lexer"
./test_compiler $FCC_PATH --chapter 1 --stage lex

