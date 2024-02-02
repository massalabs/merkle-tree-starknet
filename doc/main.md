The plan of this document is inspired by
https://www.notion.so/massa-innoteam/SoW-0c36c161f659456d955fadbc379eeeac?pvs=4#3e6c8dfa76c54386919ec528f7efbf78

# General explanation of Merkle trees
<!--
reuse info form
specification.pdf
 -->
## Genealogy
### Trie
Note:
- Prefix and key denote the same concept: They represent the path from the root node to the node they are attached. As such, they are not stored in the node. The position in the tree determines the value of a nodes prefix/key.
- Only leave nodes store a value

[![](https://mermaid.ink/img/pako:eNqFk01PxCAQhv8K4dQm3cP6ESMmJk28evJmepmUqUtsocGpuln2v8tKKe7SRE7Mk-eFMtADb41ELnjXm692B5ZYo5kf1hgqClaWD6H2xWixU9-CUVkGVhdFmJzGO-4Fq1P9Cf2Egm1vo60yW-X2NtpkMp1M5t8tOv75PkwU8lUQsmWuA0kxuRKTWezmMqZXYjo_5NV5TuUxpf_bTOm1VB67T7F0sWyzeXTkGJ0j5mpXXyLl5muaa-PiPcwAHWFsQCDgliZHJN3SwIi0W5qjIonnVjoRj3jFB7QDKOlf6uGkNJx2OGDDhZ9K7GDqqeGNPnoVJjIve91yQXbCik-jBMInBW8WhghRKjL2OTz-33-g4iPoV2O80kH_gccfWuLakg?type=png)](https://mermaid.live/edit#pako:eNqFk01PxCAQhv8K4dQm3cP6ESMmJk28evJmepmUqUtsocGpuln2v8tKKe7SRE7Mk-eFMtADb41ELnjXm692B5ZYo5kf1hgqClaWD6H2xWixU9-CUVkGVhdFmJzGO-4Fq1P9Cf2Egm1vo60yW-X2NtpkMp1M5t8tOv75PkwU8lUQsmWuA0kxuRKTWezmMqZXYjo_5NV5TuUxpf_bTOm1VB67T7F0sWyzeXTkGJ0j5mpXXyLl5muaa-PiPcwAHWFsQCDgliZHJN3SwIi0W5qjIonnVjoRj3jFB7QDKOlf6uGkNJx2OGDDhZ9K7GDqqeGNPnoVJjIve91yQXbCik-jBMInBW8WhghRKjL2OTz-33-g4iPoV2O80kH_gccfWuLakg)

### PATRICIA trie
Also know as Radix trie

Patricia tries are a compact form of tries. This is accomplished by fusing the nodes that have only one child.
As in tries, prefix/key denote the same concept and are not stored in nodes. As oposed to the trie, where the position of a node in a tree fully represent its prefix/key, because of the fuse of nodes and associated paths, "fused nodes" have to store their "representation" (what they are fused of).

[![](https://mermaid.ink/img/pako:eNptlE1vhCAQhv-K4aTJbtLPbWKTnnrdUw9NGi-o4y6pgkFod7Psfy_CKlDwJM878zLMoBfUsBZQibqe_TZHzEVW0Ypm-uGMiTzPiuL1ts7zkUNHTmXGi2KJGTzKhpXL2uOy9uJlL6c8t8v5-YZzuXBHf3AvocweLfHSMQ02xDTQIOWscWR8nzBO1zXzKP3hX7qsITgv-ApN-Boc2T7FtjyZy6PU5yiVNLYkWxFpAonRlO_MI-ddwlnSVk5pByNFJi_OxN2ubLt9yxRXt9Pwea3YoHTXfSJrpZ3XQVmmb4sKbs2iYKrM1LzBGg7KvwqOWx83Zb3VEq8bHTLSKHNGNx6D6aT8ga6cK29WJtGekKqg104xvVN-G9EGDcAHTFr9jV7m-AqJIwxQoVK_ttBh2YsKbawkJ9jj0ydpxdEGCC7BV1dpd1fRq3bHUrCPM21QOYdukBxbLOCd4APHwwKhJYLxvf1T6Lo7ctCpI6ZfjOmgDvcTXP8AxZNOfQ?type=png)](https://mermaid.live/edit#pako:eNptlE1vhCAQhv-K4aTJbtLPbWKTnnrdUw9NGi-o4y6pgkFod7Psfy_CKlDwJM878zLMoBfUsBZQibqe_TZHzEVW0Ypm-uGMiTzPiuL1ts7zkUNHTmXGi2KJGTzKhpXL2uOy9uJlL6c8t8v5-YZzuXBHf3AvocweLfHSMQ02xDTQIOWscWR8nzBO1zXzKP3hX7qsITgv-ApN-Boc2T7FtjyZy6PU5yiVNLYkWxFpAonRlO_MI-ddwlnSVk5pByNFJi_OxN2ubLt9yxRXt9Pwea3YoHTXfSJrpZ3XQVmmb4sKbs2iYKrM1LzBGg7KvwqOWx83Zb3VEq8bHTLSKHNGNx6D6aT8ga6cK29WJtGekKqg104xvVN-G9EGDcAHTFr9jV7m-AqJIwxQoVK_ttBh2YsKbawkJ9jj0ydpxdEGCC7BV1dpd1fRq3bHUrCPM21QOYdukBxbLOCd4APHwwKhJYLxvf1T6Lo7ctCpI6ZfjOmgDvcTXP8AxZNOfQ)


### Bonsai trie
Bonsai introduces several enhancements to the Besu client. Unlike other clients that store nodes of the world state by hash, Bonsai stores them by location in the trie. This approach enables natural pruning by replacing old nodes at the same position in the trie with the new ones. Unlike hash-based storage, which only adds new nodes to the database and keeps growing, Bonsai’s technique ensures that there is only one version of the state at any given time. Consequently, it does not support managing reorganizations (reorgs) as it only maintains a single state. Bonsai addresses this limitation by introducing “trielogs” (https://hackmd.io/@kt2am/BktBblIL3)

Quick comparision between forest of tries and bonsai tries https://besu.hyperledger.org/public-networks/concepts/data-storage-formats

### Merkle tree

 A Merkle Tree is a specialized tree where every "leaf" or node is labeled with the cryptographic hash of a data block. Conversely, every non-leaf node, often referred to as a branch, inner node, or inode, carries the cryptographic hash of its child nodes' labels. Here’s a visual representation:

![](https://upload.wikimedia.org/wikipedia/commons/thumb/9/95/Hash_Tree.svg/1920px-Hash_Tree.svg.png)

Alternate representation

[![](https://mermaid.ink/img/pako:eNp9U7FugzAQ_RXrplYlkp1EHRg6dfBAplaq1LiDEzsFFTACu2oV8u81NiZJgYCEfO_ePb877CPslZAQw2fNqxSxEtnnVVV0y8AHPYAob9Iz8s1zI2OUWvCuy2D04Bjk3nMYfPgFxVdKjntDJujgsRAZCZF5ITIYmnA0YWnGUzJRPfaBZ4wky3ET473J3N6rcfXEDOb2Xl9W-zf8W7RYPLW4pfgfRFpKAo_igYWvEEsK_VISOKGHHumEes5Zz5UnZBijC5fDXFy4Ghp14TpodN_G7PwhFVzzbfdBu1ztv5p-OkE6aAaxoCJLwUqIoJB1wTNhz_yxSzDQqSwkg9guhTxwk2sGkU-ZRm74z1smdOoJujbyMjukHu0ETladG61efss9xB01AlNZp_I549Z7EUApMq3qjb957gJGUPHyXSlLOfC8kac_Nxj4Xg?type=png)](https://mermaid.live/edit#pako:eNp9U7FugzAQ_RXrplYlkp1EHRg6dfBAplaq1LiDEzsFFTACu2oV8u81NiZJgYCEfO_ePb877CPslZAQw2fNqxSxEtnnVVV0y8AHPYAob9Iz8s1zI2OUWvCuy2D04Bjk3nMYfPgFxVdKjntDJujgsRAZCZF5ITIYmnA0YWnGUzJRPfaBZ4wky3ET473J3N6rcfXEDOb2Xl9W-zf8W7RYPLW4pfgfRFpKAo_igYWvEEsK_VISOKGHHumEes5Zz5UnZBijC5fDXFy4Ghp14TpodN_G7PwhFVzzbfdBu1ztv5p-OkE6aAaxoCJLwUqIoJB1wTNhz_yxSzDQqSwkg9guhTxwk2sGkU-ZRm74z1smdOoJujbyMjukHu0ETladG61efss9xB01AlNZp_I549Z7EUApMq3qjb957gJGUPHyXSlLOfC8kac_Nxj4Xg)

In this depiction, hashes 0-0 and 0-1 represent the hash values of data blocks L1 and L2, respectively. Meanwhile, hash 0 is derived from the combined hashes 0-0 and 0-1.

For comprehensive information on the Merkle Tree, visit here. The benefits of the Merkle Tree data structure are outlined here.

One important feature of Merkle Trees is that they allow exhibiting compact proofs of existence of an element in the tree. The proof size is logarithmic in the number of elements in the tree.


### Merkle - Patricia trie
A Merkle-Patricia Trie (MPT) is a combination of a Merkle Tree and a Patricia Tree. This data structure is famous because it is being used by Ethereum to store the state of an Ethereum blockchain. The Ethereum version of the MPT is composed of 3 types of nodes:

- Branch: A node with up to 16 child links, each corresponding to a hex character.

- Extension: A node storing a key segment with a common prefix and a link to the next node.

- Leaf: An end-node holding the key’s final segment and its value.

Alternate representation

[![](https://mermaid.ink/img/pako:eNqtltFumzAUhl8FWYqySWkHB0iaaOrFtErbRVOprVRpzi4MNg0qMRGBLVWUPHsNTgAb07VafQH48_HBP3B-s0NhShmaoceMrJfWgluiDQa7mMf5bDeMkvRvuCRZPhQdLiLv1iSM-eNwZvkja5gR_tQi9n6_HwxkDnncFIFMnKVpjm9vbu5n1tU2Z3wTp9yai4S_ZeDtOmNRvMULJPtlk6jp28fM6DTnbi7iN2J5GaMWj4MgYZ82n5sJZKLNmDtiAmfb3Cq1NIFfg-zLpRbLOF1wTUUgBIdLB3-rzqwtoGyBw21sq8DBjgoAgwpc7KrAw54KfOyrYIzHKpjgiQou8IUKpniqAoKJCgIcqCDEoQoopipgWJMf4UgBf0hSMFwdX3msCSOR00xLHMPH0HwQlgqh6dbvTmZ5Ys-dFIKdVUtoQ8f1_d4sUoKex_PPbevq_odhWp_C1kIT-BCF8GaFU3fcqxDMCp1z5x0CRU0BNlZ2NeN1vSqzexbKoCx1RkWhd_K8YgJlo25vztINOukMFqHZxButAnqtAnSrAN0qQLcK0K0CdKsA3SpAtwrQrQJ0qwDdKkC3CtCtAnSrAN0qQLcK0K0Ceq1CdIz11Hqnifueeur7GBL3zfU06U3RV0y2bT1c_XyHXXitrN6HyPP-X55nlmefO_BvsxgMpA8eDofSLsSpZfzV5XGkIaAQkeC4pRznwin13LHOzi5P-7Nk5W6s3qPcjqs4lUEnyu0Qr0P8DhkbSbP8cp-ubt8g6T-ttbeV8otGZgOnJkhMMDDB0ASpCTIVlrt7_fBOTFhSzdyaTWvmoRFasWxFYir-MXdlxALlS7ZiCzQTl5RFpEjyBRrJoWLDrsn2Iab5UgbkWcHao_XQWGwTe5GdFHl698xDNItIsmEjVKwpydn3mIhSWtWU0ThPs2v5r1v98o7QmvBfaXqK2b8ArSbxGw?type=png)](https://mermaid.live/edit#pako:eNqtltFumzAUhl8FWYqySWkHB0iaaOrFtErbRVOprVRpzi4MNg0qMRGBLVWUPHsNTgAb07VafQH48_HBP3B-s0NhShmaoceMrJfWgluiDQa7mMf5bDeMkvRvuCRZPhQdLiLv1iSM-eNwZvkja5gR_tQi9n6_HwxkDnncFIFMnKVpjm9vbu5n1tU2Z3wTp9yai4S_ZeDtOmNRvMULJPtlk6jp28fM6DTnbi7iN2J5GaMWj4MgYZ82n5sJZKLNmDtiAmfb3Cq1NIFfg-zLpRbLOF1wTUUgBIdLB3-rzqwtoGyBw21sq8DBjgoAgwpc7KrAw54KfOyrYIzHKpjgiQou8IUKpniqAoKJCgIcqCDEoQoopipgWJMf4UgBf0hSMFwdX3msCSOR00xLHMPH0HwQlgqh6dbvTmZ5Ys-dFIKdVUtoQ8f1_d4sUoKex_PPbevq_odhWp_C1kIT-BCF8GaFU3fcqxDMCp1z5x0CRU0BNlZ2NeN1vSqzexbKoCx1RkWhd_K8YgJlo25vztINOukMFqHZxButAnqtAnSrAN0qQLcK0K0CdKsA3SpAtwrQrQJ0qwDdKkC3CtCtAnSrAN0qQLcK0K0Ceq1CdIz11Hqnifueeur7GBL3zfU06U3RV0y2bT1c_XyHXXitrN6HyPP-X55nlmefO_BvsxgMpA8eDofSLsSpZfzV5XGkIaAQkeC4pRznwin13LHOzi5P-7Nk5W6s3qPcjqs4lUEnyu0Qr0P8DhkbSbP8cp-ubt8g6T-ttbeV8otGZgOnJkhMMDDB0ASpCTIVlrt7_fBOTFhSzdyaTWvmoRFasWxFYir-MXdlxALlS7ZiCzQTl5RFpEjyBRrJoWLDrsn2Iab5UgbkWcHao_XQWGwTe5GdFHl698xDNItIsmEjVKwpydn3mIhSWtWU0ThPs2v5r1v98o7QmvBfaXqK2b8ArSbxGw)


# Merkle trees uses and peculiarities in Starknet.
# Introduction to the different implementations
<!--
implies:
- list those different implementations
 -->
## implem A [Bonsai - Rust](https://github.com/keep-starknet-strange/bonsai-trie/blob/oss/src/trie/merkle_tree.rs)
The state commitment scheme uses a binary Merkle-Patricia trie with the Pedersen hash function. (https://docs.starknet.io/documentation/architecture_and_concepts/Network_Architecture/starknet-state/#merkle_patricia_trie)



This is used to update, mutate and access global Starknet state as well as individual contract states (https://github.com/keep-starknet-strange/bonsai-trie/blob/5e98fe9091233b0ab0eddf3681a29f0da2b4da40/src/trie/merkle_tree.rs#L68)

## implem B - [Starkware - Python](https://github.com/starkware-libs/cairo-lang/blob/caba294d82eeeccc3d86a158adb8ba209bf2d8fc/src/starkware/starkware_utils/commitment_tree/patricia_tree/patricia_tree.py#L26)

This PMT is:
- is structured as a binary tree.
- imutable. Any modification applied to the PMT implies the creation of a new one.

## implem C - [PathFinder - Rust](https://github.com/eqlabs/pathfinder/blob/main/crates/merkle-tree/src/tree.rs)
Starknet utilises a custom Binary Merkle-Patricia Tree to store and organise its state.
From an external perspective the tree is similar to a key-value store, where both key
and value are [Felts](Felt). The difference is that each tree is immutable,
and any mutations result in a new tree with a new root. This mutated variant can then
be accessed via the new root, and the old variant via the old root.
Trees share common nodes to be efficient. These nodes perform reference counting and
will get deleted once all references are gone. State can therefore be tracked over time
by mutating the current state, and storing the new root. Old states can be dropped by
deleting old roots which are no longer required.
#### Tree definition
It is important to understand that since all keys are [Felts](Felt), this means
all paths to a key are equally long - 251 bits.
Starknet defines three node types for a tree.
`Leaf nodes` which represent an actual value stored.
`Edge nodes` which connect two nodes, and __must be__ a maximal subtree (i.e. be as
long as possible). This latter condition is important as it strictly defines a tree (i.e. all
trees with the same leaves must have the same nodes). The path of an edge node can therefore
be many bits long.
`Binary nodes` is a branch node with two children, left and right. This represents
only a single bit on the path to a leaf.
A tree storing a single key-value would consist of two nodes. The root node would be an edge node
with a path equal to the key. This edge node is connected to a leaf node storing the value.
#### Implementation details
We've defined an additional node type, an `Unresolved node`. This is used to
represent a node who's hash is known, but has not yet been retrieved from storage (and we therefore
have no further details about it).
Our implementation is a mix of nodes from persistent storage and any mutations are kept in-memory. It is
done this way to allow many mutations to a tree before committing only the final result to storage. This
may be confusing since we just said trees are immutable -- but since we are only changing the in-memory
tree, the immutable tree still exists in storage. One can therefore think of the in-memory tree as containing the state changes between tree `N` and `N + 1`.

The in-memory tree is built using a graph of `Rc<RefCell<Node>>` which is a bit painful.

## implem D - [NethermindEth - Juno - Go](https://github.com/NethermindEth/juno/tree/main/core/trie)

Trie is a dense Merkle Patricia Trie (i.e., all internal nodes have two children).
This implementation allows for a "flat" storage by keying nodes on their path rather than their hash, resulting in O(1) accesses and O(log n) insertions.
The state trie [specification] describes a sparse Merkle Trie.
Note that this dense implementation results in an equivalent commitment.
Terminology:
  - path: represents the path as defined in the specification. Together with len,
    represents a relative path from the current node to the node's nearest non-empty child.
  - len: represents the len as defined in the specification. The number of bits to take
    from the fixed-length path to reach the nearest non-empty child.
  - key: represents the storage key for trie [Node]s. It is the full path to the node from the
    root.

[specification]: https://docs.starknet.io/documentation/develop/State/starknet-state/


## Comparison (qualities and drawbacks).

Implementation | Language | Qualities      | Drawbacks
------- | ---------------- | ---------- | ---------:
Pathfinder  | Rust | High performance, scalability, Cairo specification adherence |
Juno  | Golang        |    User-friendliness, ease of deployment, Ethereum compatibility    |
  Starkware - Patricia tree | python |       | python lol
  Bonsai | Rust |  Can revert to specific commit, Optimized for holding Starknet Felt items, Madara-compatible root hash, A Flat DB allowing direct access to items without requiring trie traversal, Trie Logs, Thread-safe transactional states       |

# [How to test your implementation](./test_framework.md)

# Resources

- https://github.com/keep-starknet-strange/bonsai-trie
  - https://github.com/keep-starknet-strange/bonsai-trie/blob/oss/src/trie/merkle_tree.rs
  - https://hackmd.io/@kt2am/BktBblIL3
  - https://ethereum.stackexchange.com/questions/268/ethereum-block-architecture/6413#6413
  - https://github.com/keep-starknet-strange/madara
  - https://github.com/lambdaclass/merkle_patricia_tree
- https://github.com/massalabs/madara-bonsai/blob/main/documentation/specification.html (remove the link to the css to fix the display)
  - old commit with css : https://github.com/massalabs/madara-bonsai/blob/d86abde7b0d5d0eb0503104ff69ace53e0b33ff9/documentation/specification.pdf
  - pdf version https://github.com/massalabs/madara-bonsai/blob/d86abde7b0d5d0eb0503104ff69ace53e0b33ff9/documentation/specification.pdf


  https://docs.alchemy.com/docs/patricia-merkle-tries

- starknet docs : https://docs.starknet.io/documentation/architecture_and_concepts/Network_Architecture/starknet-state/#merkle_patricia_trie

## [Hyperledger - Besu - Java](https://github.com/hyperledger/besu/tree/1a7635bc3ef75c31e5c5ac050b2cd3a22d833ada/ethereum/core/src/main/java/org/hyperledger/besu/ethereum/bonsai)

Similar data structure, for reference

Hyperledger besu implementation

Bonsai Tries is a data storage layout policy designed to reduce storage requirements and increase read performance.

Bonsai stores leaf values in a trie log, separate from the branches of the trie. Bonsai stores nodes by the location of the node instead of the hash of the node. Bonsai can access the leaf from the underlying storage directly using the account key. This greatly reduces the disk space needed for storage and allows for less resource-demanding and faster read performance. Bonsai inherently prunes orphaned nodes and old branches.



<!--
# Starknet tries

## What is it
## What is used for
## How does it work
-->

Maybe useful
[![](https://mermaid.ink/img/pako:eNpd0k2PgyAQBuC_YuaECU20Rw-btLXftT3sbcOFLePWpIixeNg0_e9LAV2CJ57hNToDT7gqgVAAa3963t0S1ibmWRCS0Yz2aeoLS0LmdE7zNHVeTQHn0jkbvSYkN57ym7fzf2-j_YN7f3IV-Rx5F31vH_kY-RT5EnmRzGYfyXLs1qoMsQ5zK9-0xS7E3rdvcQhxdHgM327QUommblD4edrMJsTWT87iFKLyM7Q4h7g4YCuAgsRe8kaYw32-ywz0DSUyKMxSYM2Hu2bm3F8mygetPn_bKxS6H5DC0AmusWy4-Vc5FlE0WvWVuy_22lDoePullInU_P7A1x9jEI1P?type=png)](https://mermaid.live/edit#pako:eNpd0k2PgyAQBuC_YuaECU20Rw-btLXftT3sbcOFLePWpIixeNg0_e9LAV2CJ57hNToDT7gqgVAAa3963t0S1ibmWRCS0Yz2aeoLS0LmdE7zNHVeTQHn0jkbvSYkN57ym7fzf2-j_YN7f3IV-Rx5F31vH_kY-RT5EnmRzGYfyXLs1qoMsQ5zK9-0xS7E3rdvcQhxdHgM327QUommblD4edrMJsTWT87iFKLyM7Q4h7g4YCuAgsRe8kaYw32-ywz0DSUyKMxSYM2Hu2bm3F8mygetPn_bKxS6H5DC0AmusWy4-Vc5FlE0WvWVuy_22lDoePullInU_P7A1x9jEI1P)

