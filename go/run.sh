pushd .

# build rust_ffi
cd ../rust/rust_ffi && cargo build --release && popd

# build go
pushd .
cd src
go build

# run the program, we need to set the LD_LIBRARY_PATH to the rust_ffi/target/debug so the program can find the shared library
LD_LIBRARY_PATH="../../rust/rust_ffi/target/debug" ./main >result.txt

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
