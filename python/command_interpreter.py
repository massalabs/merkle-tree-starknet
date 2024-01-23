import ctypes


class CommandInterpreter:
    def __init__(self, ffi_lib: ctypes.CDLL):
        self.ffi_lib = ffi_lib

    def run_command(self, command, tree):
        pass
