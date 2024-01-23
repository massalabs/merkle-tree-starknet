// ffi.ts

// Determine library extension based on
// your OS.
let libSuffix = "";
switch (Deno.build.os) {
  case "windows":
    libSuffix = "dll";
    break;
  case "darwin":
    libSuffix = "dylib";
    break;
  default:
    libSuffix = "so";
    break;
}

const libName = `../rust_ffi/target/debug/librust_ffi.${libSuffix}`;
// Open library and define exported symbols
const dylib = Deno.dlopen(libName, {
  add: { parameters: ["isize", "isize"], result: "isize" },
  concatenate_strings: { parameters: ["buffer", "buffer"], result: "buffer" },
  print_string: { parameters: ["buffer"], result: "void" },
  get_string: { parameters: [], result: "buffer" },
  ffi_string: { parameters: [], result: "pointer" },
} as const);

// import from the implementation in TS that has to be tested
import {
  operand_35,
  operand_34,
  get_hello,
  get_world,
} from "../node/src/index.ts";

// Call the symbol `add`
const result = dylib.symbols.add(operand_35(), operand_34()); // 69
console.log(`Result from external addition of 35 and 34: ${result}`);

const buf1: Uint8Array = new TextEncoder().encode(get_hello());
const buf2: Uint8Array = new TextEncoder().encode(get_world());

dylib.symbols.print_string(buf1);

console.log("##### ffi_string");
const static_str = Deno.UnsafePointerView.getCString(
  dylib.symbols.ffi_string()
);
console.log("pointer from static rust str:", static_str);

console.log("##### get_string");
// MANDATORY use Deno.UnsafePointerView.getCString() to get the string
// WARNING the memory is leaked from Rust
const str = Deno.UnsafePointerView.getCString(dylib.symbols.get_string());
console.log("buffer from dynamic rust string:", str);

console.log("##### concatenate_strings");
// MANDATORY use Deno.UnsafePointerView.getCString() to get the string
// WARNING the memory is leaked from Rust
const concatenate_strings = Deno.UnsafePointerView.getCString(
  dylib.symbols.concatenate_strings(buf1, buf2)
);
console.log("buffer from concatenated string:", concatenate_strings);
