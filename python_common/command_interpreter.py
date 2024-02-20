import ctypes
from typing import Callable, Dict, Iterable


from enum import IntEnum

from icecream import ic
from . import bindings, test_tree


# def save


# typedef enum CommandId {
#   Insert,
#   Remove,
#   CheckRootHash,
#   Get,
#   Contains,
# } CommandId;
#
# Map the Rust/C enum to a Python Enum.
# /!\ this has to be done manually as C enums does not seems to map directly to Python enums.
class CommandId(IntEnum):
    Insert = 0
    Remove = 1
    CheckRootHash = 2
    Get = 3
    Contains = 4

    tree: test_tree.ITestTree


class CommandInterpreter:
    tree: test_tree.ITestTree
    fix_path: str | None
    debug: bool

    @classmethod
    async def create(
        cls,
        test_env_creator: Callable[[], test_tree.ITestTree],
        fix_path: str | None,
        debug: bool,
    ) -> "CommandInterpreter":
        self = CommandInterpreter()
        self.fix_path = fix_path
        # if fix_path mode, clear the file
        if self.fix_path:
            with open(self.fix_path, "w") as f:
                f.write("# yaml-language-server: $schema=./schema.json\n\n")

        self.tree = await test_env_creator()
        self.debug = debug
        return self

    async def run_command(self, command: bindings.Command) -> None:
        match command.id:
            case CommandId.Insert:
                await self.command_insert(command)

            case CommandId.Remove:
                await self.command_remove(command)

            case CommandId.CheckRootHash:
                await self.command_check_root_hash(command)

            case CommandId.Get:
                await self.command_get(command)

            case CommandId.Contains:
                await self.command_contains(command)

            case _:
                print("Unknown command in run command")

    def fix(self, cmd: str, fields: Dict[str, list[int] | str | bool | None]) -> None:
        if self.fix_path:
            with open(self.fix_path, "a") as f:
                f.write(f"- {cmd}:\n")
                for key, val in fields.items():
                    f.write(f"    {key}: ")
                    if isinstance(val, str):
                        f.write(f'"{val}"\n')
                    elif isinstance(val, bool):
                        f.write(f"{str(val).lower()}\n")
                    else:
                        f.write(f"{val}\n")

                f.write(f"\n")

    def dbg(self, cmd: str, fields: Dict[str, list[int] | str | bool | None]) -> None:
        print(f"{cmd}:")
        for key, val in fields.items():
            print(f"    {key}: ", end="")
            if isinstance(val, str):
                print(f'"{val}"')
            elif isinstance(val, bool):
                print(f"{str(val).lower()}")
            else:
                print(f"{val}".replace(" ", ""))

    async def command_insert(self, command: bindings.Command) -> None:
        key: bytes = get_key(command)
        value: bytes = int(get_value(command).decode("utf-8"), 0).to_bytes(32, "big")
        val_int_str = hex(int.from_bytes(value))

        if self.fix_path:
            self.fix("insert", {"key": to_key(key), "value": val_int_str})

        if self.debug:
            self.dbg("insert", {"key": to_key(key), "value": val_int_str})

        await self.tree.insert(key=key, value=value)

    async def command_remove(self, command: bindings.Command) -> None:
        key_remove: bytes = get_key(command)

        if self.fix_path:
            self.fix("remove", {"key": to_key(key_remove)})

        if self.debug:
            self.dbg("remove", {"key": to_key(key_remove)})

        await self.tree.remove(key_remove)

    async def command_check_root_hash(self, command: bindings.Command) -> None:
        val_hash: bytes = get_value(command)
        current_root: bytes = await self.tree.check_root_hash()

        if self.debug:
            self.dbg(
                "check_root_hash", {"expected_value": current_root.decode("utf-8")}
            )

        if self.fix_path:
            self.fix(
                "check_root_hash", {"expected_value": current_root.decode("utf-8")}
            )
        else:
            if val_hash != current_root:
                print(f"ref_hash  : {val_hash}")
                print(f"root_hash : {current_root}")
                raise ValueError("Hash root mismatch")

    async def command_get(self, command: bindings.Command) -> None:
        val: int = int(get_value(command), 0)
        key: bytes = get_key(command)

        if self.debug:
            self.dbg("get", {"key": to_key(key), "expected_value": hex(val)})

        get: int = int.from_bytes(await self.tree.get(key))

        if self.fix_path:
            self.fix("get", {"key": to_key(key), "expected_value": hex(get)})
        else:
            if get != val:
                print(f"got    : {hex(get)}")
                print(f"ref val: {hex(val)}")
                raise ValueError("Get value mismatch")

    async def command_contains(self, command: bindings.Command):
        key: bytes = get_key(command)
        val: bool = get_value(command).decode("utf-8").lower() == "true"

        contains = await self.tree.contains(key=key)

        if self.fix_path:
            self.fix("contains", {"key": to_key(key), "expected_value": contains})
        else:
            if contains != val:
                print(f"contains {key}: {contains}")
                print(f"ref val {val}")
                raise ValueError("Contains value mismatch")


def get_value(command: bindings.Command) -> bytes:
    match command.id:
        case CommandId.Insert | CommandId.Contains | CommandId.Get:
            args = bytearray(
                ctypes.cast(command.arg2.ptr, ctypes.POINTER(ctypes.c_ubyte))[
                    : command.arg2.len - 1
                ]
            )
            return bytes(args)

        case CommandId.CheckRootHash:
            args = bytearray(
                ctypes.cast(command.arg1.ptr, ctypes.POINTER(ctypes.c_ubyte))[
                    : command.arg1.len
                ]
            )
            return bytes(args)

        case _:
            raise ValueError("unknown command in get value")


def get_key(command: bindings.Command) -> bytes:
    match command.id:
        case CommandId.Insert | CommandId.Contains | CommandId.Get | CommandId.Remove:
            args = bytearray(
                ctypes.cast(command.arg1.ptr, ctypes.POINTER(ctypes.c_ubyte))[
                    : command.arg1.len
                ]
            )
            return bytes(args)

        case _:
            raise ValueError("unknown command in get key")


def to_key(key: bytes) -> list[int] | None:
    if len(key) == 0:
        return None

    k: list[int] = [int(x) for x in key]

    # drop leading 0
    while k[0] == 0:
        k.pop(0)

    return k
