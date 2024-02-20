from abc import ABC, abstractmethod

Key = bytes
Value = bytes


class ITestTree(ABC):

    @classmethod
    @abstractmethod
    async def create(cls) -> "ITestTree":
        raise NotImplementedError

    @abstractmethod
    async def insert(self, key: Key, value: Value) -> None:
        raise NotImplementedError

    @abstractmethod
    async def get(self, key: Key) -> Value:
        raise NotImplementedError

    @abstractmethod
    async def remove(self, key: Key) -> None:
        raise NotImplementedError

    @abstractmethod
    async def contains(self, key: Key) -> bool:
        raise NotImplementedError

    @abstractmethod
    async def check_root_hash(self) -> Value:
        raise NotImplementedError
