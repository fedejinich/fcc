#!/bin/bash

set -ue

cd "$(dirname "$0")/.."

# Default: run both
RUN_UNIT=false
RUN_COMPLIANCE=false

usage() {
  echo "Usage: bash scripts/tests.sh [OPTIONS]"
  echo ""
  echo "Run unit tests and/or compliance tests for the fcc compiler."
  echo ""
  echo "Options:"
  echo "  --unit        Run only unit tests"
  echo "  --compliance  Run only compliance tests"
  echo "  --help        Show this help message"
  echo ""
  echo "If no options are provided, runs both unit and compliance tests."
}

# Parse arguments
while [[ $# -gt 0 ]]; do
  case $1 in
  --unit)
    RUN_UNIT=true
    shift
    ;;
  --compliance)
    RUN_COMPLIANCE=true
    shift
    ;;
  --help)
    usage
    exit 0
    ;;
  *)
    echo "Unknown option: $1"
    usage
    exit 1
    ;;
  esac
done

# If no flags provided, run both
if [[ "$RUN_UNIT" == false && "$RUN_COMPLIANCE" == false ]]; then
  RUN_UNIT=true
  RUN_COMPLIANCE=true
fi

echo "FCC TEST RUNNER"
echo ""
echo "================================================================================"
echo ""
echo "Cleaning existing build..."
echo ""

cargo clean

echo ""
echo "Building fcc compiler..."
echo ""

cargo build

echo ""

if [[ "$RUN_UNIT" == true ]]; then
  bash scripts/unit_tests.sh
  echo ""
  echo "================================================================================"
  echo ""
fi

if [[ "$RUN_COMPLIANCE" == true ]]; then
  bash scripts/compliance_tests.sh
  echo ""
  echo "================================================================================"
  echo ""
fi

echo "TESTS COMPLETED SUCCESSFULLY!"
