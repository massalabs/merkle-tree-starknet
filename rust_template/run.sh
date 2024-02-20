#!/usr/bin/env bash

source "$(dirname "$0")/../scripts/common.sh"

parse_args $@

if [ -n "$file_path" ]; then
    cargo run -- -f "$file_path"
elif [ -n "$dir_path" ]; then
    cargo run -- -d "$dir_path"
else
    cargo run -- -h
fi
