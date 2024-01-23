# main.py

import ctypes
import os
import platform

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

# Define the result type and argument types of the function `add`
rust_lib.add.restype = ctypes.c_int
rust_lib.add.argtypes = [ctypes.c_int, ctypes.c_int]

# Call the function `add`
result = rust_lib.add(35, 34)  # 69

print(f"Result from external addition of 35 and 34: {result}")



# play with strings
rust_lib.concatenate_strings.restype = ctypes.c_char_p

# call and concat
s1 = b"Hello, "
s2 = b"world!"
result = rust_lib.concatenate_strings(s1, s2)

print(result.decode('utf-8'))

# free memory make the program crash, we suspect a conflict with python garbage collector
# rust_lib.free_concatenated_string(result)