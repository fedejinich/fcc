# FCC - C Compiler

A C compiler implementation written in Rust.

## Usage

```bash
# Compile a C file
./fcc program.c

# Available flags
./fcc --help
```

## Building

```bash
cargo build --release
```

## Testing

The project includes comprehensive unit tests:

```bash
# Note: Some integration tests may fail without test files
cargo test --lib  # Run only library tests
```

## Project Structure

- `src/lexer.rs` - Tokenization
- `src/ast/` - Abstract syntax tree and parsing
- `src/tacky/` - Intermediate representation
- `src/codegen/` - Assembly code generation
- `src/driver.rs` - CLI and compilation pipeline
- `test/` - Unit tests (84 tests total)

