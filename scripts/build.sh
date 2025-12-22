#!/bin/bash

cd "$(dirname "$0")/.."

cargo clean
cargo build
