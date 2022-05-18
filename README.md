# fcc (another C compiler)

A compiler for a tiny subset of C, written in Rust. Inspired by [Nora Sandler's book](https://norasandler.com/2022/03/29/Write-a-C-Compiler-the-Book.html).

Requirements
- ONLY targets `x86_64` architectures.
- `gcc`: used for linking the assembly files.

## Clone

```bash
git clone https://github.com/fedejinich/fcc.git --recurse-submodules
```

## Build

Build `fcc` cargo project and output and executable to `~/target/debug`.

```bash
cargo build
```

## Compile With fcc

Compile .c files with `fcc` compiler.

```bash 
./fcc <SOURCE_PATH>
```

## Run Integration Tests 

You can run the integration test suite from [Nora's repo](https://github.com/nlsandler/write_a_c_compiler) by going to `~/fcc/tests/resources/wirte_a_c_compiler` and running:

```
./test_compiler.sh /path/to/fcc
```

#### Test specific stages

To test stage 1 and stage 3,

```bash
./test_compiler.sh /path/to/your/compiler 1 3
```

To test from stage 1 to stage 6,

```bash
./test_compiler.sh /path/to/your/compiler `seq 1 6`
```
