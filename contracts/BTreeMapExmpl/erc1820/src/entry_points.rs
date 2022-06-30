//! Contains definition of the entry points.
use alloc::{string::String, vec, vec::Vec};

use casper_types::{
    {CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Parameter, Key},
    bytesrepr::Bytes, account::AccountHash
};
use Address;
use constants::{GET_MANAGER_ENTRY_POINT, I_ID_RUNTIME_ARG_NAME, I_NAME_RUNTIME_ARG_NAME, IMPLEMENTS_ERC165_INTERFACE_ENTRY_POINT, IMPLEMENTS_ERC165_INTERFACE_NO_CACHE_ENTRY_POINT, INTERFACE_HASH_ENTRY_POINT, NEW_MANAGER_RUNTIME_ARG_NAME, SET_MANAGER_ENTRY_POINT, UPDATE_ERC165_CACHE_ENTRY_POINT};

use crate::constants::{
    ACCOUNT_RUNTIME_ARG_NAME, GET_INTERFACE_ENTRY_POINT, I_HASH_RUNTIME_ARG_NAME,
    IMPLEMENTER_RUNTIME_ARG_NAME, SET_INTERFACE_ENTRY_POINT
};

/// `get_manager`
pub fn get_manager() -> EntryPoint {
    EntryPoint::new(
        String::from(GET_MANAGER_ENTRY_POINT),
        vec![
            Parameter::new(ACCOUNT_RUNTIME_ARG_NAME, Address::cl_type())
        ],
        Address::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// `set_manager`
pub fn set_manager() -> EntryPoint {
    EntryPoint::new(
        String::from(SET_MANAGER_ENTRY_POINT),
        vec![
            Parameter::new(ACCOUNT_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(NEW_MANAGER_RUNTIME_ARG_NAME, Address::cl_type())
        ],
        CLType::Key,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// `get_interface_implementer`
pub fn get_interface_implementer() -> EntryPoint {
    EntryPoint::new(
        String::from(GET_INTERFACE_ENTRY_POINT),
        vec![
            Parameter::new(ACCOUNT_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(I_HASH_RUNTIME_ARG_NAME, String::cl_type())
        ],
        CLType::Key,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// `set_interface_implementer`
pub fn set_interface_implementer() -> EntryPoint {
    EntryPoint::new(
        String::from(SET_INTERFACE_ENTRY_POINT),
        vec![
            Parameter::new(ACCOUNT_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(I_HASH_RUNTIME_ARG_NAME, String::cl_type()),
            Parameter::new(IMPLEMENTER_RUNTIME_ARG_NAME, Address::cl_type())
        ],
        Address::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// `interface_hash`
pub fn interface_hash() -> EntryPoint {
    EntryPoint::new(
        String::from(INTERFACE_HASH_ENTRY_POINT),
        vec![
            Parameter::new(I_NAME_RUNTIME_ARG_NAME, String::cl_type())
        ],
        Bytes::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// `update_erc165_cache`
pub fn update_erc165_cache() -> EntryPoint {
    EntryPoint::new(
        String::from(UPDATE_ERC165_CACHE_ENTRY_POINT),
        vec![
            Parameter::new(ACCOUNT_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(I_ID_RUNTIME_ARG_NAME, Bytes::cl_type())
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// `implements_erc165_interface`
pub fn implements_erc165_interface() -> EntryPoint {
    EntryPoint::new(
        String::from(IMPLEMENTS_ERC165_INTERFACE_ENTRY_POINT),
        vec![
            Parameter::new(ACCOUNT_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(I_HASH_RUNTIME_ARG_NAME, Bytes::cl_type()),
            Parameter::new(IMPLEMENTER_RUNTIME_ARG_NAME, Address::cl_type())
        ],
        CLType::Bool,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// `implements_erc165_interfaceNoCache`
pub fn implements_erc165_interface_no_cache() -> EntryPoint {
    EntryPoint::new(
        String::from(IMPLEMENTS_ERC165_INTERFACE_NO_CACHE_ENTRY_POINT),
        vec![
            Parameter::new(ACCOUNT_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(I_HASH_RUNTIME_ARG_NAME, Bytes::cl_type()),
            Parameter::new(IMPLEMENTER_RUNTIME_ARG_NAME, Address::cl_type())
        ],
        CLType::Bool,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the default set of ERC20 token entry points.
pub fn default() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(set_interface_implementer());
    entry_points.add_entry_point(get_interface_implementer());
    entry_points.add_entry_point(set_manager());
    entry_points.add_entry_point(get_manager());
    entry_points.add_entry_point(interface_hash());
    entry_points
}