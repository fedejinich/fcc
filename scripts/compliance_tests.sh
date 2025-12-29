#!/bin/bash

FCC_PATH="$HOME/Projects/fcc/target/debug/fcc"

set -e

STAGE=""
CHAPTER=""

# Parse arguments
while [[ $# -gt 0 ]]; do
  case $1 in
  --stage)
    STAGE="$2"
    shift 2
    ;;
  --chapter)
    CHAPTER="$2"
    shift 2
    ;;
  *)
    shift
    ;;
  esac
done

echo "RUNNING COMPLIANCE TESTS"
echo ""
echo "---"

cd "$(dirname "$0")/.."

cd writing-a-c-compiler-tests/

# Default chapter if not provided
if [[ -z "$CHAPTER" ]]; then
  CHAPTER=7
fi

echo ""

STAGE_FLAG=""
if [[ -n "$STAGE" ]]; then
  STAGE_FLAG="--stage $STAGE"

  echo "Running with stage: $STAGE"
fi
echo "Running latests tests. Chapter $CHAPTER"
echo ""

./test_compiler "$FCC_PATH" --chapter $CHAPTER $STAGE_FLAG --bitwise --latest-only
echo ""

if ! [[ -n "$STAGE" ]]; then
  echo "Running all tests"
  ./test_compiler "$FCC_PATH" --chapter $CHAPTER $STAGE_FLAG --bitwise
  echo ""
fi

echo "COMPLIANCE TESTS COMPLETED SUCCESSFULLY!"
