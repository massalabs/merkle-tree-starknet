#!/usr/bin/env bash

source "$(dirname "$0")/../scripts/common.sh"

parse_args $@

build_rust_ffi
# FIXME: we need to set GO111MODULE=off because the go program is not in a module
export GO111MODULE=off

# build go
go build main.go || exit 1

# run the program, we need to set the LD_LIBRARY_PATH to the rust_ffi so the program can find the shared library
export LD_LIBRARY_PATH="../rust_ffi"

if [ -n "$file_path" ]; then
    ./main -f "$file_path"
elif [ -n "$dir_path" ]; then
    ./main -d "$dir_path"
else
    ./main -h
fi
