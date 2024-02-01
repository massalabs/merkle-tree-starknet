package main

// #cgo LDFLAGS: -L../../rust/rust_ffi/target/debug
// #cgo LDFLAGS: -lrust_ffi
// #cgo CPPFLAGS: -I../../rust/rust_ffi
// #include "bindings.h"
// #include <stdlib.h>
import "C"
import (
	"fmt"
	"unsafe"
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
}
