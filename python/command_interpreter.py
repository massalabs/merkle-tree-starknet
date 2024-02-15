import ctypes
from ctypes import c_char_p
from bindings import Command, ArrayWrapper

from starkware.starkware_utils.commitment_tree.patricia_tree.patricia_tree import (
    PatriciaTree,
)
from starkware.storage.storage import FactFetchingContext
from starkware.storage.storage_utils import SimpleLeafFact
from enum import IntEnum
from starkware.python.utils import from_bytes, to_bytes


from icecream import ic


# typedef enum CommandId {
#   Insert,
#   Remove,
#   CheckRootHash,
#   Get,
#   Contains,
# } CommandId;
#
# Map the enum to a Python Enum.
# /!\ this has to be done manually as C enums does not seems to map directly to Python enums.
class CommandId(IntEnum):
    Insert = 0
    Remove = 1
    CheckRootHash = 2
    Get = 3
    Contains = 4


class CommandInterpreter:
    def __init__(self, ffi_lib: ctypes.CDLL):
        self.ffi_lib = ffi_lib

    async def run_command(
        self, ffc: FactFetchingContext, command: Command, tree: PatriciaTree
    ) -> PatriciaTree:
        match command.id:
            case CommandId.Insert:
                val: int = int(get_value(command), 0)
                key: int = get_key(command)
                print("Insert", key, val)

                leaf: SimpleLeafFact = SimpleLeafFact(val)
                tree = await tree.update(ffc=ffc, modifications=[(key, leaf)])

            case CommandId.Remove:
                key_remove: int = get_key(command)
                print("Remove", key_remove)

                leaf_r: SimpleLeafFact = SimpleLeafFact.empty()
                tree = await tree.update(ffc=ffc, modifications=[(key_remove, leaf_r)])

            case CommandId.CheckRootHash:
                print("CheckRootHash")
                val_hash: str = get_value(command)
                current_root: str = hex(from_bytes(tree.root))
                if val_hash != current_root:
                    print(f"ref_hash  : {val_hash}")
                    print(f"root_hash : {current_root}")
                    raise ValueError("Hash root mismatch")
                else:
                    print("Hash root match")

            case CommandId.Get:
                print("Get")
            case CommandId.Contains:
                print("Contains")
            case _:
                print("Unknown command in run command")

        return tree


def get_value(command: Command) -> str:
    match command.id:
        case CommandId.Insert | CommandId.Contains | CommandId.Get:
            arg: bytes | None = c_char_p(command.arg2).value
            if not arg:
                raise ValueError("empty value")
            return arg.decode("utf-8")

        case CommandId.CheckRootHash:
            args = [command.arg1.ptr[i] for i in range(command.arg1.len)]
            return bytes(args).decode("utf-8")

        case _:
            raise ValueError("unknown command in get value")


def get_key(command: Command):
    match command.id:
        case CommandId.Insert | CommandId.Contains | CommandId.Get | CommandId.Remove:
            args = [command.arg1.ptr[i] for i in range(command.arg1.len)]
            integer = int.from_bytes(bytes(args), byteorder="big")
            return integer

        case _:
            raise ValueError("unknown command in get key")
