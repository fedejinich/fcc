# fcc (another C compiler)

A compiler for a tiny subset of C, written in Rust. This is a personal "recreation" of the `gcc` compiler, inspired by [Nora Sandler's book](https://norasandler.com/2022/03/29/Write-a-C-Compiler-the-Book.html).

Requirements
- ONLY targets `x86_64` architectures.
- `gcc`: used for linking the assembly files.
- `docker`: used for working in MacOS M1 environments (OPTIONAL).

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

### Mac OS (with M1)

Since this is only targetting `x86_64` architectures you'll need to run tests inside a docker container, to do so you'll first need to install an `x86_64` compiler by running:

```bash
brew tap messense/macos-cross-toolchains # adds 'macos-cross-toolchains'
brew install x86_64-unknown-linux-gnu # installs x86_64-unknown-linux-gnu toolchain
```

Then run the tests with this script

```bash
./fcc_mac_test.sh
```

This will build the cargo project for `x86_64` linux, then copies the project into the linux container, and finally runs the tests in that environment.

### TEMP

Non relevant data, used while coding

```
# On docker
cd /fcc/tests/resources/write_a_c_compiler && ./test_compiler.sh /fcc/target/x86_64-unknown-linux-gnu/debug/fcc

# On mac 
./test_compiler.sh /Users/fedejinich/Projects/fcc/target/debug/fcc
```
