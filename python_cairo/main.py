import asyncio
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


from python_common.main import init_test
from python_common.test_tree import ITestTree


class TestTree(ITestTree):
    tree: PatriciaTree
    ffc: FactFetchingContext

    @classmethod
    async def create(cls) -> "TestTree":
        self = TestTree()
        self.ffc = FactFetchingContext(
            storage=MockStorage(), hash_func=pedersen_hash_func
        )
        self.tree = await instantiate_tree(self.ffc)
        return self

    async def insert(self, key: bytes, value: bytes) -> None:
        self.tree = await self.tree.update(
            ffc=self.ffc,
            modifications=[
                (from_bytes(key), SimpleLeafFact(value=from_bytes(value)))
            ],
        )

    async def get(self, key: bytes) -> bytes:
        get: SimpleLeafFact = await self.tree.get_leaf(
            ffc=self.ffc, index=from_bytes(key), fact_cls=SimpleLeafFact
        )
        return to_bytes(get.value)

    async def remove(self, key: bytes) -> None:
        self.tree = await self.tree.update(
            ffc=self.ffc, modifications=[(from_bytes(key), SimpleLeafFact.empty())]
        )

    async def contains(self, key: bytes) -> bool:
        get: bytes = await self.get(key)
        return from_bytes(get) != SimpleLeafFact(0).value

    async def check_root_hash(self) -> bytes:
        return hex(from_bytes(self.tree.root)).encode("utf-8")


async def instantiate_tree(ffc: FactFetchingContext) -> PatriciaTree:
    tree: PatriciaTree = await PatriciaTree.empty_tree(
        ffc=ffc, height=251, leaf_fact=SimpleLeafFact(value=0)
    )

    return tree


async def main():
    await init_test(TestTree.create)


if __name__ == "__main__":
    asyncio.run(main())
