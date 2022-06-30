mod constants;

use alloc::vec::Vec;
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{ContractPackageHash, Key};
use contract_utils::{get_key, set_key, Dict};
use constants::{
    VALIDATORS_WHITELIST_DICTIONARY_KEY_NAME, VALIDATORS_UNSTAKE_LIST_DICTIONARY_KEY_NAME,
    OWNER, VALIDATORS_WHITELIST_HASH_NAME, VALIDATORS_WHITELIST_PACKAGE_HASH_NAME
};

pub struct Whitelists {
    dict: Dict,
}

impl Whitelists {
    pub fn instance() -> Whitelists {
        Whitelists {
            dict: Dict::instance(VALIDATORS_WHITELIST_DICTIONARY_KEY_NAME),
        }
    }

    pub fn init() {
        Dict::init(VALIDATORS_WHITELIST_DICTIONARY_KEY_NAME)
    }

    pub fn get(&self, owner: &Key) -> Key {
        match self.dict.get_by_key(owner) {
            Some(whitelist) => whitelist,
            None => Key::from_formatted_str(
                "account-hash-0000000000000000000000000000000000000000000000000000000000000000",
            )
            .unwrap(),
        }
    }

    pub fn set(&self, owner: &Key, value: Key) {
        self.dict.set_by_key(owner, value);
    }
}
pub struct UnstakeList {
    dict: Dict,
}

impl UnstakeList {
    pub fn instance() -> UnstakeList {
        UnstakeList {
            dict: Dict::instance(VALIDATORS_UNSTAKE_LIST_DICTIONARY_KEY_NAME),
        }
    }

    pub fn init() {
        Dict::init(VALIDATORS_UNSTAKE_LIST_DICTIONARY_KEY_NAME)
    }

    pub fn get(&self, owner: &Key) -> Key {
        match self.dict.get_by_key(owner) {
            Some(whitelist) => whitelist,
            None => Key::from_formatted_str(
                "account-hash-0000000000000000000000000000000000000000000000000000000000000000",
            )
            .unwrap(),
        }
    }

    pub fn set(&self, owner: &Key, value: Key) {
        self.dict.set_by_key(owner, value);
    }
}

pub fn set_hash(contract_hash: Key) {
    set_key(VALIDATORS_WHITELIST_HASH_NAME, contract_hash);
}

pub fn get_hash() -> Key {
    get_key(VALIDATORS_WHITELIST_HASH_NAME).unwrap_or_revert()
}

pub fn set_package_hash(package_hash: ContractPackageHash) {
    set_key(VALIDATORS_WHITELIST_PACKAGE_HASH_NAME, package_hash);
}

pub fn get_package_hash() -> ContractPackageHash {
    get_key(VALIDATORS_WHITELIST_PACKAGE_HASH_NAME).unwrap_or_revert()
}

pub fn set_owner(owner: Key) {
    set_key(OWNER, owner);
}

pub fn get_owner() -> Key {
    match get_key(OWNER) {
        Some(owner) => owner,
        None => Key::from_formatted_str(
            "account-hash-0000000000000000000000000000000000000000000000000000000000000000",
        )
        .unwrap(),
    }
}