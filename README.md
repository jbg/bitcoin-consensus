# bitcoin-consensus
Rust bindings to libbitcoinconsensus

This project allows Bitcoin software to be written in Rust while using the same library for script verification that Bitcoin Core uses.

This is a work in progress. Version numbers will match the version of the libbitcoinconsensus library.

`cargo build` will automatically check out the Bitcoin Core GitHub repository, configure and build it, and build the bindings statically linked to libbitcoinconsensus and libsecp256k1.
