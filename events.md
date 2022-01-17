### FT/NFT contracts

We have [FT](https://nomicon.io/Standards/FungibleToken/Event.html) and [NFT](
https://nomicon.io/Standards/NonFungibleToken/Event.html) events standards.

I've created a simple contract that emulates FT/NFT contract by providing logs.  
[Indexer](https://github.com/near/near-indexer-for-explorer) parses these logs and adds the information to the corresponding tables.

You need to run Indexer for localnet, then run the following commands:
```bash
node compile.js
NEAR_ENV=local near deploy --wasmFile path_to_sources/target/wasm32-unknown-unknown/release/nft_duck.wasm --accountId test.near
NEAR_ENV=local near call test.near move_ft '{}' --accountId test.near
NEAR_ENV=local near call test.near move_nft '{}' --accountId test.near
```
