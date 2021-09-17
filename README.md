# bitcoin-consensus
Rust bindings to libbitcoinconsensus

[![crates.io](https://img.shields.io/crates/v/bitcoin-consensus.svg)](https://crates.io/crates/bitcoin-consensus) [![docs.rs](https://docs.rs/bitcoin-consensus/badge.svg)](https://docs.rs/bitcoin-consensus) [![CircleCI](https://circleci.com/gh/jbg/bitcoin-consensus.svg?style=svg)](https://circleci.com/gh/jbg/bitcoin-consensus)

This project allows Bitcoin software to be written in Rust while using the same library for script verification that Bitcoin Core uses.

`cargo build` will automatically check out the Bitcoin Core GitHub repository, configure and build it, and build the bindings statically linked to libbitcoinconsensus and libsecp256k1.

The Bitcoin Core consensus code requires an implementation of the C++ standard library.
To give flexibility to its users, `bitcoin-consensus` does not link in a version of the C++ standard library itself.
Instead, we recommend depending in [`link-cplusplus`](https://docs.rs/link-cplusplus/1.0.5/link_cplusplus/) in your application.

Currently building against the consensus library from Bitcoin Core v0.18.0.
