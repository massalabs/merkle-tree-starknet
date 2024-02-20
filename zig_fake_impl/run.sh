#!/usr/bin/env bash

source "$(dirname "$0")/../scripts/common.sh"

parse_args $@

build_rust_ffi
# build zig
zig build || exit 1

if [ -n "$file_path" ]; then
    ./zig-out/bin/main -f"$file_path"
elif [ -n "$dir_path" ]; then
    ./zig-out/bin/main -d"$dir_path"
else
    ./zig-out/bin/main -h
fi