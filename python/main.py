import ctypes
import os
import platform
from icecream import ic

# # Define the result type and argument types of the function `add`
# rust_lib.add.restype = ctypes.c_int
# rust_lib.add.argtypes = [ctypes.c_int, ctypes.c_int]

# # Call the function `add`
# result = rust_lib.add(35, 34)  # 69

# print(f"Result from external addition of 35 and 34: {result}")


# play with strings
# rust_lib.concatenate_strings.restype = ctypes.c_char_p

# # call and concat
# s1 = b"Hello, "
# s2 = b"world!"
# result = rust_lib.concatenate_strings(s1, s2)

# print(result.decode("utf-8"))

# free memory make the program crash, we suspect a conflict with python garbage collector
# rust_lib.free_concatenated_string(result)


###
###
###

import asyncio
import random
from typing import Iterable, List, Set, Tuple

from queue import Queue

import sys

# get python home
py_home: str = sys.prefix
# needed to import from starkware
sys.path.append(py_home + "/../cairo-lang/src/")
# needed to import from starkware.storage, strangely 'from starkware.storage.test_utils import MockStorage' does not work
# but 'from storage.test_utils import MockStorage' does work
sys.path.append(py_home + "/../cairo-lang/src/starkware/")


from starkware.crypto.signature.fast_pedersen_hash import pedersen_hash_func
from starkware.python.random_test_utils import parametrize_random_object
from starkware.python.utils import from_bytes, to_bytes
from starkware.starkware_utils.commitment_tree.binary_fact_tree import BinaryFactDict
from starkware.starkware_utils.commitment_tree.patricia_tree.nodes import (
    BinaryNodeFact,
    EdgeNodeFact,
    PatriciaNodeFact,
)
from starkware.starkware_utils.commitment_tree.patricia_tree.patricia_tree import (
    PatriciaTree,
)
from starkware.storage.storage import FactFetchingContext
from starkware.storage.storage_utils import SimpleLeafFact
from storage.test_utils import MockStorage


from command_interpreter import CommandInterpreter
from bindings import Command, ArrayWrapper, CommandList, load_rust_library


# @pytest.fixture
def generate_ffc() -> FactFetchingContext:
    return FactFetchingContext(storage=MockStorage(), hash_func=pedersen_hash_func)


def hash_preimage(preimage: Tuple[int, ...]) -> int:
    """
    Preimages have variadic length.

    Binary node facts are simply the hash of the two children; edge node facts are the hash of the
    bottom node fact and the path, plus the path length.
    """
    node_fact: PatriciaNodeFact
    if len(preimage) == 2:
        node_fact = BinaryNodeFact(*map(to_bytes, preimage))
    else:
        length, path, bottom = preimage
        node_fact = EdgeNodeFact(
            bottom_node=to_bytes(bottom), edge_path=path, edge_length=length
        )
    return from_bytes(node_fact._hash(hash_func=pedersen_hash_func))


def verify_leaves_are_reachable_from_root(
    root: int, leaf_hashes: Iterable[int], preimages: BinaryFactDict
):
    """
    Given a list of leaves and a collection of preimages, verifies that the preimages suffice to
    descend to all leaves.
    """
    leaves_reached: Set[int] = set()
    facts_to_open: Queue[int] = Queue()
    facts_to_open.put(root)

    while not facts_to_open.empty():
        next_fact = facts_to_open.get()
        if next_fact in leaf_hashes:
            leaves_reached.add(next_fact)
            continue
        assert next_fact in preimages

        preimage = preimages[next_fact]
        if len(preimage) == 3:
            # Edge node. Next fact is the third entry.
            facts_to_open.put(preimage[2])
        else:
            # Binary node; both children should be opened.
            left_child, right_child = preimage
            facts_to_open.put(left_child)
            facts_to_open.put(right_child)

    # Done traversing, assert we reached all leaves.
    assert leaves_reached == set(leaf_hashes)


def print_tree(tree: PatriciaTree):
    pass


async def test_empty_tree(ffc: FactFetchingContext):
    tree = await instantiate_tree(ffc)

    # root = from_bytes(tree.root)
    # print("root", root)

    leaf1 = SimpleLeafFact(value=1)
    leaf2 = SimpleLeafFact(value=2)

    tree = await tree.update(ffc=ffc, modifications=[(0, leaf1)])
    print("root", hex(from_bytes(tree.root)))
    # print(tree)

    tree = await tree.update(ffc=ffc, modifications=[(1, leaf1)])
    print("root", hex(from_bytes(tree.root)))

    tree = await tree.update(ffc=ffc, modifications=[(2, leaf2)])
    print("root", hex(from_bytes(tree.root)))

    tree = await tree.update(ffc=ffc, modifications=[(3, leaf1)])
    print("root", hex(from_bytes(tree.root)))

    tree = await tree.update(ffc=ffc, modifications=[(4, leaf1)])
    print("root", hex(from_bytes(tree.root)))

    tree = await tree.update(ffc=ffc, modifications=[(5, leaf2)])
    print("root", hex(from_bytes(tree.root)))

    tree = await tree.update(ffc=ffc, modifications=[(6, leaf1)])
    print("root", hex(from_bytes(tree.root)))

    tree = await tree.update(ffc=ffc, modifications=[(7, leaf1)])
    print("root", hex(from_bytes(tree.root)))

    tree = await tree.update(ffc=ffc, modifications=[(8, leaf1)])
    print("root", hex(from_bytes(tree.root)))

    tree = await tree.update(ffc=ffc, modifications=[(9, leaf2)])
    print("root", hex(from_bytes(tree.root)))

    print(tree.height)

    # leaves = await tree.get_leaves(
    #     ffc=ffc, indices=[0, 1, 2, 3, 4, 5, 6, 7], fact_cls=SimpleLeafFact
    # )
    # print(tree.height)
    # print(leaves)

    # tree.validate_dataclass()
    # get_validated_fields(tree)


async def instantiate_tree(ffc: FactFetchingContext) -> PatriciaTree:
    tree: PatriciaTree = await PatriciaTree.empty_tree(
        ffc=ffc, height=251, leaf_fact=SimpleLeafFact(value=0)
    )

    return tree


async def run_test(test_path: str):
    ic(test_path)
    lib = load_rust_library()
    interpreter = CommandInterpreter(lib)
    ffc = generate_ffc()
    tree: PatriciaTree = await instantiate_tree(ffc)

    # move to bindings.py
    lib.load_scenario.argtypes = [ctypes.c_char_p]
    lib.load_scenario.restype = CommandList

    new_var: bytes = test_path.encode("utf-8")
    commands: CommandList = lib.load_scenario(new_var)

    vec_cmd = [commands.test_commands[i] for i in range(commands.len)]

    for cmd in vec_cmd:
        tree = await interpreter.run_command(ffc, cmd, tree)


async def test_empty_tree2(ffc: FactFetchingContext):
    tree = await instantiate_tree(ffc)

    leaf1 = SimpleLeafFact(value=1)

    tree = await tree.update(ffc=ffc, modifications=[(1, leaf1)])
    print("root", hex(from_bytes(tree.root)))

    tree = await tree.update(ffc=ffc, modifications=[(2, leaf1)])
    print("root", hex(from_bytes(tree.root)))


def main():
    # get all scenario
    tests = [file for file in os.listdir("../scenario") if file.endswith(".yml")]
    tests = ["1.yml", "4.yml"]
    # print(tests)

    for test in tests:
        print(f"Running test {test}")
        asyncio.run(run_test(f"../scenario/{test}"))


if __name__ == "__main__":
    # asyncio.run(test_empty_tree(ffc()))
    main()
