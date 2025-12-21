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

### Available flags

```bash
# Stop at specific compilation stages
./fcc program.c --lex        # Stop after lexing
./fcc program.c --parse      # Stop after parsing
./fcc program.c --validate   # Stop after semantic analysis
./fcc program.c --tacky      # Stop after generating TACKY IR
./fcc program.c --codegen    # Stop after generating assembly

# Debug output
./fcc program.c --debug       # Enable debug logging
./fcc program.c --trace       # Enable verbose logging
./fcc program.c --print-ast   # Print the AST
./fcc program.c --print-tacky # Print the TACKY IR
```

## Build

```bash
cargo build --release
```

## Test

The project includes comprehensive unit tests:

```bash
bash tests.sh # runs tests for all chapters
```

## Progress

- [x] Chapter 1-4: Expressions and operators
- [ ] Chapter 5: Local variables
