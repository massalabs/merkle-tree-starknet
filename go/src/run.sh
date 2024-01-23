#!/bin/bash

pushd .

# build rust_ffi
cd ../../rust/rust_ffi
cargo build || exit 1
popd

# build go
pushd .
go build || exit 1

# run the program, we need to set the LD_LIBRARY_PATH to the rust_ffi/target/debug so the program can find the shared library
LD_LIBRARY_PATH="../../rust/rust_ffi/target/debug" ./main
