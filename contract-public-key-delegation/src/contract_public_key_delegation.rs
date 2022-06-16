use std::str;

use casper_client::Error;
use casper_types::{AsymmetricType, PublicKey, EntryPoints, NamedKeys, RuntimeArgs};
use contract_utils::{Address, data::get_caller_address};

pub const HASH_NAME: &str = "public_key_contract_delegation_hash";
pub const UREF_NAME: &str = "public_key_contract_delegation_uref";

fn public_key_for_contract() -> PublicKey {
    // 
    let hex_public_key: 
    let public_key = PublicKey::from_hex(&hex_public_key).map_err(|error| {
        eprintln!("Can't parse {} as a public key: {}", hex_public_key, error);
        Error::FailedToParseKey
    })?;

    public_key
}

pub fn delegate_to(validator: PublicKey) {
    
    // Call delegation function

}

pub fn undelegate_from(validator: PublicKey) {
    
    // Call undelegation function

}

pub fn initialize_contract() {
    
    // Get PublicKey for the contract
    
    // Save Contract's PublicKey into NamedKeys

    // Create CSPR MainPurse for the contract

    // Save MainPurse into NamedKeys

}

fn call() {
    
    let caller: Address = get_caller_address();
    
    // Entry points
    let entry_points: EntryPoints = get_entry_points();

    // Named keys
    let named_keys: NamedKeys = NamedKeys::new();

    // Install upgradable contract
    let (contract_hash, contract_version) = storage::new_contract(entry_points, named_keys, HASH_NAME, UREF_NAME);

    // Runtime arguments for "initialize_contract" function
    let runtime_arguments: RuntimeArgs = RuntimeArgs::new();

    // "initialize_contract" function call
    // To set CSPR MainPurse and PublicKey for the contract
    // TODO call versioned contract
    let _: () = runtime::call_contract(contract_hash, "initialize_contract", runtime_arguments);
    
}

fn get_entry_points() -> EntryPoints {

    // Create contract's entry points
    let mut entry_points = EntryPoints::new();
    
    // Entry point: initialize_contract
    entry_points.add_entry_point(
        EntryPoint::new(
            String::from("initialize_contract"),
            vec![],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    );
    
    // Entry point: delegate_to
    entry_points.add_entry_point(
        EntryPoint::new(
            String::from("delegate_to"),
            vec![Parameter::new("validator", PublicKey::cl_type()),],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    );
    
    // Entry point: undelegate_from
    entry_points.add_entry_point(
        EntryPoint::new(
            String::from("undelegate_from"),
            vec![Parameter::new("validator", PublicKey::cl_type()),],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    );
    
    // Return entry points
    entry_points

}