#!/bin/bash

FCC_PATH="$HOME/Projects/fcc/target/debug/fcc"

set -e

echo "Testing FCC"

cd writing-a-c-compiler-tests/

echo "Cleaning previous build"
cargo clean

echo "Building fcc"
cargo build

# echo "Testing chapter 1"
#
# echo "Testing lexer"
# ./test_compiler "$FCC_PATH" --chapter 1 --stage lex
#
# echo "Testing parser"
# ./test_compiler "$FCC_PATH" --chapter 1 --stage parse
#
# echo "Testing codegen"
# ./test_compiler "$FCC_PATH" --chapter 1 --stage codegen
#
# echo "Integration tests"
# ./test_compiler "$FCC_PATH" --chapter 1

# echo "Testing chapter 2"
#
# echo "Testing lexer"
# ./test_compiler "$FCC_PATH" --chapter 2 --stage lex
#
# echo "Testing parser"
# ./test_compiler "$FCC_PATH" --chapter 2 --stage parse
#
# echo "Testing tacky"
# ./test_compiler "$FCC_PATH" --chapter 2 --stage tacky
#
# echo "Testing codegen"
# ./test_compiler "$FCC_PATH" --chapter 2 --stage codegen
#
# echo "Integration tests"
# ./test_compiler "$FCC_PATH" --chapter 2

echo "Testing chapter 3"

# echo "Testing lexer"
# ./test_compiler "$FCC_PATH" --chapter 3 --stage lex
#
# echo "Testing parser"
# ./test_compiler "$FCC_PATH" --chapter 3 --stage parse
#
# echo "Testing tacky"
# ./test_compiler "$FCC_PATH" --chapter 3 --stage tacky
#
# echo "Testing codegen"
# ./test_compiler "$FCC_PATH" --chapter 3 --stage codegen

echo "Integration tests"
./test_compiler "$FCC_PATH" --chapter 3 --bitwise --stage lex
