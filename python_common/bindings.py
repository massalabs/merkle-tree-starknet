import ctypes
import platform

# form:
# typedef struct Bytes {
#   const uint8_t *ptr;
#   uintptr_t len;
# } Bytes;
class Bytes(ctypes.Structure):
    _fields_ = [
        ("ptr", ctypes.c_char_p),
        ("len", ctypes.c_size_t),
    ]


# form:
# typedef struct Command {
#   enum CommandId id;
#   struct Bytes arg1;
#   const char *arg2;
# } Command;
class Command(ctypes.Structure):
    _fields_ = [
        ("id", ctypes.c_int),
        ("arg1", Bytes),
        ("arg2", Bytes),
    ]

# form:
# typedef struct CommandList {
#   const struct Command *test_commands;
#   uintptr_t len;
# } CommandList;
class CommandList(ctypes.Structure):
    _fields_ = [
        ("test_commands", ctypes.POINTER(Command)),
        ("len", ctypes.c_size_t),
    ]


def load_rust_library() -> ctypes.CDLL:
    # Determine library extension based on the OS.
    lib_suffix = ""
    if platform.system() == "Windows":
        lib_suffix = "dll"
    elif platform.system() == "Darwin":
        lib_suffix = "dylib"
    else:
        lib_suffix = "so"

    lib_name = f"../rust_ffi/librust_ffi.{lib_suffix}"

    # Load the shared library
    rust_lib = ctypes.CDLL(lib_name)

    return rust_lib


lib: ctypes.CDLL | None = None


def get_lib() -> ctypes.CDLL:
    global lib
    if lib is None:
        lib = load_rust_library()
    return lib


def load_scenario(path: str) -> CommandList:
    get_lib().load_scenario.argtypes = [ctypes.c_char_p]
    get_lib().load_scenario.restype = CommandList
    return get_lib().load_scenario(path.encode("utf-8"))


def load_random() -> CommandList:
    get_lib().load_random.argtypes = []
    get_lib().load_random.restype = CommandList
    return get_lib().load_random()


def free_scenario(cmd: CommandList) -> None:
    get_lib().free_scenario.argtypes = [CommandList]
    get_lib().free_scenario(cmd)
