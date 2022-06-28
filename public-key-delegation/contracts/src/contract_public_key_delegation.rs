#![no_main]

#![feature(default_alloc_error_handler)]

extern crate alloc;

mod entry_points;
mod data;

use std::str;

use entry_points::{ ENTRY_POINT_INIT, CONTRACT_PUB_KEY, CONTRACT_MAIN_PURSE };
use data::{ get_key, set_key };

use casper_contract::{ contract_api::{runtime, system, storage}, unwrap_or_revert::UnwrapOrRevert };
use casper_types::{
    PublicKey, NamedKey, EntryPoints,
    RuntimeArgs, runtime_args, U512, system::auction,
};

const HASH_NAME: &str = "public_key_delegation_contract_hash";
const UREF_NAME: &str = "public_key_delegation_contract_uref";
const INIT: &str = "initialized";

pub extern "C" fn delegate_to() {
    
    // Get entry point args
    let validator: PublicKey = runtime::get_named_arg(auction::ARG_VALIDATOR);
    let amount: U512 = runtime::get_named_arg(auction::ARG_AMOUNT);

    // Get contract's public key from the context's NamedKeys
    let delegator: PublicKey = get_key(CONTRACT_PUB_KEY).unwrap_or_revert();
    
    // Call delegation function
    delegate(delegator, validator, amount);

}

fn delegate(delegator: PublicKey, validator: PublicKey, amount: U512) {
    let contract_hash = system::get_auction();
    let args = runtime_args! {
        auction::ARG_DELEGATOR => delegator,
        auction::ARG_VALIDATOR => validator,
        auction::ARG_AMOUNT => amount,
    };
    runtime::call_contract::<U512>(contract_hash, auction::METHOD_DELEGATE, args);
}

pub extern "C" fn undelegate_from() {
    
    // Get entry point args
    let validator: PublicKey = runtime::get_named_arg(auction::ARG_VALIDATOR);
    let amount: U512 = runtime::get_named_arg(auction::ARG_AMOUNT);

    // Get contract's public key from the context's NamedKeys
    let delegator: PublicKey = get_key(CONTRACT_PUB_KEY).unwrap_or_revert();
    
    // Call delegation function
    undelegate(delegator, validator, amount);

}

fn undelegate(delegator: PublicKey, validator: PublicKey, amount: U512) {
    let contract_hash = system::get_auction();
    let args = runtime_args! {
        auction::ARG_DELEGATOR => delegator,
        auction::ARG_VALIDATOR => validator,
        auction::ARG_AMOUNT => amount,
    };
    let _amount: U512 = runtime::call_contract(contract_hash, auction::METHOD_UNDELEGATE, args);
}

pub extern "C" fn initialize_contract() {
    
    // Check that conrtact is not initialized
    let value: Option<bool> = get_key(INIT);
    match value {
        Some(_) => {}
        None => {
            
            // Read PublicKey from runtime args
            let new_public_key: PublicKey = runtime::get_named_arg(CONTRACT_PUB_KEY);

            // Save values into NamedKeys
            set_key(CONTRACT_PUB_KEY, new_public_key);
    
            // Create CSPR MainPurse for the contract
            // Save MainPurse into NamedKeys
            set_key(CONTRACT_PURSE, system::create_purse());
            
            // Make contract being initialized
            set_key(INIT, true);

        }
    }

}

pub extern "C" fn set_public_key() {
    
    // TODO
    // Check ownership

    // Read PublicKey from runtime args
    let new_public_key: PublicKey = runtime::get_named_arg(CONTRACT_PUB_KEY);

    // Save values into NamedKeys
    set_key(CONTRACT_PUB_KEY, new_public_key);

}

pub extern "C" fn set_main_purse() {

    // TODO
    // Check ownership

    // Read Purse to set from runtime args
    let new_contract_purse: URef = runtime::get_named_arg(CONTRACT_MAIN_PURSE);
    
    set_key(CONTRACT_MAIN_PURSE, system::create_purse());

}

fn call() {

    // Read PublicKey from runtime args
    let new_public_key: PublicKey = runtime::get_named_arg(PUBLIC_KEY);
    
    // Entry points
    let entry_points: EntryPoints = entry_points::get_entry_points();

    // Named keys
    let named_keys: NamedKey = NamedKey::default();

    // Install upgradable contract
    let (contract_hash, contract_version) = storage::new_contract(entry_points, Some(named_keys), Some(HASH_NAME.to_string()), Some(UREF_NAME.to_string()));

    // Runtime arguments for "initialize_contract" function
    // let runtime_arguments: RuntimeArgs = RuntimeArgs::new();
    let args = runtime_args! {
        CONTRACT_PUB_KEY => new_public_key,
    };

    // Initialize contract
    // Set PublicKey for the contract
    let _: () = runtime::call_contract(contract_hash, ENTRY_POINT_INIT, args);

    // TODO 
    // Call versioned contract instead
    
}

// fn get_main_purse() -> URef {
    
//     let contract_main_purse_key = runtime::get_key(CONTRACT_MAIN_PURSE).unwrap_or_revert();
//     let contract_main_purse = contract_main_purse_key.as_uref().unwrap_or_revert();
//     *contract_main_purse

// }