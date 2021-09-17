extern crate num_cpus;

use std::env;
use std::path::PathBuf;
use std::process::Command;


fn main() {
    let pkg_version = env::var("CARGO_PKG_VERSION").unwrap();
    let bitcoin_version = pkg_version.split('+').skip(1).next().unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    env::set_current_dir(&out_dir).unwrap();
    let tag = format!("v{}", bitcoin_version);
    Command::new("git").args(&["clone", "-b", tag.as_str(), "https://github.com/bitcoin/bitcoin"]).status().unwrap();
    let bitcoin_build_dir: PathBuf = [out_dir.as_str(), "bitcoin"].iter().collect();
    env::set_current_dir(&bitcoin_build_dir).unwrap();

    Command::new("sh").arg("./autogen.sh").status().unwrap();
    Command::new("sh").args(&["./configure",
                              "--with-gui=no",
                              "--with-daemon=no",
                              "--with-utils=no",
                              "--disable-wallet",
                              "--disable-zmq",
                              "--disable-bench",
                              "--disable-tests",
                              "--with-miniupnpc=no"]).status().unwrap();
    let cpus = num_cpus::get();
    Command::new("make").arg(format!("-j{}", cpus)).status().unwrap();

    println!("cargo:rustc-link-search=native={}/src/.libs", bitcoin_build_dir.to_str().unwrap());
    println!("cargo:rustc-link-search=native={}/src/secp256k1/.libs", bitcoin_build_dir.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=bitcoinconsensus");
    println!("cargo:rustc-link-lib=static=secp256k1");
}
