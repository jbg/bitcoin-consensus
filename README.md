# bitcoin-consensus
Rust bindings to libbitcoinconsensus

This project allows Bitcoin software to be written in Rust while using the same library for script verification that Bitcoin Core uses.

`cargo build` will automatically check out the Bitcoin Core GitHub repository, configure and build it, and build the bindings statically linked to libbitcoinconsensus and libsecp256k1.

Currently building against libbitcoinconsensus v0.14.1.
