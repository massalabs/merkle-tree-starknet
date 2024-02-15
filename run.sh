#!/usr/bin/env bash

if [ $# -eq 0 ]; then
    tests=("c" "go" "python" "rust_bosai" "rust_pathfinder" "typescript" "zig")
else
    tests=("$@")
fi

for test in "${tests[@]}"; do
    echo "Running $test tests"
    pushd $test
    ./run.sh
    if [ $? -ne 0 ]; then
        echo "Error: failed to run $test tests"
        exit 1
    else
        echo "Success: $test tests ran successfully"
    fi
    popd
done
