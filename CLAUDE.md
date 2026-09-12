# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

fcc is a Rust implementation of a C compiler for a small language subset, based on Nora Sandler's "Writing a C Compiler" book. It compiles a subset of C to x86_64 assembly on macOS/Linux.

**Current progress:** Chapter 8 (Loops) - see `scripts/compliance_tests.sh:37` for the latest implemented chapter.

**Apple Silicon Note:** The compiler generates x86_64 code. Run `arch -x86_64 zsh` before running tests or compiled binaries on ARM Macs.

## Essential Commands

### Building
```bash
cargo build --release
```

### Testing

```bash
# Run all tests (unit + compliance)
bash scripts/tests.sh

# Run only unit tests
bash scripts/tests.sh --unit

# Run only compliance tests
bash scripts/tests.sh --compliance

# Run compliance tests from a specific chapter
bash scripts/tests.sh --compliance --chapter 8

# Run compliance tests at a specific stage
bash scripts/tests.sh --compliance --stage lex
```

**Unit tests:** Run individual test suites with `cargo test --test <name>`:
- `cargo test --test lexer_tests`
- `cargo test --test parser_tests`
- `cargo test --test semantic_tests`
- `cargo test --test tacky_gen_tests`
- `cargo test --test codegen_tests`
- `cargo test --test folder_tests`

### Compiler Usage

```bash
# Compile a C file
./target/debug/fcc program.c

# Stop at specific stages
./target/debug/fcc program.c --lex       # tokenization only
./target/debug/fcc program.c --parse     # stop after parsing
./target/debug/fcc program.c --validate  # stop after semantic analysis
./target/debug/fcc program.c --tacky     # stop after TACKY IR generation
./target/debug/fcc program.c --codegen   # stop after codegen (before assembly)
./target/debug/fcc program.c -S          # generate assembly only (no linking)

# Debug output
./target/debug/fcc program.c --debug     # debug logging
./target/debug/fcc program.c --trace     # trace logging (most verbose)
./target/debug/fcc program.c --print-ast # print C AST
./target/debug/fcc program.c --print-tacky # print TACKY IR
```

## Architecture Overview

### Compilation Pipeline

The compiler follows a multi-stage pipeline with distinct IR representations:

```
Source (.c)
    ↓ [gcc -E preprocessing]
Preprocessed
    ↓ [Lexer]
Token Stream
    ↓ [Parser]
C AST (c_ast::ast)
    ↓ [Semantic Analysis: VariableResolver + LoopLabeler]
Transformed C AST
    ↓ [TACKY Generation]
TACKY IR (three-address code)
    ↓ [x86_64 Codegen]
Assembly AST (with pseudo-registers)
    ↓ [PseudoRegisterReplacer]
Assembly AST (with stack offsets)
    ↓ [InstructionFixer]
Fixed Assembly AST
    ↓ [Emitter]
Assembly Text (.s)
    ↓ [gcc linking]
Executable
```

### Module Structure

- **`src/lexer.rs`** - Tokenizes preprocessed C source
- **`src/c_ast/`** - C abstract syntax tree
  - `parser.rs` - Recursive descent parser
  - `semantic/var_res.rs` - Variable resolution and renaming
  - `semantic/loop_lab.rs` - Loop labeling for break/continue
- **`src/tacky/`** - Three-address code IR
  - `builder.rs` - TackyBuilder for instruction emission
  - `from.rs` - Lowers C AST to TACKY
- **`src/codegen/x64/`** - x86_64 code generation
  - `from.rs` - Converts TACKY to assembly AST
  - `fixer/reg_replace.rs` - Replaces pseudo-registers with stack offsets
  - `fixer/instruction_fix.rs` - Fixes x86_64 encoding constraints
  - `emit.rs` - Generates assembly text
- **`src/common/`** - Shared utilities
  - `folder.rs` - Visitor pattern traits (FolderC, FolderAsm)
- **`src/driver.rs`** - Orchestrates the pipeline

### Key Architectural Patterns

#### 1. Folder Pattern (Visitor/Transformer)

The compiler uses a "folder" pattern for AST transformations. Three traits:
- **`FolderC`** - Transforms C AST → C AST (semantic analysis)
- **`FolderAsm`** - Transforms assembly AST → assembly AST (fixer passes)

Each folder provides:
- Default recursive traversal for all node types
- Override points for specific transformations
- Ability to expand nodes (1 instruction → N instructions via `Vec` return type)

**Usage:** Implement the folder trait and override only the methods you need. The framework handles recursive traversal.

#### 2. TackyBuilder Pattern

Located in `src/tacky/builder.rs`. Encapsulates TACKY IR construction:
- Scoped counter for generating unique temporary names and labels
- Ergonomic `emit_*` methods for common instructions
- "Thin" design: emits exactly what's requested, no reordering

**Usage:**
```rust
let mut builder = TackyBuilder::new();
let tmp = builder.fresh_temp("x");
builder.emit_copy(TackyValue::Constant(42), tmp.clone());
builder.emit_return(tmp);
let instructions = builder.finish();
```

#### 3. From/Into Trait Chains

The pipeline stages chain using Rust's `From` trait:
- `impl From<Program> for TackyProgram` - C AST → TACKY
- `impl From<TackyProgram> for AsmProgram` - TACKY → Assembly

### Semantic Analysis Passes

Located in `src/c_ast/semantic/`, both implement `FolderC`:

#### VariableResolver (`var_res.rs`)
- Renames variables to unique identifiers (e.g., `x` → `x.0`, `x.1`)
- Validates scoping rules (no duplicate declarations, no undeclared usage)
- Uses global `AtomicUsize` counter (`VAR_RES_COUNT`) for unique naming
- Tracks variable declarations per scope using HashMap
- Creates new scope on compound statements and for-loops

#### LoopLabeler (`loop_lab.rs`)
- Assigns unique labels to loops for break/continue statements
- Uses global `AtomicUsize` counter (`LOOP_LABEL_COUNT`)
- Labels are attached directly to loop AST nodes (While/DoWhile/For)
- Validates break/continue only appear inside loops
- Recursively tracks current loop label during traversal

### TACKY IR (Three-Address Code)

Located in `src/tacky/ast.rs`. Intermediate representation with:
- At most 3 operands per instruction
- All computations assigned to named variables (temporaries for intermediates)
- Explicit labels and jumps for control flow

**Key instructions:**
- `Return(TackyValue)`
- `Unary(op, src, dst)` - `dst = op src`
- `Binary(op, src1, src2, dst)` - `dst = src1 op src2`
- `Copy(src, dst)` - `dst = src`
- `Jump(label)`, `JumpIfZero(condition, label)`, `JumpIfNotZero(condition, label)`
- `Label(label)`

**Design benefit:** Separates semantic lowering from x86 lowering; easier to reason about, debug, and test.

### x86_64 Codegen and Fixer Passes

Located in `src/codegen/x64/`:

#### Initial Codegen (`from.rs`)
- Converts TACKY → Assembly AST
- Uses **pseudo-registers** initially (not real x86 registers)
- Helper functions for common patterns:
  - `emit_conditional_jump()` - Cmp + JmpCC
  - `emit_relational()` - Cmp + Mov(0) + SetCC
  - `emit_div_rem()` - Mov→RAX, Cdq, Idiv, Mov→result

#### Fixer Pass 1: PseudoRegisterReplacer (`fixer/reg_replace.rs`)
- Replaces pseudo-registers with stack offsets
- Builds HashMap: `Pseudo(id)` → `Stack(offset)` where offsets are `-4, -8, -12, ...`
- Returns total stack space needed (`last_offset`)
- **Stack layout grows downward from %rbp**

#### Fixer Pass 2: InstructionFixer (`fixer/instruction_fix.rs`)
- Fixes x86_64 instruction encoding constraints
- Uses R10, R11, and CX as scratch registers
- Handles:
  1. Memory-to-memory operations (use R10 as temp)
  2. Division with immediate operand (use R10 as temp)
  3. Multiplication where dest is memory (use R11 as temp)
  4. Shifts with count not in CL (move to CX first)
  5. Compare with immediate as second operand (use R11 as temp)
- Emits `AllocateStack` instruction based on `last_offset` from previous pass
- Can expand 1 instruction → multiple instructions

**Pipeline order matters:** PseudoRegisterReplacer must run before InstructionFixer.

## Important Conventions

### Naming Conventions
- Unique identifiers use dot notation: `variable.N` (e.g., `x.0`, `tmp.42`)
- Loop labels: `loop_st.N` format
- Temporary variables: `tmp.N` format

### Code Style
- Clippy lints: `unwrap_used` and `expect_used` set to "warn"
- Prefer `Result<T, String>` for error handling
- Use `trace!`, `debug!`, `info!`, `error!` logging macros

### Testing Strategy
- Unit tests in `test/` directory, one file per major component
- Compliance tests in `writing-a-c-compiler-tests/` submodule
- Compliance tests driven by chapter number (see `scripts/compliance_tests.sh`)

## Pipeline Consistency Requirement

**CRITICAL:** The pipeline diagram in `src/main.rs` (lines 6-69) must match the latest implemented chapter from the book. The current chapter is specified in `scripts/compliance_tests.sh:37` (default CHAPTER variable).

When implementing new chapters, update both:
1. The pipeline diagram in `src/main.rs` if new passes are added
2. The default chapter number in `scripts/compliance_tests.sh`

## Development Workflow

1. **Adding a new feature:** Typically requires changes across multiple stages:
   - Lexer: Add new tokens if needed
   - Parser: Parse new syntax into C AST nodes
   - Semantic: Add validation/transformation passes
   - TACKY: Lower new AST constructs to TACKY IR
   - Codegen: Generate assembly for new TACKY instructions

2. **Adding a new pass:** Implement the appropriate Folder trait (`FolderC` or `FolderAsm`):
   - Override only the methods you need to transform
   - Default implementations handle recursive traversal
   - Use `Vec` return types to expand nodes when needed

3. **Debugging:** Use the stage flags (`--lex`, `--parse`, etc.) to stop at intermediate stages and inspect output with `--print-ast` or `--print-tacky`.

## File References

When working with code, use the pattern `file_path:line_number` for references:
- C AST types: `src/c_ast/ast.rs`
- Parser logic: `src/c_ast/parser.rs`
- TACKY builder: `src/tacky/builder.rs`
- x86 codegen: `src/codegen/x64/from.rs`
- Fixer passes: `src/codegen/x64/fixer/`
