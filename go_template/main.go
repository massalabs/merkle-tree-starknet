package main

// #cgo LDFLAGS: -L../rust_ffi
// #cgo LDFLAGS: -lrust_ffi
// #cgo CPPFLAGS: -I../rust_ffi
// #include "bindings.h"
// #include <stdlib.h>
import "C"
import (
	"../go_common"

	// import your dependencies here
)

type TestEnv struct {
	// wrap your implementation here
}

func (t TestEnv) Insert(key go_common.Key, value go_common.Value) {
	// call your implementation here
}

func (t TestEnv) Remove(key go_common.Key) {
	// call your implementation here
}

func (t TestEnv) Get(key go_common.Key) go_common.Value {
	// call your implementation here
	return go_common.Value{}
}

func (t TestEnv) Contains(key go_common.Key) bool {
	// call your implementation here
	return false
}

func (t TestEnv) Check_root_hash() go_common.Value {
	// call your implementation here
	return go_common.Value{}
}

func create_test_env() go_common.TestEnvI {
	// create your implementation here
	return TestEnv{}
}

func main() {
	go_common.Init_runner(create_test_env)
}
