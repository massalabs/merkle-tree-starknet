#!/usr/bin/env bash

source "$(dirname "$0")/../scripts/common.sh"

parse_args $@

build_rust_ffi

# rebuild project
make all

if [ -n "$file_path" ]; then
    ./main -f"$file_path"
elif [ -n "$dir_path" ]; then
    ./main -d"$dir_path"
else
    ./main -h
fi