use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use casper_types::bytesrepr::{Bytes, FromBytes, ToBytes};
use casper_types::{CLType, CLTyped, URef};
use casper_types::account::AccountHash;
use ::{Address, detail};
use casper_contract::{
    contract_api::{runtime, storage}
};
use address::Address::Account;
use constants::IMPLEMENTERS_REGISTRY_KEY_NAME;
use detail::get_immediate_caller_address;

#[inline]
pub(crate) fn implementers_registry() -> URef {
    detail::get_uref(IMPLEMENTERS_REGISTRY_KEY_NAME)
}

pub fn create_or_update_implementer(
    implementer_uref: URef,
    account: Address,
    interface_hash: String,
    implementer: Address
) {
    let hash_string: String = to_str(account);

    let mut implementers: BTreeMap<String, Address> = storage::dictionary_get(
        implementer_uref,
        hash_string.as_str()
    ).unwrap_or_default().unwrap_or_default();

    implementers.insert(interface_hash, implementer);

    storage::dictionary_put(
        implementer_uref,
        hash_string.as_str(),
        implementers);
}

pub fn get_implementer(implementer_uref: URef, account: Address, interface_hash: String) -> Address {
    let hash_string = to_str(account);
    let implements: BTreeMap<String, Address> = storage::dictionary_get(
        implementer_uref,
        hash_string.as_str()
    ).unwrap_or_default().unwrap_or_default();

    *implements.get(&interface_hash).unwrap()
}

pub(crate) fn to_str(owner: Address) -> String {
    base64::encode(&owner.to_bytes().unwrap())
}

fn encode(data: String) -> String {
    base64::encode(data)
}

fn decode(data: String) -> String {
    String::from_vec(base64::decode(data).unwrap()).unwrap_or_default().0
}