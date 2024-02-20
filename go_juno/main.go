package main

// #cgo LDFLAGS: -L../rust_ffi
// #cgo LDFLAGS: -lrust_ffi
// #cgo CPPFLAGS: -I../rust_ffi
// #include "bindings.h"
// #include <stdlib.h>
import "C"
import (
	"../go_common"
	"github.com/NethermindEth/juno/core/felt"
	"github.com/NethermindEth/juno/core/trie"
	"github.com/NethermindEth/juno/db"
)

type TestEnv struct {
	tree    *trie.Trie
	storage *trie.TransactionStorage
}

func felt_to_bytes(val *felt.Felt) []byte {
	arr := val.Bytes()
	slice := make([]byte, len(arr))
	copy(slice, arr[:])
	return slice
}

func (t TestEnv) Insert(key go_common.Key, value go_common.Value) {
	k := new(felt.Felt).SetBytes(key)
	v := new(felt.Felt).SetBytes(value)

	t.tree.Put(k, v)
}

func (t TestEnv) Remove(key go_common.Key) {
	t.tree.Put(new(felt.Felt).SetBytes(key), &felt.Zero)

}

func (t TestEnv) Get(key go_common.Key) go_common.Value {
	val, error := t.tree.Get(new(felt.Felt).SetBytes(key))
	if error != nil {
		panic("Get : error getting value")
	}

	return felt_to_bytes(val)
}

func (t TestEnv) Contains(key go_common.Key) bool {
	return t.Get(key) != nil
}

func (t TestEnv) Check_root_hash() go_common.Value {
	root, err := t.tree.Root()
	if err != nil {
		panic(err)
	}

	return felt_to_bytes(root)
}

func create_test_env() go_common.TestEnvI {
	storage := trie.NewTransactionStorage(db.NewMemTransaction(), nil)
	tree, _ := trie.NewTriePedersen(storage, 251)
	return TestEnv{tree, storage}
}

func main() {
	go_common.Init_runner(create_test_env)

}
