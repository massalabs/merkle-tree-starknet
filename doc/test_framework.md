# Overview
This projet provides a validation framework for various implementations of PMT in various languages.
The primary targeted languages are:
- C,
- go,
- rust,
- typescript,
- python,
- zig.

The solution is extensible to other languages.

## Definitions
Validating an implementation of a PMT involves three actors:
1. The PMT implementation to be validated: *Implementation* (for short),
2. The validation framework: *Framework*,
3. A test runner implemented in the same language as the PMT implementation to be tested: *Runner*.

### *Implementation*
- Responsibility:
State of the art implementation of the PMT in the targeted language.

### *Framework*
- Responsibility:
Offer an uniform way to execute operations over an *Implementation* and validate its states.

### *Runners*
- Responsibility:
Glues the *Framework* and the *Implementation*. Its role is to request test instructions from the *Framework*, call the *Implementation* accordingly then pass the result back to the *Framework*.


# Technical considerations
### *Implementation*
By definition 
 It's API is already defined by definition.
### *Framework*
This is the main deliverable.
It is delivered:
- as a rust library that exposes its interfaces with the `C` ABI.
- TBC: it is also delivered as a `wasm` file.

### *Runners* and bindings
For *Runners* to call the *Framework* we also provide thin bindings to C for go, python and zig.
The Rust *Implementation* can obviously go the native way.
The TypeScript *Implementation* is a special case where the `wasm` interface is envisioned. We might consider https://docs.deno.com/runtime/manual/runtime/ffi_api and go the `C` way everywhere.




## Tests workflow
It's implemented by the *Runners*
*Runners* implementations are implemented in the same language as the *Implementation*. That leaves room for more flexibility.

*Runners* take the form of this pseudo code:
```rs
pub fn run_test(implementation, framework, test_case) {
    commands = framework.get_commands(test_case);
    for command in commands {
        run_command(command, implementation);
    }
}

pub fn run_all_test(framework) {
    tests = framework.get_all_tests();
    for test in tests {
        run_test(implementation, framework, test)
    }
}
```


```go
func run_command(command *C.Command, tree *trie.Trie) {
	/// TODO : implement each command who can be executed on the trie
	switch command.id {
	case C.Insert:
		panic("Insert not implemented")
	case C.Remove:
		panic("Remove not implemented")
	case C.Commit:
		panic("Commit not implemented")
	case C.CheckRootHash:
		panic("CheckRootHash not implemented")
	case C.RevertTo:
		panic("RevertTo not implemented")
	case C.Get:
		panic("Get not implemented")
	case C.Contains:
		panic("Contains not implemented")
	case C.GetProof:
		panic("GetProof not implemented")
	case C.VerifyProof:
		panic("VerifyProof not implemented")
	}
}
```



## If needed write your runner
## Implement the interface / trait that glued your implementation with the test framework
