# FCC Compiler Tests

Unit tests for the FCC C compiler project.

## Structure

```
test/
├── lexer_tests.rs    # Tokenization and lexical analysis (19 tests)
├── util_tests.rs     # Utility functions (15 tests)
├── ast_tests.rs      # Abstract syntax tree structures (16 tests)
├── driver_tests.rs   # Command-line interface (17 tests)
└── tacky_tests.rs    # Intermediate representation (17 tests)
```

## Total Coverage

**84 comprehensive unit tests** covering all major compiler components:

- **Lexer** - Token parsing, operators, edge cases, error handling
- **AST** - Program structures, expressions, declarations, operators
- **Utils** - String manipulation, file extensions, indentation
- **Driver** - CLI flags, argument validation, preprocessing
- **Tacky IR** - Instructions, values, jumps, binary/unary operators

## Features

- **Modular organization** - Each file focuses on a specific component
- **Comprehensive coverage** - Tests all public APIs and edge cases
- **Idiomatic Rust** - Follows Rust testing conventions
- **Easy to maintain** - Clean separation of concerns
- **Extensible** - Simple to add new tests for new features

Each test file can be run independently and focuses on testing the public interface of its corresponding module.