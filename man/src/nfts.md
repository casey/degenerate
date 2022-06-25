# Degenerate Bitcoin NFTs

**[The Command Line is the Frontline: This is the bleeding edge.](#warnings)**

Degenerate NFTs are generative art produced by [Degenerate](#degenerate) and issued as NFTs on the Bitcoin blockchain using the [Ordinal](#ordinals) NFT scheme.

We are minting NFTs in order to gain experience with the technology, gather feedback, and build a community. Degenerate NFTs should be treated as digital baubles that glint and glimmer but may suddenly and spontaneously shatter.

## Mints

Mints will be announced on [Twitter](https://twitter.com/degencomputer). Until the underlying NFT scheme is mature, we'll be giving away NFTs or selling them for nominal amounts of sats using the Lightning Network.

Minted NFTs will be posted to [Instagram](https://instagram.com/degencomputer).

## Discussion and Help

Visit the [Degenerate Discord](https://discord.gg/87cjuz4FYg) to discuss the Degenerate engine, ordinals, Degenerate NFTs, or get help minting your own Ordinal NFTs of any kind.

## Minting, sending, and receiving Ordinals

Ordinal NFTs are bound to ordinal numbers, a numbering scheme for satoshis. Ownership of ordinals is tracked using the Bitcoin blockchain, and transferring ordinals is done using Bitcoin transactions.

There are no ordinal-aware wallets, so transfers must be done manually, either by misappropriating existing wallet software, or by writing your own.

To find a good ordinal number to attach an NFT to, use an ord API server. A public instance is running at api.ordinals.com:8000. To look up the ordinal numbers in a given UTXO, use the list API endpoint. [Example API URL for c581fd4054f1663c7193c640b4d81eeb3d4fca06f1bb29937a082c9122b1a1d6:0](http://api.ordinals.com:8000/list/c581fd4054f1663c7193c640b4d81eeb3d4fca06f1bb29937a082c9122b1a1d6:0).

Ordinal NFTs can be minted and verified using the `mint` and `verify` subcommands of the [`ord` utility](https://github.com/casey/ord).

## Warnings

Ordinals, the Ordinal NFT scheme, and Degenerate NFTs should be considered pre-alpha quality and subject to change at any time.

Ordinal numbers, defined in [The Ordinal BIP](https://github.com/casey/ord/blob/master/bip.mediawiki), are a mapping of ordinal numbers to satoshis, and ultimately to UTXOs. Ordinal NFTs are assigned to ordinal numbers, and thus rely on the stability and correctness of the underlying mapping. The Ordinal mapping is extremely simple and believe to be complete and bug-free, but further review and discussion is most welcome. The Ordinal spec, `ord` command line tool, and issue tracker are hosted in the [ord GitHub repository](https://github.com/casey/ord/).

The Ordinal NFT scheme, implemented [here](https://github.com/casey/ord/blob/master/src/nft.rs), is a scheme for issuing NFTs and assigning them to ordinal numbers, allowing their ownership to be tracked on the Bitcoin blockchain. The NFT scheme has received minimal review, and should be considered unstable. Improvements to the NFT schema may be necessary that render previously issued NFTs invalid according to the new schema.
