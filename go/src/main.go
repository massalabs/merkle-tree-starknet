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

	// fmt.Println("trie: ", trie)

	// bytes_key := new(felt.Felt).SetBytes([]byte{0x01, 0x01, 0x01})
	// val1, _ := new(felt.Felt).SetString("0x66342762FDD54D033c195fec3ce2568b62052e")
	// _, error_add := trie.Put(bytes_key, val1)
	// if error_add != nil {
	// 	fmt.Println("error_add: ", error_add)
	// }
	// error := trie.Commit()
	// if error != nil {
	// 	fmt.Println("error: ", error)
	// }

	// check_root(trie, "4dff6cff9ce781e7700c6f43fd1e944736a9144ca946a280afe5fafea4e44cc")
	// fmt.Println(trie)

	// bytes_key2 := new(felt.Felt).SetBytes([]byte{0x01, 0x01, 0x02})
	// val2, _ := new(felt.Felt).SetString("0x66342762FDD54D033c195fec3ce2568b62052e")
	// trie.Put(bytes_key2, val2)
	// trie.Commit()
	// check_root(trie, "21c57215657fa4a475a54f99ad8571836dec23cd724ca25650dfdc7b554981b")

	// bytes_key3 := new(felt.Felt).SetBytes([]byte{0x01, 0x01, 0x03})
	// val3, _ := new(felt.Felt).SetString("0x66342762FDD54D033c195fec3ce2568b62052e")
	// trie.Put(bytes_key3, val3)
	// trie.Commit()
	// check_root(trie, "67fa4710f37a846c966d087db635a6be4d22451e1ba774f3f6f52829228eeb1")

	// trie.Dump()

	// trie.Put(new(felt.Felt).SetUint64(1), val1)
	// trie.Commit()
	// fmt.Println(trie.Root())

	// trie.Put(new(felt.Felt).SetUint64(2), val2)
	// trie.Commit()
	// fmt.Println(trie.Root())

	// fmt.Println("rootkey :" , trie.RootKey())
	// trie.Dump()

	command_list := C.get_test3()
	run_test(&command_list, trie)
	defer C.free_test(command_list)

}

func check_root(trie *trie.Trie, expected string) {
	root, error := trie.Root()
	if error != nil {
		panic("error root")
	}

	root_hash := root.Text(16)
	if root_hash == expected {
		fmt.Println("root OK :", expected)
	} else {
		fmt.Println("root hash: ", root_hash)
		fmt.Println("ref  hash: ", expected)
		panic("test doesn't match")
	}
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
		// fmt.Println("insert")
		key := key(command)
		// fmt.Println("key: ", key)
		value := value(command)
		// fmt.Println("value: ", value)
		val, _ := new(felt.Felt).SetString(value)
		tree.Put(new(felt.Felt).SetBytes(key), val)

	case C.Remove:
		fmt.Println("remove")
		key := key(command)
		tree.Put(new(felt.Felt).SetBytes(key), nil)

	case C.Commit:
		fmt.Println("commit")
		id := id(command)
		fmt.Println("id: ", id)
		tree.Commit()

	case C.CheckRootHash:
		// fmt.Println("check_root_hash")
		root_hash := value(command)
		if strings.HasPrefix(root_hash, "0x") {
			root_hash = root_hash[2:]
		}
		// fmt.Println("expected hash root: ", root_hash)
		check_root(tree, root_hash)

	case C.RevertTo:
		// fmt.Println("revert_to")
		id := command.id
		fmt.Println("id:", id)

	case C.Get:
		fmt.Println("get")
		key := key(command)
		fmt.Println("key: ", key)

		// key := command.key
		// value := command.value
		// fmt.Printf("get %v %s\n", key, value)
		// res := bonsaiStorage.get(key)
		// assert(res == value)
	case C.Contains:
		fmt.Println("contains")
		key := key(command)
		fmt.Println("key: ", key)
		// key := command.key
		// value := command.value
		// fmt.Printf("contains %v %s\n", key, value)
		// res := bonsaiStorage.contains(key)
		// assert(res == value)
	case C.GetProof:
		fmt.Println("get_proof")
		key := key(command)
		fmt.Println("key: ", key)
		// key := command.key
		// value := command.value
		// fmt.Printf("get_proof %v %s\n", key, value)
		// proof := bonsaiStorage.getProof(key)
		// assert(proof == value)
	case C.VerifyProof:

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
		fmt.Println("key: ", key)
		return []byte(key)

	default:
		panic("unknown command")

	}

}
