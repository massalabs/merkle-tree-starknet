
# General explanation of Merkle trees
<!--
reuse info form
specification.pdf
 -->
# Merkle trees uses and peculiarities in Starknet.
# Introduction to the different implementations
<!--
implies:
- list those different implementations
 -->
## implem A
## implem B
## Comparison (qualities and drawbacks).

# How to test your implementation
## If needed write your runner
## Implement the interface / trait that glued your implementation with the test framework


## Resources

- https://github.com/keep-starknet-strange/bonsai-trie
  - https://github.com/keep-starknet-strange/bonsai-trie/blob/oss/src/trie/merkle_tree.rs
  - https://hackmd.io/@kt2am/BktBblIL3
  - https://ethereum.stackexchange.com/questions/268/ethereum-block-architecture/6413#6413
  - https://github.com/hyperledger/besu/tree/1a7635bc3ef75c31e5c5ac050b2cd3a22d833ada/ethereum/core/src/main/java/org/hyperledger/besu/ethereum/bonsai
  - https://github.com/keep-starknet-strange/madara
  - https://github.com/lambdaclass/merkle_patricia_tree
- https://github.com/massalabs/madara-bonsai/blob/main/documentation/specification.html (remove the link to the css to fix the display)
  - old commit with css : https://github.com/massalabs/madara-bonsai/blob/d86abde7b0d5d0eb0503104ff69ace53e0b33ff9/documentation/specification.pdf
  - pdf version https://github.com/massalabs/madara-bonsai/blob/d86abde7b0d5d0eb0503104ff69ace53e0b33ff9/documentation/specification.pdf


  https://docs.alchemy.com/docs/patricia-merkle-tries

- starknet docs : https://docs.starknet.io/documentation/architecture_and_concepts/Network_Architecture/starknet-state/#merkle_patricia_trie


<!--
# Starknet tries

## What is it
## What is used for
## How does it work
-->

Maybe useful
[![](https://mermaid.ink/img/pako:eNpd0k2PgyAQBuC_YuaECU20Rw-btLXftT3sbcOFLePWpIixeNg0_e9LAV2CJ57hNToDT7gqgVAAa3963t0S1ibmWRCS0Yz2aeoLS0LmdE7zNHVeTQHn0jkbvSYkN57ym7fzf2-j_YN7f3IV-Rx5F31vH_kY-RT5EnmRzGYfyXLs1qoMsQ5zK9-0xS7E3rdvcQhxdHgM327QUommblD4edrMJsTWT87iFKLyM7Q4h7g4YCuAgsRe8kaYw32-ywz0DSUyKMxSYM2Hu2bm3F8mygetPn_bKxS6H5DC0AmusWy4-Vc5FlE0WvWVuy_22lDoePullInU_P7A1x9jEI1P?type=png)](https://mermaid.live/edit#pako:eNpd0k2PgyAQBuC_YuaECU20Rw-btLXftT3sbcOFLePWpIixeNg0_e9LAV2CJ57hNToDT7gqgVAAa3963t0S1ibmWRCS0Yz2aeoLS0LmdE7zNHVeTQHn0jkbvSYkN57ym7fzf2-j_YN7f3IV-Rx5F31vH_kY-RT5EnmRzGYfyXLs1qoMsQ5zK9-0xS7E3rdvcQhxdHgM327QUommblD4edrMJsTWT87iFKLyM7Q4h7g4YCuAgsRe8kaYw32-ywz0DSUyKMxSYM2Hu2bm3F8mygetPn_bKxS6H5DC0AmusWy4-Vc5FlE0WvWVuy_22lDoePullInU_P7A1x9jEI1P)