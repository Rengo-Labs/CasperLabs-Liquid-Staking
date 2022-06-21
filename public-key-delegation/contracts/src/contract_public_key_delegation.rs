mod key_pairs;

use key_pairs::get_new_public_key;

use std::str;
use alloc::string::String;

use casper_client::Error;
use casper_contract::{ contract_api::{runtime, system, storage}, unwrap_or_revert::UnwrapOrRevert };
// TODO
// Erase AccountHash, 
use casper_types::{
    PublicKey, EntryPoints, NamedKeys, AccountHash,
    RuntimeArgs, runtime_args, U512, system::auction,
};

const HASH_NAME: &str = "public_key_delegation_contract_hash";
const UREF_NAME: &str = "public_key_delegation_contract_uref";

const PUBLIC_KEY: &str = "contract_public_key";
const ACCOUNT_HASH: &str = "contract_account_hash";
const PUBLIC_KEY_HEX: &str = "contract_public_key_hex";
const PUBLIC_KEY: &str = "contract_public_key";
const CONTRACT_PURSE: &str = "contract_purse";
const INIT: &str = "contract_purse";

const ENTRY_POINT_INIT: &str = "initialize_contract";
const ENTRY_POINT_DELEGATE: &str = "delegate_to";
const ENTRY_POINT_UNDELEGATE: &str = "initialized";

pub extern "C" delegate_to() {
    
    // Get entry point args
    let validator: PublicKey = runtime::get_named_arg(auction::ARG_VALIDATOR);
    let amount: U512 = runtime::get_named_arg(auction::ARG_AMOUNT);

    // Get contract's public key from the context's NamedKeys
    let delegator: PublicKey = get_key(PUBLIC_KEY);
    
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

pub extern "C" undelegate_from() {
    
    // Get entry point args
    let validator: PublicKey = runtime::get_named_arg(auction::ARG_VALIDATOR);
    let amount: U512 = runtime::get_named_arg(auction::ARG_AMOUNT);

    // Get contract's public key from the context's NamedKeys
    let delegator: PublicKey = get_key(PUBLIC_KEY);
    
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

pub extern "C" initialize_contract() {
    
    // Check that conrtact is not initialized
    let value: Option<bool> = get_key(INIT);
    match value {
        Some(_) => {}
        None => {
            
            // Generate Contract's PublicKey, PublicKeyHex, AccountHash
            // Save values into NamedKeys
            set_contracts_public_key();
    
            // Create CSPR MainPurse for the contract
            // Save MainPurse into NamedKeys
            set_main_purse();
            
            // Make contract being initialized
            set_key(INIT, true);

        }
    }

}

fn set_contracts_public_key() {
    
    let public_key: PublicKey = get_new_public_key();
    set_key(PUBLIC_KEY, public_key);

    let account_hash: AccountHash = public_key.to_account_hash();
    set_key(ACCOUNT_HASH, account_hash);

    let public_key_hex: String = public_key.to_hex();
    set_key(PUBLIC_KEY_HEX, public_key_hex);

}

fn set_main_purse() {
    
    set_key(system::create_purse());

}

fn call() {
    
    // Entry points
    let entry_points: EntryPoints = get_entry_points();

    // Named keys
    let named_keys: NamedKeys = NamedKeys::new();

    // Install upgradable contract
    let (contract_hash, contract_version) = storage::new_contract(entry_points, named_keys, HASH_NAME, UREF_NAME);

    // Runtime arguments for "initialize_contract" function
    let runtime_arguments: RuntimeArgs = RuntimeArgs::new();

    // Initialize contract
    // Set CSPR MainPurse and PublicKey for the contract
    // TODO call versioned contract
    let _: () = runtime::call_contract(contract_hash, ENTRY_POINT_INIT, runtime_arguments);
    
}

fn get_entry_points() -> EntryPoints {

    // Create contract's entry points
    let mut entry_points = EntryPoints::new();
    
    // Entry point: initialize_contract
    entry_points.add_entry_point(
        EntryPoint::new(
            String::from(ENTRY_POINT_INIT),
            vec![],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    );
    
    // Entry point: delegate_to
    entry_points.add_entry_point(
        EntryPoint::new(
            String::from(ENTRY_POINT_DELEGATE),
            vec![
                Parameter::new(auction::ARG_VALIDATOR, PublicKey::cl_type()),
                Parameter::new(AMOUNT_KEY_NAME, U512::cl_type()),
                ],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    );
    
    // Entry point: undelegate_from
    entry_points.add_entry_point(
        EntryPoint::new(
            String::from(ENTRY_POINT_UNDELEGATE),
            vec![
                Parameter::new(auction::ARG_VALIDATOR, PublicKey::cl_type()),
                Parameter::new(AMOUNT_KEY_NAME, U512::cl_type()),
                ],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    );
    
    // Return entry points
    entry_points

}

fn get_key<T: FromBytes + CLTyped>(name: &str) -> Option<T> {
    match runtime::get_key(name) {
        None => None,
        Some(value) => {
            let key = value.try_into().unwrap_or_revert();
            let result = storage::read(key).unwrap_or_revert().unwrap_or_revert();
            Some(result)
        }
    }
}

fn set_key<T: ToBytes + CLTyped>(name: &str, value: T) {
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

// fn get_main_purse() -> URef {
    
//     let contract_main_purse_key = runtime::get_key(CONTRACT_PURSE).unwrap_or_revert();
//     let contract_main_purse = contract_main_purse_key.as_uref().unwrap_or_revert();
//     *contract_main_purse

// }