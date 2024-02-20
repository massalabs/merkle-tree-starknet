package go_common

// #cgo LDFLAGS: -L../rust_ffi
// #cgo LDFLAGS: -lrust_ffi
// #cgo CPPFLAGS: -I../rust_ffi
// #include "bindings.h"
// #include <stdlib.h>
// #include <string.h>
import "C"

import (
	"bytes"
	"encoding/hex"
	"errors"
	"flag"
	"fmt"
	"log"
	"os"
	"strings"
	"unsafe"
)

type Key []byte
type Value []byte

var ErrGet = errors.New("fail to get correct value")
var ErrRootHash = errors.New("fail to verify root_hash")
var ErrContains = errors.New("fail tree does not contain required key")

type TestEnvI interface {
	Insert(Key, Value)
	Remove(Key)
	Get(Key) Value
	Contains(Key) bool
	Check_root_hash() Value
}

func Init_runner(creator func() TestEnvI) {

	scenarioDir := flag.String("d", "", "directory to the scenario files")
	scenarioFile := flag.String("f", "", "scenario file")

	flag.Parse()

	if *scenarioDir != "" {
		// iterate over the files in the directory
		files, err := os.ReadDir(*scenarioDir)
		if err != nil {
			log.Fatal(err)
		}

		for _, file := range files {
			// skip non yml files
			if !strings.HasSuffix(file.Name(), ".yml") {
				continue
			}

			run_test(*scenarioDir+"/"+file.Name(), creator)
		}
	} else if *scenarioFile != "" {
		run_test(*scenarioFile, creator)
	} else {
		panic("no scenario file or directory provided")
	}
}

func hexstr_to_bytes(c_str string) []byte {
	if strings.HasPrefix(c_str, "0x") {
		c_str = c_str[2:]
	}
	if len(c_str)%2 != 0 {
		c_str = "0" + c_str
	}
	byteArr, err := hex.DecodeString(c_str)
	if err != nil {
		panic("error decoding hex string")
	}

	byteBuf := bytes.NewBuffer(byteArr)

	return byteBuf.Bytes()
}

func bytes_to_hexstr(byteArr []byte) string {
	hex_str := hex.EncodeToString(byteArr)
	// drop all leading 0
	for i, c := range hex_str {
		if c != '0' {
			return "0x" + hex_str[i:]
		}
	}
	return "0x0"
}

func value(command *C.Command) Value {
	switch command.id {
	case C.Insert, C.Get:
		val := (*C.char)(unsafe.Pointer(command.arg2.ptr))
		c_str := C.GoStringN(val, C.int(command.arg2.len-1))
		return hexstr_to_bytes(c_str)

	case C.Contains:
		val := (*C.char)(unsafe.Pointer(command.arg2.ptr))
		c_str := C.GoStringN(val, C.int(command.arg2.len-1))
		if c_str == "true" {
			return []byte{1}
		} else {
			return []byte{0}
		}

	case C.CheckRootHash:
		val := (*C.char)(unsafe.Pointer(command.arg1.ptr))
		c_str := C.GoStringN(val, C.int(command.arg1.len))
		return hexstr_to_bytes(c_str)

	default:
		panic("unknown command")
	}
}

func key(command *C.Command) Key {
	switch command.id {
	case C.Remove, C.Insert, C.Contains, C.Get:
		return C.GoBytes(unsafe.Pointer(command.arg1.ptr), C.int(command.arg1.len))

	default:
		panic("unknown command")
	}
}

func run_command(command *C.Command, tree *TestEnvI) error {
	switch command.id {
	case C.Insert:
		var k Key = key(command)
		var v Value = value(command)
		(*tree).Insert(Key(k), v)

	case C.Remove:
		var k Key = key(command)
		(*tree).Remove(k)

	case C.CheckRootHash:
		var v Value = value(command)
		v_str := bytes_to_hexstr(v)

		root := (*tree).Check_root_hash()
		root_str := bytes_to_hexstr(root)

		if root_str != v_str {
			fmt.Println("Fail to verify root_hash:", root_str, "\nwith expected:           ", v_str)
			return ErrRootHash
		}

	case C.Get:
		var k Key = key(command)
		var v Value = value(command)
		v_str := bytes_to_hexstr(v)

		val := (*tree).Get(k)
		val_str := bytes_to_hexstr(val)

		if val_str != v_str {
			fmt.Println("Fail, got:", val_str, "\nwhile expecting:", v_str)
			return ErrGet
		}

	case C.Contains:
		var k Key = key(command)
		var v Value = value(command)
		var v_bool bool = v[0] == 1

		val := (*tree).Contains(k)

		if !val {
			fmt.Println("Fail, to verify contains : ", val, " with expected : ", v_bool)
			return ErrContains
		}

	default:
		panic("unknown command")
	}
	return nil
}

func run_test(path string, creator func() TestEnvI) {
	tree := creator()

	path_p := C.CString(path)
	defer C.free(unsafe.Pointer(path_p))
	command_list := C.load_scenario(path_p)
	defer C.free_scenario(command_list)

	commands := unsafe.Slice(command_list.test_commands, command_list.len)
	for index := range commands {
		if run_command(&commands[index], &tree) != nil {
			fmt.Println("Test", path, "FAIL at command: ", index)
			return
		}
	}
	fmt.Println("Test", path, "SUCCESS")
}
