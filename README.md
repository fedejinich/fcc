# fcc (another C compiler)

Requirements
- ONLY works for `x86_64` architectures
- `gcc`

## Compile .c Files With fcc

```bash 
./fcc -s FILE_PATH/FILE_NAME.c
```

## Run Integration Tests 

### Mac OS (with M1)

Since this is only targetting x86 architecutres you'll need to run tests inside a docker container, to do so you'll first need to install a x86 compiler by runing:

```bash
brew tap messense/macos-cross-toolchains
brew install x86_64-unknown-linux-gnu
```

Then run the tests with this script

```bash
./fcc_test.sh
```

This will build the cargo project for x86_64 linux, then copies the project into the linux container and finally runs the tests in that environment.