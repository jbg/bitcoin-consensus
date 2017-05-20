#[macro_use] extern crate bitflags;

#[link(name="bitcoinconsensus")]
extern {
    fn bitcoinconsensus_verify_script(script_pub_key: *const u8, script_pub_key_len: u16, tx_to: *const u8, tx_to_len: u16, n_in: u16, flags: u16, error: *mut u16) -> u16;
    fn bitcoinconsensus_verify_script_with_amount(script_pub_key: *const u8, script_pub_key_len: u16, amount: i64, tx_to: *const u8, tx_to_len: u16, n_in: u16, flags: u16, error: *mut u16) -> u16;
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
    pub flags ScriptVerificationFlags: u16 {
        const NONE = 0,
        const P2SH = (1 << 0),
        const DER_SIG = (1 << 2),
        const NULL_DUMMY = (1 << 4),
        const CHECK_LOCK_TIME_VERIFY = (1 << 9),
        const CHECK_SEQUENCE_VERIFY = (1 << 10),
        const WITNESS = (1 << 11),
        const ALL = P2SH.bits | DER_SIG.bits | NULL_DUMMY.bits | CHECK_LOCK_TIME_VERIFY.bits | CHECK_SEQUENCE_VERIFY.bits | WITNESS.bits
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

pub fn verify_script(pub_key: Vec<u8>, tx: Vec<u8>, input: u16, flags: ScriptVerificationFlags) -> Result<(), ScriptVerificationError> {
    unsafe {
        let mut err: u16 = 0;
        let res = bitcoinconsensus_verify_script(pub_key.as_ptr(), pub_key.len() as u16, tx.as_ptr(), tx.len() as u16, input, flags.bits, &mut err as *mut u16);
        if res == 1 { Ok(()) } else { Err(map_error(err)) }
    }
}

pub fn verify_script_with_amount(pub_key: Vec<u8>, amount: i64, tx: Vec<u8>, input: u16, flags: ScriptVerificationFlags) -> Result<(), ScriptVerificationError> {
    unsafe {
        let mut err: u16 = 0;
        let res = bitcoinconsensus_verify_script_with_amount(pub_key.as_ptr(), pub_key.len() as u16, amount, tx.as_ptr(), tx.len() as u16, input, flags.bits, &mut err as *mut u16);
        if res == 1 { Ok(()) } else { Err(map_error(err)) }
    }
}

#[cfg(test)]
mod tests {
    extern crate rustc_serialize;

    use self::rustc_serialize::hex::FromHex;
    use super::{verify_script, NONE};

    #[test]
    fn verify_transaction() {
        let res = verify_script("76a9144621d47f08fcb1e6be0b91144202de7a186deade88ac".from_hex().unwrap(),
                                "01000000015884e5db9de218238671572340b207ee85b628074e7e467096c267266baf77a4000000006a4730440220340f35055aceb14250e4954b23743332f671eb803263f363d1d7272f1d487209022037a0eaf7cb73897ba9069fc538e7275c5ae188e934ae47ca4a70453b64fc836401210234257444bd3aead2b851bda4288d60abe34095a2a8d49aff1d4d19773d22b32cffffffff01a0860100000000001976a9147821c0a3768aa9d1a37e16cf76002aef5373f1a888ac00000000".from_hex().unwrap(),
                                0,
                                NONE);
        assert!(res.is_ok(), format!("{:?}", res));
    }
}
