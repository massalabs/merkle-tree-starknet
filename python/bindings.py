import ctypes
import platform


# typedef struct ArrayWrapper {
#   const uint8_t *ptr;
#   uintptr_t len;
# } ArrayWrapper;
class ArrayWrapper(ctypes.Structure):
    _fields_ = [
        ("ptr", ctypes.c_char_p),
        ("len", ctypes.c_size_t),
    ]


# typedef struct Command {
#   enum CommandId id;
#   struct ArrayWrapper arg1;
#   const char *arg2;
# } Command;
class Command(ctypes.Structure):
    _fields_ = [
        ("id", ctypes.c_int),
        ("arg1", ArrayWrapper),
        ("arg2", ctypes.c_char_p),
    ]

# typedef struct CommandList {
#   const struct Command *test_commands;
#   uintptr_t len;
# } CommandList;
class CommandList(ctypes.Structure):
    _fields_ = [
        ("test_commands", ctypes.POINTER(Command)),
        ("len", ctypes.c_size_t),
    ]


# void free_test(struct CommandList cmd);



def load_rust_library() -> ctypes.CDLL:
    # Determine library extension based on your OS.
    lib_suffix = ""
    if platform.system() == "Windows":
        lib_suffix = "dll"
    elif platform.system() == "Darwin":
        lib_suffix = "dylib"
    else:
        lib_suffix = "so"

    lib_name = f"../rust_ffi/target/debug/librust_ffi.{lib_suffix}"

    # Load the shared library
    rust_lib = ctypes.CDLL(lib_name)

    return rust_lib

