#!/bin/bash

set -ue

cd "$(dirname "$0")/.."

echo "Cleaning .asm, .c, and .i files"

rm *.asm
rm *.c
rm *.i

echo "Cleaning build"

cargo clean

