import {
  parseArgs,
  type ParseOptions,
} from "https://deno.land/std/cli/parse_args.ts";
import console from "node:console";

import * as fs from "node:fs";
import * as path from "node:path";

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

// typedef enum CommandId {
//   Insert,
//   Remove,
//   CheckRootHash,
//   Get,
//   Contains,
// } CommandId;
enum CommandId {
  Insert,
  Remove,
  CheckRootHash,
  Get,
  Contains,
}

// typedef struct Bytes {
//   const uint8_t *ptr;
//   uintptr_t len;
// } Bytes;

class Bytes {
  ptr: ArrayBuffer;
  len: bigint;
  static array_wrapper_size = 16; // ptr (8) + len (8)

  constructor(raw: ArrayBuffer) {
    const view = new DataView(raw);
    this.len = view.getBigUint64(8, true);
    const ptr = Deno.UnsafePointer.create<ArrayBuffer>(
      view.getBigUint64(0, true),
    );
    if (ptr === null) {
      throw new Error("Failed to create pointer");
    }
    this.ptr = Deno.UnsafePointerView.getArrayBuffer(ptr, Number(this.len), 0);
  }
}

// typedef struct Command {
//   enum CommandId id;
//   struct Bytes arg1;
//   struct Bytes arg2;
// } Command;
class Command {
  id: CommandId;
  arg1: Bytes;
  arg2: Bytes;
  static command_size = 40; // id (4) + padding (4) + arg1 (16) + arg2 (16)

  constructor(id: number, arg1: Bytes, arg2: Bytes) {
    this.id = id;
    this.arg1 = arg1;
    this.arg2 = arg2;
  }
}

// typedef struct CommandList {
//   const struct Command *test_commands;
//   uintptr_t len;
// } CommandList;
class Commands {
  ptr: Deno.PointerObject<Command>;
  len: bigint;
  static commands_size = 16; // data (8) + len (8)

  constructor(raw: Uint8Array) {
    const view = new DataView(raw.buffer);
    const ptr = Deno.UnsafePointer.create<Command>(view.getBigUint64(0, true));
    if (ptr === null) {
      throw new Error("Failed to create pointer");
    }
    this.ptr = ptr;
    this.len = view.getBigUint64(8, true);
  }

  getCommand(i: number): Command {
    if (i < 0 || i >= this.len) {
      throw new Error("Index out of bounds");
    }

    const command = Deno.UnsafePointerView.getArrayBuffer(
      this.ptr,
      Command.command_size,
      i * Command.command_size,
    );
    let offset = 0;

    const command_id = new DataView(command).getUint32(offset, true);
    offset += 8; // sizeof(id)==4 + padding==4

    const cmd_arg1 = new Bytes(
      Deno.UnsafePointerView.getArrayBuffer(
        this.ptr,
        Bytes.array_wrapper_size,
        i * Command.command_size + offset,
      ),
    );
    offset += Bytes.array_wrapper_size;

    const cmd_arg2 = new Bytes(
      Deno.UnsafePointerView.getArrayBuffer(
        this.ptr,
        Bytes.array_wrapper_size,
        i * Command.command_size + offset,
      ),
    );

    return new Command(command_id, cmd_arg1, cmd_arg2);
  }
}

const CommandList = ["pointer", "usize"];

const libName = `../rust_ffi/librust_ffi.${libSuffix}`;

// Open library and define exported symbols
const dylib = Deno.dlopen(
  libName,
  {
    load_scenario: { parameters: ["buffer"], result: { struct: CommandList } },
    free_scenario: { parameters: [{ struct: CommandList }], result: "void" },
  } as const,
);

export type Key = Uint8Array;
export type Value = Uint8Array;

/**
 * Represents a test environment for key-value operations.
 */
export interface ITestEnv {
  /**
   * Inserts a key-value pair into the test environment.
   * @param k The key to insert.
   * @param v The value to insert.
   */
  insert(k: Key, v: Value): void;

  /**
   * Removes a key from the test environment.
   * @param k The key to remove.
   */
  remove(k: Key): void;

  /**
   * Retrieves the value associated with a key from the test environment.
   * @param k The key to retrieve the value for.
   * @returns The value associated with the key.
   */
  get(k: Key): Value;

  /**
   * Checks if a key exists in the test environment.
   * @param k The key to check.
   * @returns `true` if the key exists, `false` otherwise.
   */
  contains(k: Key): boolean;

  /**
   * Checks the root hash of the test environment.
   * @returns The root hash value.
   */
  check_root_hash(): Value;
}

/**
 * Initializes the runner based on the provided creator function.
 * @param creator - A function that creates an instance of `ITestEnv`.
 */
export function init_runner(creator: () => ITestEnv): void {
  const args = parseArgs(Deno.args, {
    alias: {
      "help": "h",
      "dir": "d",
      "file": "f",
    },
    string: ["dir", "file"],
    boolean: ["help"],
  });

  if (args.help || !(args.dir || args.file) || (args.dir && args.file)) {
    console.log(
      "Usage: deno run --allow-read --allow-ffi --unstable-ffi main.ts --dir <scenarioDir> | --file <scenarioFile>",
    );
    return;
  }

  if (args.dir !== undefined) {
    readScenarioDirectory(args.dir, creator);
  } else if (args.file !== undefined) {
    runTest(args.file, creator);
  } else {
    throw new Error("No scenario file or directory provided");
  }
}

/**
 * Reads the scenario directory and runs tests for each YAML file found.
 *
 * @param scenarioDir - The path to the scenario directory.
 * @param creator - A function that creates an instance of the test environment.
 */
function readScenarioDirectory(scenarioDir: string, creator: () => ITestEnv) {
  fs.readdir(scenarioDir, (err, files: string[]) => {
    if (err) {
      console.error(err);
      return;
    }

    files.forEach((file) => {
      if (!file.endsWith(".yml")) {
        return;
      }
      const filePath = path.join(scenarioDir, file);
      runTest(filePath, creator);
    });
  });
}

/**
 * Runs a test scenario.
 *
 * @param scenarioPath - The path to the scenario file.
 * @param creator - A function that creates the test environment.
 * @throws {Error} If the library fails to open, load a symbol, or free a test.
 */
function runTest(scenarioPath: string, creator: () => ITestEnv) {
  console.log(`Running test scenario: ${scenarioPath}`);
  if (dylib == null) {
    throw new Error("Failed to open library");
  }
  if (dylib.symbols == null) {
    throw new Error("Failed to load symbol");
  }
  if (dylib.symbols.load_scenario == null) {
    throw new Error("Failed to load symbol");
  }
  if (dylib.symbols.free_scenario == null) {
    throw new Error("Failed to load symbol");
  }

  const test_env = creator();
  let buf: Uint8Array = new TextEncoder().encode(scenarioPath);
  // make sure the buffer is null-terminated
  buf = new Uint8Array([...buf, 0]);

  const cmd_list = dylib.symbols.load_scenario(buf);
  const commands = new Commands(cmd_list);

  for (let i = 0; i < commands.len; i++) {
    const command = commands.getCommand(i);
    try {
      run_command(command, test_env);
    } catch (err) {
      console.log(`Test ${scenarioPath} failed at command ${i}: ${err}`);
      break;
    }
  }

  dylib.symbols.free_scenario(cmd_list);
}

/**
 * Executes the specified command on the test environment.
 *
 * @param command - The command to execute.
 * @param test_env - The test environment.
 * @throws {Error} If the command is unknown.
 */
function run_command(command: Command, test_env: ITestEnv) {
  switch (command.id) {
    case CommandId.Insert: {
      const key = new Uint8Array(command.arg1.ptr);
      const value = new Uint8Array(command.arg2.ptr);

      test_env.insert(key, value);
      break;
    }

    case CommandId.Remove:
      test_env.remove(new Uint8Array(command.arg1.ptr));
      break;

    case CommandId.Get: {
      const value = new Uint8Array(command.arg1.ptr);

      const get = test_env.get(new Uint8Array(command.arg1.ptr));

      if (get != value) {
        throw new Error("Get wrong value");
      }

      break;
    }

    case CommandId.Contains: {
      const key = new Uint8Array(command.arg1.ptr);
      const value = new Uint8Array(command.arg2.ptr);

      const contains = test_env.contains(key);

      if (value.toString() === "true") {
        if (!contains) {
          throw new Error("Contains wrong value");
        }
      } else {
        if (contains) {
          throw new Error("Contains wrong value");
        }
      }

      break;
    }

    case CommandId.CheckRootHash: {
      const value = new Uint8Array(command.arg1.ptr);

      const root = test_env.check_root_hash();

      if (root != value) {
        throw new Error("CheckRootHash wrong value");
      }

      break;
    }
    default:
      throw new Error("Unknown command");
  }
}
