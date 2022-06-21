mod key_pairs;

use key_pairs::get_new_public_key;

use std::str;
use alloc::string::String;

use casper_client::Error;
use casper_contract::{ contract_api::{runtime, system, storage}, unwrap_or_revert::UnwrapOrRevert };
use casper_types::{
    AsymmetricType, PublicKey, EntryPoints, NamedKeys,
    RuntimeArgs, runtime_args, U512, system::auction,
    AccountHash, 
};
use contract_utils::{Address, data::get_caller_address};

pub const HASH_NAME: &str = "public_key_delegation_contract_hash";
pub const UREF_NAME: &str = "public_key_delegation_contract_uref";
pub const PUBLIC_KEY: &str = "contract_public_key";
pub const ACCOUNT_HASH: &str = "contract_account_hash";
pub const PUBLIC_KEY_HEX: &str = "contract_public_key_hex";

fn public_key_for_contract() -> PublicKey {
    
    // Options to create `PublicKey`

    // pub fn from_bytes(bytes: &[u8]) -> Result<PublicKey, SignatureError>
    // Construct a PublicKey from a slice of bytes.
    // The caller is responsible for ensuring that the bytes passed into this method
    // actually represent a curve25519_dalek::curve::CompressedEdwardsY
    // and that said compressed point is actually a point on the curve.
    let bytes_curve25519: &[u8] = 
    let public_key_1 = from_bytes(bytes_curve25519);
    
    let public_key: PublicKey = get_new_public_key();

    /*
    let hex_public_key: 
    let public_key = PublicKey::from_hex(&hex_public_key).map_err(|error| {
        eprintln!("Can't parse {} as a public key: {}", hex_public_key, error);
        Error::FailedToParseKey
    })?;
    */

    let account_hash: AccountHash = public_key.to_account_hash();
    set_key(ACCOUNT_HASH, account_hash);

    let public_key_hex: String = public_key.to_hex();
    set_key(PUBLIC_KEY_HEX, public_key_hex);



    public_key
}

pub extern "C" delegate_to(validator: PublicKey) {
    
    // Get entry point args
    let validator: PublicKey = runtime::get_named_arg(ARG_VALIDATOR);
    let amount: U512 = runtime::get_named_arg(ARG_AMOUNT);

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
    let validator: PublicKey = runtime::get_named_arg(ARG_VALIDATOR);
    let amount: U512 = runtime::get_named_arg(ARG_AMOUNT);

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

pub fn initialize_contract() {
    
    // Get PublicKey for the contract
    
    // Save Contract's PublicKey into NamedKeys

    // Create CSPR MainPurse for the contract
    let value: Option<bool> = get_key("initialized");
    match value {
        Some(_) => {}
        None => {
            set_main_purse(system::create_purse());
            set_key("initialized", true);
        }
    }

    // TODO
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
