# main.py

import binascii
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

lib_name = f"../rust/rust_ffi/target/debug/librust_ffi.{lib_suffix}"

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

print(result.decode("utf-8"))

# free memory make the program crash, we suspect a conflict with python garbage collector
# rust_lib.free_concatenated_string(result)


###
###
###

import asyncio
import random
from typing import Iterable, Set, Tuple

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


# @pytest.fixture
def ffc() -> FactFetchingContext:
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


# @pytest.mark.asyncio
# @parametrize_random_object(n_nightly_runs=5)
# @pytest.mark.parametrize(
#     "height,n_leaves",
#     [(2, 2**2), (10, 2**10), (10, 5), (10, 2**10 // 2)],
#     ids=["full_tree_small", "full_tree_large", "sparse_tree", "dense_tree"],
# )
# async def test_update_and_decommit(
#     random_object: random.Random, ffc: FactFetchingContext, height: int, n_leaves: int
# ):
#     """
#     Builds a Patricia tree using update(), and tests that the facts stored suffice to decommit.
#     """
#     print("TOTO")
#     # assert(False)

#     tree = await PatriciaTree.empty_tree(
#         ffc=ffc, height=height, leaf_fact=SimpleLeafFact(value=0)
#     )

#     # Create some random modifications, store the facts and update the tree.
#     # Note that leaves with value 0 are not modifications (hence, range(1, ...)).
#     leaves = [
#         SimpleLeafFact(value=value)
#         for value in random_object.choices(range(1, 1000), k=n_leaves)
#     ]
#     leaf_hashes_bytes = await asyncio.gather(
#         *(leaf_fact.set_fact(ffc=ffc) for leaf_fact in leaves)
#     )
#     leaf_hashes = [from_bytes(leaf_hash_bytes) for leaf_hash_bytes in leaf_hashes_bytes]
#     indices = random_object.sample(range(2**height), k=n_leaves)
#     modifications = list(zip(indices, leaves))
#     preimages: BinaryFactDict = {}
#     tree = await tree.update(ffc=ffc, modifications=modifications, facts=preimages)
#     root = from_bytes(tree.root)

#     # Sanity check - the hash of the values should be the keys.
#     for fact, preimage in preimages.items():
#         assert (
#             hash_preimage(preimage=preimage) == fact
#         ), f"Corrupted preimages: hash of {preimage} is not {fact}."

#     # Verify that the root can be reached using the preimages, from every leaf.
#     verify_leaves_are_reachable_from_root(
#         root=root, leaf_hashes=leaf_hashes, preimages=preimages
#     )


async def test_empty_tree(ffc: FactFetchingContext):
    print("TITI")

    tree: PatriciaTree = await PatriciaTree.empty_tree(
        ffc=ffc, height=3, leaf_fact=SimpleLeafFact(value=0)
    )

    root = from_bytes(tree.root)
    print("root", root)

    leaf1 = SimpleLeafFact(value=1)
    leaf2 = SimpleLeafFact(value=2)

    tree = await tree.update(ffc=ffc, modifications=[(0, leaf1)])
    print("root", hex(from_bytes(tree.root) + 5))

    tree = await tree.update(ffc=ffc, modifications=[(1, leaf1)])
    print("root", hex(from_bytes(tree.root) + 5))

    tree = await tree.update(ffc=ffc, modifications=[(2, leaf2)])
    print("root", hex(from_bytes(tree.root) + 5))

    tree = await tree.update(ffc=ffc, modifications=[(3, leaf1)])
    print("root", hex(from_bytes(tree.root) + 5))

    tree = await tree.update(ffc=ffc, modifications=[(4, leaf1)])
    print("root", hex(from_bytes(tree.root) + 5))

    tree = await tree.update(ffc=ffc, modifications=[(5, leaf2)])
    print("root", hex(from_bytes(tree.root) + 5))

    tree = await tree.update(ffc=ffc, modifications=[(6, leaf1)])
    print("root", hex(from_bytes(tree.root) + 5))

    tree = await tree.update(ffc=ffc, modifications=[(7, leaf1)])
    print("root", hex(from_bytes(tree.root) + 5))

    leaves = await tree.get_leaves(
        ffc=ffc, indices=[0, 1, 2, 3, 4, 5, 6, 7], fact_cls=SimpleLeafFact
    )
    print(leaves)

    # tree.validate_dataclass()
    # get_validated_fields(tree)


asyncio.run(test_empty_tree(ffc()))
