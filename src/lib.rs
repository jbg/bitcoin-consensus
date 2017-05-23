//! Rust bindings to libbitcoinconsensus
//!
//! ```rust
//! extern crate bitcoin_consensus;
//! extern crate hex;
//!
//! use hex::FromHex;
//! use bitcoin_consensus::{verify_script, ScriptVerificationFlags};
//!
//! fn main() {
//!     let pubkey: Vec<u8> = Vec::from_hex("76a9144621d47f08fcb1e6be0b91144202de7a186deade88ac").unwrap();
//!     let tx: Vec<u8> = Vec::from_hex("01000000015884e5db9de218238671572340b207ee85b628074e7e467096c267266baf77a4000000006a4730440220340f35055aceb14250e4954b23743332f671eb803263f363d1d7272f1d487209022037a0eaf7cb73897ba9069fc538e7275c5ae188e934ae47ca4a70453b64fc836401210234257444bd3aead2b851bda4288d60abe34095a2a8d49aff1d4d19773d22b32cffffffff01a0860100000000001976a9147821c0a3768aa9d1a37e16cf76002aef5373f1a888ac00000000").unwrap();
//!     match verify_script(&pubkey, &tx, 0, ScriptVerificationFlags::empty()) {
//!         Ok(_) => println!("transaction verified"),
//!         Err(e) => panic!("transaction did not verify: {:?}", e)
//!     }
//! }
//! ```

// Copyright 2017 Jasper Bryant-Greene
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use] extern crate bitflags;

extern {
    fn bitcoinconsensus_verify_script(script_pub_key: *const u8, script_pub_key_len: u16, tx_to: *const u8, tx_to_len: u16, n_in: u16, flags: u16, error: *mut u16) -> u16;
    fn bitcoinconsensus_verify_script_with_amount(script_pub_key: *const u8, script_pub_key_len: u16, amount: i64, tx_to: *const u8, tx_to_len: u16, n_in: u16, flags: u16, error: *mut u16) -> u16;
    fn bitcoinconsensus_version() -> u16;
}

#[derive(Debug)]
pub enum ScriptVerificationError {
    TxIndexError,
    TxSizeMismatch,
    TxDeserializeError,
    AmountRequired,
    InvalidFlags,
    UnknownError(u16)
}

bitflags! {
    pub struct ScriptVerificationFlags: u16 {
        const VERIFY_P2SH = 1;
        const VERIFY_DER_SIG = (1 << 2);
        const VERIFY_NULL_DUMMY = (1 << 4);
        const VERIFY_CHECK_LOCK_TIME = (1 << 9);
        const VERIFY_CHECK_SEQUENCE = (1 << 10);
        const VERIFY_WITNESS = (1 << 11);
    }
}

fn map_error(err: u16) -> ScriptVerificationError {
    match err {
        1 => ScriptVerificationError::TxIndexError,
        2 => ScriptVerificationError::TxSizeMismatch,
        3 => ScriptVerificationError::TxDeserializeError,
        4 => ScriptVerificationError::AmountRequired,
        5 => ScriptVerificationError::InvalidFlags,
        _ => ScriptVerificationError::UnknownError(err)
    }
}

/// Verify that the transaction input correctly spends the previous output, considering any
/// additional constraints specified by flags.
pub fn verify_script(pub_key: &[u8], tx: &[u8], input: u16, flags: ScriptVerificationFlags) -> Result<(), ScriptVerificationError> {
    unsafe {
        let mut err: u16 = 0;
        let res = bitcoinconsensus_verify_script(pub_key.as_ptr(), pub_key.len() as u16, tx.as_ptr(), tx.len() as u16, input, flags.bits, &mut err as *mut u16);
        if res == 1 { Ok(()) } else { Err(map_error(err)) }
    }
}

/// Verify that the transaction input correctly spends the previous output, considering any
/// additional constraints specified by flags.
pub fn verify_script_with_amount(pub_key: &[u8], amount: i64, tx: &[u8], input: u16, flags: ScriptVerificationFlags) -> Result<(), ScriptVerificationError> {
    unsafe {
        let mut err: u16 = 0;
        let res = bitcoinconsensus_verify_script_with_amount(pub_key.as_ptr(), pub_key.len() as u16, amount, tx.as_ptr(), tx.len() as u16, input, flags.bits, &mut err as *mut u16);
        if res == 1 { Ok(()) } else { Err(map_error(err)) }
    }
}

/// Return the linked version of libbitcoinconsensus.
pub fn version() -> u16 {
    unsafe {
        bitcoinconsensus_version()
    }
}
