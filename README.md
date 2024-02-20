# Starknet Merkle Tree test framework

## Testing an implementation

### Where to start
To test an implementation using this test framework, two main options are available:
1. clone the whole repository
2. download the package corresponding to the triplet `language.arch.os` from [the releases page](https://github.com/massalabs/merkle-tree-starknet/releases/)

__Option 1__: is more flexible, gives access to more examples but requires installing more compilation environments, at least the Rust one.

__Option 2__: is easier to use, but only includes the package corresponding to your `language.arch.os`.

Whatever option is taken, aside from the required dependencies, one gets a folder named `<lang>_template`. This is the sole place where the work should happen (this folder may be copied/renamed before starting to work).

### What to do
In any supported language, the work to do decay to:
- wrap you Merkle Tree implementation
- implement a defined interface (insert, remove, get_root_hash…) and a factory to instantiate the wrapped implementation.
- build [if need] and run the result with a scenario as argument.

The provided `<lang>_pkg/<lang>_template` folder contains a `main.<lang>` file that does exactly that.


In any case, a helper script (`run.sh`) is provided.
It takes care of rebuilding before running if required.
_When using a prebuild package, dependencies build must be disabled._

Some scenarios are provided in the folder `scenarios`.

Common run options are:
- `-f <filename>`: execute the scenario given in the given file.
- `-d <dirname>`: execute all the scenarios in the given directory.

Special run options are:
- `-r <DIR_PATH><NUMBER>`: generate NUMBER random scenarios and store them in the given directory.
- `--fix`: run the scenario in a way that the expected values are corrected, this is useful to fix root hashes for random scenarios. The corrected scenario is saved alongside the original one.

Special run options are only available when using the python_cairo reference implementation.

## Languages specific considerations
### C
Na

### Golang
Tuning off `GO111MODULE` may be required.
```bash
export GO111MODULE=off
```

### Python
Na

### Rust
Na

### Typescript
Uses `deno` to take advantage of its `ffi` capabilities.
Tested version is `deno 1.41.2 `

### Zig
Minimal version required is `0.12.0`. (as of end of March 2024 it has to be released soon).