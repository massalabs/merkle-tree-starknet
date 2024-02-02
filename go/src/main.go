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

	// "github.com/itchyny/base58-go"
	"github.com/mr-tron/base58"
)

func main() {
	addition := C.add(34, 35)
	fmt.Println("Result from external addition of 35 and 34: ", addition)

	hello := C.CString("Hello")
	defer C.free(unsafe.Pointer(hello)) // schedule the release of the memory
	world := C.CString(" world!")
	defer C.free(unsafe.Pointer(world)) // schedule the release of the memory

	concatenated := C.concatenate_strings(hello, world)
	fmt.Println(C.GoString(concatenated))
	C.free_concatenated_string(concatenated)

	// a storage in memory
	storage := trie.NewTransactionStorage(db.NewMemTransaction(), nil)

	trie, _ := trie.NewTriePedersen(storage, 24)

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

	case C.RevertTo:
		panic("RevertTo not implemented")
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
	case C.GetProof:
		panic("GetProof not implemented")
	case C.VerifyProof:
		panic("VerifyProof not implemented")
	}
}

func get_arg1(command *C.Command) string {
	arg1 := C.GoString(command.arg1)
	decoded, err := base58.Decode(arg1)
	if err != nil {
		panic("error decoding")
	}
	return string(decoded)
}

func id(command *C.Command) int {
	switch command.id {
	case C.Commit:
		fallthrough
	case C.RevertTo:
		i, err := strconv.Atoi(get_arg1(command))
		if err != nil {
			panic("error converting")
		}
		return i
	default:
		panic("Command has no id")
	}
}

func value(command *C.Command) string {

	switch command.id {
	case C.Insert:
		fallthrough
	case C.Contains:
		fallthrough
	case C.Get:
		fallthrough
	case C.GetProof:
		arg2 := C.GoString(command.arg2)
		return string(arg2)
	case C.CheckRootHash:
		return get_arg1(command)

	default:
		panic("unknown command")

	}
}

func key(command *C.Command) []uint8 {
	switch command.id {
	case C.Remove:
		fallthrough
	case C.Insert:
		fallthrough
	case C.Contains:
		fallthrough
	case C.Get:
		fallthrough
	case C.GetProof:
		key := get_arg1(command)
		return []byte(key)

	default:
		panic("unknown command")

	}

}
