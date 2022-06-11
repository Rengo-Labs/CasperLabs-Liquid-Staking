use alloc::string::{String, ToString};
use core::convert::TryInto;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    ApiError, CLTyped, Key, URef,
};


pub struct Dict {
    uref: URef,
}

impl Dict {
    pub fn instance(name: &str) -> Dict {
        let key = runtime::get_key(name).unwrap_or_revert();
        let uref = *key.as_uref().unwrap_or_revert();
        Dict { uref }
    }

    pub fn init(name: &str) {
        storage::new_dictionary(name).unwrap_or_revert();
    }

    pub fn at(uref: URef) -> Dict {
        Dict { uref }
    }

    pub fn get<T: CLTyped + FromBytes>(&self, key: &str) -> Option<T> {
        storage::dictionary_get(self.uref, key)
            .unwrap_or_revert()
            .unwrap_or_default()
    }

    pub fn get_by_key<T: CLTyped + FromBytes>(&self, key: &Key) -> Option<T> {
        self.get(&key_to_str(key))
    }

    pub fn set<T: CLTyped + ToBytes>(&self, key: &str, value: T) {
        storage::dictionary_put(self.uref, key, Some(value));
    }

    pub fn set_by_key<T: CLTyped + ToBytes>(&self, key: &Key, value: T) {
        self.set(&key_to_str(key), value);
    }

    pub fn remove<T: CLTyped + ToBytes>(&self, key: &str) {
        storage::dictionary_put(self.uref, key, Option::<T>::None);
    }

    pub fn remove_by_key<T: CLTyped + ToBytes>(&self, key: &Key) {
        self.remove::<T>(&key_to_str(key));
    }
}

pub fn key_to_str(key: &Key) -> String {
    match key {
        Key::Account(account) => account.to_string(),
        Key::Hash(package) => hex::encode(package),
        _ => runtime::revert(ApiError::UnexpectedKeyVariant),
    }
}

pub fn get_key<T: FromBytes + CLTyped>(name: &str) -> Option<T> {
    match runtime::get_key(name) {
        None => None,
        Some(value) => {
            let key = value.try_into().unwrap_or_revert();
            let value = storage::read(key).unwrap_or_revert().unwrap_or_revert();
            Some(value)
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