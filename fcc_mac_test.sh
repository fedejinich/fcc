# setup env variables for x86_64 target
export CC_x86_64_unknown_linux_gnu=x86_64-unknown-linux-gnu-gcc
export CXX_x86_64_unknown_linux_gnu=x86_64-unknown-linux-gnu-g++
export AR_x86_64_unknown_linux_gnu=x86_64-unknown-linux-gnu-ar
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-unknown-linux-gnu-gcc

# build for x86_64-unknown-linux-gnu
cargo clean && cargo build --target x86_64-unknown-linux-gnu

# run x86_64 build in an x86_64 docker container
docker build -t fcc . && docker run --rm -it --platform linux/amd64 fcc