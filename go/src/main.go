package main

// #cgo LDFLAGS: -L../../rust/rust_ffi/target/debug
// #cgo LDFLAGS: -lrust_ffi
// #cgo CPPFLAGS: -I../../rust/rust_ffi
// #include "bindings.h"
// #include <stdlib.h>
import "C"
import (
	"fmt"
	"strconv"
	"strings"
	"unsafe"

	"github.com/NethermindEth/juno/core/felt"
	"github.com/NethermindEth/juno/core/trie"
	"github.com/NethermindEth/juno/db"
)

func main() {
	// a storage in memory
	storage := trie.NewTransactionStorage(db.NewMemTransaction(), nil)

	trie, _ := trie.NewTriePedersen(storage, 251)

	//get the test commands scenario

	path := C.CString("/home/jf/workspace/rust/starknet/merkle-tree-starknet/scenario/3.yml")
	defer C.free(unsafe.Pointer(path))
	command_list := C.load_scenario(path)

	run_test(&command_list, trie)
	defer C.free_test(command_list)
}

func run_test(command_list *C.CommandList, tree *trie.Trie) {
	commands := unsafe.Slice(command_list.test_commands, command_list.len)
	for index, _ := range commands {
		run_command(&commands[index], tree)
	}
}

func run_command(command *C.Command, tree *trie.Trie) {
	switch command.id {
	case C.Insert:
		key := key(command)
		value := value(command)
		fmt.Println("Exec insert with key :", key, " and value :", value)
		val, _ := new(felt.Felt).SetString(value)
		tree.Put(new(felt.Felt).SetBytes(key), val)

	case C.Remove:
		fmt.Println("remove")
		key := key(command)
		fmt.Println("Exec Remove with key :", key)
		tree.Put(new(felt.Felt).SetBytes(key), nil)
	case C.Commit:
		// id := id(command)
		fmt.Println("Exec Commit")
		tree.Commit()
	case C.CheckRootHash:
		expected := value(command)
		if strings.HasPrefix(expected, "0x") {
			expected = expected[2:]
		}

		fmt.Println("Exec CheckRootHash with hash :", expected)
		root, error := tree.Root()
		if error != nil {
			panic("error check root hash")
		}

		root_hash := root.Text(16)
		if root_hash != expected {
			fmt.Println("Fail to verify root_hash : ", root_hash, " with expected : ", expected)
			panic("test doesn't match")
		}

	// case C.RevertTo:
	// 	panic("RevertTo not implemented")
	case C.Get:
		key := key(command)
		value := value(command)

		val, error := tree.Get(new(felt.Felt).SetBytes(key))
		if error != nil {
			panic("Get : error getting value")
		}
		fmt.Println("Exec Get on key: ", key, " with value", val)
		if val.Text(16) != value {
			panic("Get : value doesn't match")
		}
	case C.Contains:
		panic("Contains not implemented")
	}
}

func value(command *C.Command) string {
	switch command.id {
	case C.Insert, C.Contains, C.Get:
		return string(C.GoString(command.arg2))
	case C.CheckRootHash:
		return string(unsafe.Slice(command.arg1.ptr, command.arg1.len))
	default:
		panic("unknown command")
	}
}

func key(command *C.Command) []uint8 {
	switch command.id {
	case C.Remove, C.Insert, C.Contains, C.Get:
		arr := unsafe.Slice((*uint8)(command.arg1.ptr), command.arg1.len)
		// convertedArr := make([]uint8, len(arr))
		// for i, v := range arr {
		// 	convertedArr[i] = uint8(v)
		// }
		// return convertedArr
		return arr

	default:
		panic("unknown command")
	}
}

