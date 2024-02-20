import asyncio
import sys

from python_common.main import init_test
from python_common.test_tree import ITestTree

# import you dependencies here


class TestTree(ITestTree):
    # wrap your implementaion here

    @classmethod
    async def create(cls) -> "TestTree":
        raise NotImplementedError("create")

    async def insert(self, key: bytes, value: bytes) -> None:
        raise NotImplementedError("insert")

    async def get(self, key: bytes) -> bytes:
        raise NotImplementedError("get")

    async def remove(self, key: bytes) -> None:
        raise NotImplementedError("remove")

    async def contains(self, key: bytes) -> bool:
        raise NotImplementedError("contains")

    async def check_root_hash(self) -> bytes:
        raise NotImplementedError("check_root_hash")


async def main():
    await init_test(TestTree.create)


if __name__ == "__main__":
    asyncio.run(main())
