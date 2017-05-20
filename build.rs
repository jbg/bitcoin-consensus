extern crate num_cpus;

use std::process::Command;
use std::env;

fn main() {
    Command::new("git").args(&["submodule", "update", "--init"]).status().unwrap();
    env::set_current_dir("bitcoin-core").unwrap();

    Command::new("sh").arg("./autogen.sh").status().unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    Command::new("sh").args(&["./configure",
                              &format!("--prefix={}", out_dir),
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
    Command::new("make").args(&[&format!("-j{}", cpus), "install"]).status().unwrap();
    env::set_current_dir("src/secp256k1").unwrap();
    Command::new("make").args(&[&format!("-j{}", cpus), "install"]).status().unwrap();

    let target = env::var("TARGET").unwrap();
    if !target.contains("msvc") {
        let cpplib = if target.contains("darwin") || target.contains("freebsd") { "c++" } else { "stdc++" };
        println!("cargo:rustc-link-lib=dylib={}", cpplib);
    }

    println!("cargo:rustc-link-search=native={}/lib", out_dir);
    println!("cargo:rustc-link-lib=static=bitcoinconsensus");
    println!("cargo:rustc-link-lib=static=secp256k1");
}
