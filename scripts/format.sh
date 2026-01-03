#!/bin/bash

set -ue

cd "$(dirname "$0")/.."

echo "Formatting project with rustfmt"

cargo fmt

echo "Done"
