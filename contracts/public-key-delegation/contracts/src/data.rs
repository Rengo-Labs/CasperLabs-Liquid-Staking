use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert, ext_ffi::casper_random_bytes,
};

use casper_types::{
    bytesrepr::{ FromBytes, ToBytes }, CLTyped,
    SecretKey, PublicKey, api_error
};
use core::convert::TryInto;

const RANDOM_BYTES_COUNT: usize = 32;

pub fn get_key<T: FromBytes + CLTyped>(name: &str) -> Option<T> {
    match runtime::get_key(name) {
        None => None,
        Some(value) => {
            let key = value.try_into().unwrap_or_revert();
            let result = storage::read(key).unwrap_or_revert().unwrap_or_revert();
            Some(result)
        }
    }
}

pub fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
    match runtime::get_key(name) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(name, key);
        }
    }
}

fn random_bytes() -> [u8; RANDOM_BYTES_COUNT] {
    let mut ret = [0; RANDOM_BYTES_COUNT];
    let result = unsafe { ext_ffi::casper_random_bytes(ret.as_mut_ptr(), RANDOM_BYTES_COUNT) };
    api_error::result_from(result).unwrap_or_revert();
    ret
}