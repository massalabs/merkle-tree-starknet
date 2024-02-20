#!/usr/bin/env bash

test_all_implementations=false
file_path=""
dir_path=""

while getopts ":a f: d:" opt; do
    case $opt in
    a)
        test_all_implementations=true
        ;;
    f)
        file_path="$(realpath "$OPTARG")"
        ;;
    d)
        dir_path="$(realpath "$OPTARG")"
        ;;
    \?)
        echo "Invalid option: -$OPTARG" >&2
        exit 1
        ;;
    :)
        echo "Option -$OPTARG requires an argument." >&2
        exit 1
        ;;
    esac
done
shift $((OPTIND - 1))

if [ -n "$file_path" ] && [ -n "$dir_path" ]; then
    echo "Error: options -f and -d are exclusive and cannot be used together."
    exit 1
fi

if $test_all_implementations; then
    implementations=("c" "go_juno" "python_cairo" "rust_bonsai" "rust_pathfinder" "ts" "zig")
elif [ $# -eq 0 ]; then
    echo "Usage: $0 [-a] [-f file_path] [-d dir_path] [implementation1 implementation2 ...]"
    echo ""
    echo "Runs tests for the specified implementations."
    echo "When the -a option is used, all the implementations are tested."
    echo "When the -f option is used, the specified path is used as input for the tests."
    echo "When the -d option is used, all .yml files in the specified path are used as input for the tests."
    echo "Options -f and -d are exclusive and cannot be used together."
    exit 1
else
    implementations=("$@")
fi

for implementation in "${implementations[@]}"; do
    echo ""
    echo "##################################################"
    echo "Running tests for $implementation"
    echo "##################################################"
    pushd $implementation
    if [ -n "$file_path" ]; then
        ./run.sh -f "$file_path"
    elif [ -n "$dir_path" ]; then
        ./run.sh -d "$dir_path"
    else
        ./run.sh
    fi
    popd
done
