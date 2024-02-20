#!/usr/bin/env bash

# is deno installed and executable?
if ! command -v deno &> /dev/null
then
    echo "deno could not be found, please install it"
    exit 1
fi

source "$(dirname "$0")/../scripts/common.sh"

parse_args $@

build_rust_ffi
if [ -n "$file_path" ]; then
    deno run --allow-read --allow-ffi --unstable-ffi main.ts -f "$file_path"
elif [ -n "$dir_path" ]; then
    deno run --allow-read --allow-ffi --unstable-ffi main.ts -d "$dir_path"
else
    deno run --allow-read --allow-ffi --unstable-ffi main.ts -h
fi
