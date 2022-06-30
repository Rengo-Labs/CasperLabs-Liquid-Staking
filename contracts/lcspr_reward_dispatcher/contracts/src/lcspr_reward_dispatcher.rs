#![no_std]
#![no_main]

extern crate alloc;

mod entry_points;

use entry_points::ENTRY_POINT_INIT;

use casper_contract::{
    contract_api::{runtime, system},
    unwrap_or_revert::UnwrapOrRevert,
};

use casper_types::{
    runtime_args, system::auction, CLValue, ContractHash, EntryPoints, Error, HashAddr, Key,
    PublicKey, RuntimeArgs, URef, U256, U512,
};

const HUB_CONTRACT_HASH: &str = "hub_contract_hash";
const LCSPR_TOKEN_HASH: &str = "lcspr_token_hash";
const INIT: &str = "initialized";

#[no_mangle]
fn call() {
    
    // Read runtime args
    let hub_contract_hash: ContractHash = runtime::get_named_arg(HUB_CONTRACT_HASH);
    let lcspr_token_hash: ContractHash = runtime::get_named_arg(LCSPR_TOKEN_HASH);

    // Entry points
    let entry_points: EntryPoints = entry_points::reward_dispatcher_entry_points();

    // Named keys
    let mut named_keys = NamedKeys::new();

    // Install upgradable contract
    let (contract_hash, contract_version) = storage::new_contract(
        entry_points,
        Some(named_keys),
        None,
        None,
    );

    // Initialize contract
    let _: () = runtime::call_contract(contract_hash, ENTRY_POINT_INIT,
        runtime_args! {
            HUB_CONTRACT_HASH => hub_contract_hash,
            LCSPR_TOKEN_HASH => lcspr_token_hash,
        }
    );

}

#[no_mangle]
pub extern "C" fn init() {
    
    let value: Option<bool> = get_key(INIT);
    match value {
        Some(_) => {}
        None => {

            let hub_contract_hash: ContractHash = runtime::get_named_arg(HUB_CONTRACT_HASH);
            let lcspr_token_hash: ContractHash = runtime::get_named_arg(LCSPR_TOKEN_HASH);

            set_key(HUB_CONTRACT_HASH, hub_contract_hash);
            set_key(LCSPR_TOKEN_HASH, lcspr_token_hash);
            set_key(INIT, true);
        }
    }

}

#[no_mangle]
pub extern "C" fn claim_cspr() {
    
    // Let's staker to claim CSPR tokens after withdrawal initiated
    // And lock period passed

    // Migrate from "Hub"

}

#[no_mangle]
pub extern "C" fn update_config() {
    
    // Update NamedKey values

    // Available only for Owner, Hub, DAO

}

#[no_mangle]
pub extern "C" fn dispatch_claims() {
    
    // Send all available to claim CSPR to stakers

    // Available only for Owner, Hub, DAO

}