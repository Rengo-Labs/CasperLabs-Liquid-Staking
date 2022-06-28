mod error;
mod address;

use alloc::string::{String, ToString};
use core::convert::TryInto;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    ApiError, CLTyped, Key, URef,
    system::CallStackElement,
};

use error::Error;
use address::Address;


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

/// 

/// ERC20 Data helpers
/// 
/// Gets [`URef`] under a name.
pub fn get_uref(name: &str) -> URef {
    let key = runtime::get_key(name)
        .ok_or(ApiError::MissingKey)
        .unwrap_or_revert();
    key.try_into().unwrap_or_revert()
}

/// Reads value from a named key.
pub fn read_from<T>(name: &str) -> T
where
    T: FromBytes + CLTyped,
{
    let uref = get_uref(name);
    let value: T = storage::read(uref).unwrap_or_revert().unwrap_or_revert();
    value
}

/// Gets the immediate call stack element of the current execution.
fn get_immediate_call_stack_item() -> Option<CallStackElement> {
    let call_stack = runtime::get_call_stack();
    call_stack.into_iter().rev().nth(1)
}

/// Returns address based on a [`CallStackElement`].
///
/// For `Session` and `StoredSession` variants it will return account hash, and for `StoredContract`
/// case it will use contract hash as the address.
fn call_stack_element_to_address(call_stack_element: CallStackElement) -> Address {
    match call_stack_element {
        CallStackElement::Session { account_hash } => Address::from(account_hash),
        CallStackElement::StoredSession { account_hash, .. } => {
            // Stored session code acts in account's context, so if stored session wants to interact
            // with an ERC20 token caller's address will be used.
            Address::from(account_hash)
        }
        CallStackElement::StoredContract {
            contract_package_hash,
            ..
        } => Address::from(contract_package_hash),
    }
}

/// Gets the immediate session caller of the current execution.
///
/// This function ensures that only session code can execute this function, and disallows stored
/// session/stored contracts.
pub fn get_immediate_caller_address() -> Result<Address, Error> {
    get_immediate_call_stack_item()
        .map(call_stack_element_to_address)
        .ok_or(Error::InvalidContext)
}

/// Gets the caller address which is stored on the top of the call stack.
///
/// This is similar to what [`runtime::get_caller`] does but it also supports stored contracts.
pub fn get_caller_address() -> Result<Address, Error> {
    let call_stack = runtime::get_call_stack();
    let top_of_the_stack = call_stack
        .into_iter()
        .rev()
        .next()
        .ok_or(Error::InvalidContext)?;
    let address = call_stack_element_to_address(top_of_the_stack);
    Ok(address)
}