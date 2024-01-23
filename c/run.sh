pushd .

# build rust_ffi
cd ../rust_ffi && cargo build && popd

# build our test project
make main

# run our ts test
# need to have librust_ffi.so in LD_LIBRARY_PATH, set it temporarily here
LD_LIBRARY_PATH="../rust_ffi/target/debug/" ./main > result.txt

# diff the result
diff result.txt expected.txt

# if error print it
if [ $? -ne 0 ]; then
    echo "Error: result.txt and expected.txt are not the same"
    exit 1
else
    echo "You're the boss test passed"
    exit 0
fi