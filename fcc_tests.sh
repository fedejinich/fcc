#!/bin/bash

cargo clean && cargo build && cd tests/resources/write_a_c_compiler && ./test_compiler.sh /Users/fedejinich/Projects/fcc/target/debug/fcc 1
