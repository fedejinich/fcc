#!/bin/bash

set -ue

cd "$(dirname "$0")/.."

# Default: run both
RUN_UNIT=false
RUN_COMPLIANCE=false
STAGE=""
CHAPTER=""

usage() {
  echo "Usage: bash scripts/tests.sh [OPTIONS]"
  echo ""
  echo "Run unit tests and/or compliance tests for the fcc compiler."
  echo ""
  echo "Options:"
  echo "  --unit              Run only unit tests"
  echo "  --compliance        Run only compliance tests"
  echo "  --stage <STAGE>     Specify stage for compliance tests (only with --compliance)"
  echo "  --chapter <NUM>     Specify chapter for compliance tests (only with --compliance)"
  echo "  --help              Show this help message"
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
  --stage)
    if [[ $# -lt 2 ]]; then
      echo "Error: --stage requires a value"
      exit 1
    fi
    STAGE="$2"
    shift 2
    ;;
  --chapter)
    if [[ $# -lt 2 ]]; then
      echo "Error: --chapter requires a value"
      exit 1
    fi
    CHAPTER="$2"
    shift 2
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

# Validate --stage is only used with --compliance
if [[ -n "$STAGE" && "$RUN_COMPLIANCE" == false ]]; then
  echo "Error: --stage can only be used with --compliance"
  exit 1
fi

# Validate --chapter is only used with --compliance
if [[ -n "$CHAPTER" && "$RUN_COMPLIANCE" == false ]]; then
  echo "Error: --chapter can only be used with --compliance"
  exit 1
fi

# If no flags provided, run both
if [[ "$RUN_UNIT" == false && "$RUN_COMPLIANCE" == false ]]; then
  RUN_UNIT=true
  RUN_COMPLIANCE=true
fi

echo "FCC TEST RUNNER"
echo ""
echo "---"
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
  echo "---"
  echo ""
fi

if [[ "$RUN_COMPLIANCE" == true ]]; then
  COMPLIANCE_ARGS=""
  if [[ -n "$STAGE" ]]; then
    COMPLIANCE_ARGS="$COMPLIANCE_ARGS --stage $STAGE"
  fi
  if [[ -n "$CHAPTER" ]]; then
    COMPLIANCE_ARGS="$COMPLIANCE_ARGS --chapter $CHAPTER"
  fi
  bash scripts/compliance_tests.sh $COMPLIANCE_ARGS
  echo ""
  echo "---"
  echo ""
fi

echo "TESTS COMPLETED SUCCESSFULLY!"
echo ""
