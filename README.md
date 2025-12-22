# FCC - C Compiler

A Rust implementation of a C compiler for a small language subset, inspired by Nora Sandler's [C Compiler book](https://norasandler.com/book/).

## Requirements

- Rust (1.70+)
- GCC

> **Note**: On Mac with Apple Silicon, the compiler generates x86_64 code. Run `arch -x86_64 zsh` before running tests or compiled binaries.

## Usage

```bash
# Compile a C file
./fcc program.c

# Help
./fcc --help
```

## Build

```bash
cargo build --release
```

## Test

Run the test suite by running:

```bash
bash scripts/tests.sh # runs both unit and compliance tests
# or
bash scripts/tests.sh --unit # runs unit tests
# or
bash scripts/tests.sh --compliance # runs compliance tests

```

## Progress

- [x] Chapter 1-6: If and conditionals
- [ ] Chapter 7: Compound statements
