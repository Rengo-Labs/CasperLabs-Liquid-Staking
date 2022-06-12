#![no_std]
#![no_main]

extern crate alloc;

pub mod data;
// TODO
// Migrate "helper.rs" functions into "data.rs" of utils
pub mod helpers;
pub mod constants;
pub mod entry_points;

pub use constants::{
    LIQUID_STAKING_HUB_HASH_RUNTIME_ARG_NAME, LIQUID_STAKING_HUB_CONTRACT_PACKAGE_HASH_RUNTIME_ARG_NAME,
    LIQUID_STAKING_HUB_CONTRACT_VERSION_RUNTIME_ARG_NAME,
    VALIDATORS_TO_WHITELIST_ARG_NAME, ADMINS_TO_SET_ARG_NAME, VALIDATORS_WHITELIST_HASH_NAME,
    VALIDATORS_WHITELIST_UREF_NAME, VALIDATORS_WHITELIST_CONTRACT_KEY_NAME,
    
    // TODO
    // Check if used
    VALIDATORS_WHITELIST_DICTIONARY_KEY_NAME,
    VALIDATORS_UNSTAKE_DICTIONARY_KEY_NAME, OWNER_KEY_NAME
};
pub use helpers::{ get_immediate_caller_address, get_key, get_main_purse, set_key, set_main_purse };


// TODO
// Clean out unnecessary crates
use casper_erc20::Error;
use casper_contract::{ contract_api::{runtime, storage, system}, unwrap_or_revert::UnwrapOrRevert };
use casper_types::{
    runtime_args, system::auction, PublicKey,
    ContractHash, HashAddr, Key, RuntimeArgs,
    CLValue, URef, U256, U512, EntryPoints,
    ContractPackageHash, ApiError, contracts::NamedKeys
};
use contract_utils::{ AdminControl, ContractContext, ContractStorage, Dict, Address };

