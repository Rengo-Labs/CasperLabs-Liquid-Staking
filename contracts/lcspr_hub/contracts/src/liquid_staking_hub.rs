#![no_std]
#![no_main]

/*#![feature(default_alloc_error_handler)]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");
*/

extern crate alloc;

mod entry_points;
mod helpers;

use entry_points::{
    ARG_AMOUNT, ARG_TMP_PURSE, ENTRY_POINT_INIT, PROTOCOL_FEE, PROTOCOL_LOCK_PERIOD,
};
use helpers::{
    get_delegation_purse, get_immediate_caller_address, get_key, get_withdraw_purse,
    set_delegation_purse, set_key, set_withdraw_purse,
};

use casper_contract::{
    contract_api::{runtime, system},
    unwrap_or_revert::UnwrapOrRevert,
};

use casper_types::{
    runtime_args, system::auction, CLValue, ContractHash, EntryPoints, Error, HashAddr, Key,
    PublicKey, RuntimeArgs, URef, U256, U512,
};

const CONTRACT_KEY_NAME: &str = "liquid_staking_hub";

// Named constants for Delegation / Undelegation arguments
const ARG_VALIDATOR: &str = "validator";
const ARG_DELEGATOR: &str = "delegator";

// Named Key Dictionaries
const WITHDRAW_INSTANCES_REGISTRY: &str = "withdraw_instances_registry";
const STAKERS_WITHDRAWALS_REGISTRY: &str = "stakers_withdrawals_registry";

// Named Keys
const KEY_DELEGATION_BALANCE: &str = "delegation_purse_balance";
const KEY_WITHDRAW_BALANCE: &str = "withdraw_purse_balance";

#[no_mangle]
pub extern "C" fn deposit() {
    
    // Get staker's temporary purse from pre_deposit contract
    let tmp_purse: URef = runtime::get_named_arg(ARG_TMP_PURSE);

    // Read staker's balance on temporary purse
    let cspr_amount: U512 = system::get_purse_balance(tmp_purse).unwrap_or_revert();
    let cspr_amount_u256: U256 = U256::from(cspr_amount.as_u128());

    // Get hub contract's purse to make delegations from
    let hub_delegation_purse: URef = get_delegation_purse();

    // Read balance of delegation purse
    let hub_delegation_balance: U512 =
        system::get_purse_balance(hub_delegation_purse).unwrap_or_revert();

    // Recieve CSPR from staker into hub delegation purse
    let _ = system::transfer_from_purse_to_purse(tmp_purse, hub_delegation_purse, cspr_amount, None);

    // Check balances after transfer
    let hub_delegation_balance_after: U512 =
        system::get_purse_balance(hub_delegation_purse).unwrap_or_revert();
    assert_eq!(
        hub_delegation_balance + cspr_amount,
        hub_delegation_balance_after
    );

    // Get account of the staker who called the contract
    let sender = get_immediate_caller_address().unwrap_or_revert();

    // Mint lCSPR for sender
    ERC20::default()
        .mint(sender, cspr_amount_u256)
        .unwrap_or_revert();

    // Update CSPR balance of Hub's delegation purse
    // TODO
    // Check if it's necessary to have that NamedKey
    // It's being set on regular user call
    set_key(KEY_DELEGATION_BALANCE, hub_delegation_balance_after);

    // Initiate delegation

    // Let user get rewards in lCSPR after delegation
}

// Initiate withdrawal of CSPR tokens from liquid_staking_protocol
#[no_mangle]
pub extern "C" fn withdraw() {
    // Amount of CSPR tokens to withdraw
    let cspr_amount: U512 = runtime::get_named_arg(ARG_AMOUNT);
    let cspr_amount_u256: U256 = U256::from(cspr_amount.as_u128());

    // Get account of a staker who called the contract
    let sender = get_immediate_caller_address().unwrap_or_revert();

    // Check balance of liquid staker
    let balance: U256 = ERC20::default().balance_of(sender);

    // Check Hub's Delegation purse balance
    let hub_main_purse = get_delegation_purse();
    let hub_purse_balance: U512 = system::get_purse_balance(hub_main_purse).unwrap_or_revert();

    if balance >= cspr_amount_u256 {
        
        // TODO
        // Call "burn" function of ERC20
        ERC20::default()
            .burn(sender, cspr_amount_u256)
            .unwrap_or_revert();

        // Initiate undelegation
        
        
    }

    
}

// TODO
// Move to "Reward_dispatcher"
#[no_mangle]
pub extern "C" fn claim() {
    // Read CSPR amount to claim
    let cspr_amount: U512 = runtime::get_named_arg(ARG_AMOUNT);

    // Get Hub's withdraw purse key
    let hub_withdraw_purse = get_withdraw_purse();

    // Read Hub's withdraw purse balance
    let hub_withdraw_balance: U512 =
        system::get_purse_balance(hub_withdraw_purse).unwrap_or_revert();

    // Get account of a staker who called claim method
    let liquid_staker = get_immediate_caller_address().unwrap_or_revert();

    // Get available CSPR tokens to claim
    let available_cspr: U512 = get_available_cspr_to_claim(liquid_staker);

    // Check that staker passed locked period
    if !lock_period_check(liquid_staker) {

        // Stop execution
    }

    // Check that staker has CSPR tokens to claim
    if !claim_token_check(liquid_staker, cspr_amount) {

        // Stop execution
    }

    // Check that amount to claim is equal or lower then balance of Hub's withdrawal purse
    if cspr_amount <= hub_withdraw_balance {
        // Transfer CSPR to staker
        system::transfer_from_purse_to_account(
            hub_withdraw_purse,
            *liquid_staker.as_account_hash().unwrap_or_revert(),
            cspr_amount,
            None,
        )
        .unwrap_or_revert();

        // Check balances after transfer
        let hub_withdraw_balance_after: U512 =
            system::get_purse_balance(hub_withdraw_purse).unwrap_or_revert();
        assert_eq!(
            hub_withdraw_balance - cspr_amount,
            hub_withdraw_balance_after
        );

        // Update CSPR balance for Hub's withdraw contract
        set_key(KEY_WITHDRAW_BALANCE, hub_withdraw_balance_after);
    }
    }
}

#[no_mangle]
fn call() {
    // Read PublicKey from runtime args
    let new_public_key: PublicKey = runtime::get_named_arg(PUBLIC_KEY);

    // Entry points
    let entry_points: EntryPoints = entry_points::hub_contract_entry_points();

    // Named keys
    let mut named_keys = NamedKeys::new();

    // Create WITHDRAW_INSTANCES dictionary uref
    let withdraw_instances_uref = storage::new_dictionary(WITHDRAW_INSTANCES_REGISTRY).unwrap_or_revert();

    // Get WITHDRAW_INSTANCES dictionary key
    let withdraw_instances_key = {
        runtime::remove_key(WITHDRAW_INSTANCES_REGISTRY);
        Key::from(withdraw_instances_uref)
    };
    
    // Add WITHDRAW_INSTANCES dictionary key to Named keys
    named_keys.insert(WITHDRAW_INSTANCES_REGISTRY.to_string(), withdraw_instances_key);

    // Create STAKERS_WITHDRAWALS dictionary uref
    let manager_uref = storage::new_dictionary(STAKERS_WITHDRAWALS_REGISTRY).unwrap_or_revert();

    // Get STAKERS_WITHDRAWALS dictionary key
    let manager_key = {
        runtime::remove_key(STAKERS_WITHDRAWALS_REGISTRY);
        Key::from(manager_uref)
    };

    // Add STAKERS_WITHDRAWALS dictionary key to Named keys
    named_keys.insert(STAKERS_WITHDRAWALS_REGISTRY.to_string(), withdraw_instances_key);

    // Install upgradable contract
    let (contract_hash, contract_version) = storage::new_contract(
        entry_points,
        Some(named_keys),
        Some(HASH_NAME.to_string()),
        Some(UREF_NAME.to_string()),
    );

    // Initialize contract
    // Set Hub's contract CSPR 'deposit' and 'withdrawal' purses
    let _: () = runtime::call_contract(contract_hash, ENTRY_POINT_INIT, RuntimeArgs::new());

    // TODO
    // Call versioned contract for ENTRY_POINT_INIT

}

#[no_mangle]
pub extern "C" fn init() {
    let value: Option<bool> = get_key("initialized");
    match value {
        Some(_) => {}
        None => {
            set_delegation_purse(system::create_purse());
            set_withdraw_purse(system::create_purse());
            set_key("initialized", true);
        }
    }
}

// TODO
// Implementation for next functions
// Access control

/// Administrative functions

// Set new lock_period value for Liquid Staking Hub contract
#[no_mangle]
pub extern "C" fn set_lock_period() {
    
    // Read new lock_period value from runtime args
    let new_lock_period: U512 = runtime::get_named_arg(PROTOCOL_LOCK_PERIOD);

    // Set new key value
    set_key(PROTOCOL_LOCK_PERIOD, new_lock_period);

}

// Set new protocol_fee value for Liquid Staking Hub contract
#[no_mangle]
pub extern "C" fn set_protocol_fee() {
    
    // Read new protocol_fee value from runtime args
    let new_protocol_fee: U512 = runtime::get_named_arg(PROTOCOL_FEE);

    // Set new key value
    set_key(PROTOCOL_FEE, new_protocol_fee);

}

#[no_mangle]
pub extern "C" fn add_whitelist_validators() {}

#[no_mangle]
pub extern "C" fn remove_whitelist_validators() {}

#[no_mangle]
fn distribute_rewards() {}

// #[no_mangle]
// pub extern "C" fn set_manual_validator_fraction() {
// }

// #[no_mangle]
// pub extern "C" fn manual_reward_distribution() {
// }

fn get_available_cspr_to_claim(Address: staker) -> U512 {

    // Read STAKERS_WITHDRAWALS_REGISTRY
    // liquid_staker -> withdrawa_instances
    get

    // Read WITHDRAW_INSTANCES_REGISTRY
    // Iterate withdrawa_instances -> Amount + Unlock_date
    // Calculate available amounts

    // Return available amount
}

fn request_cspr_withdrawal(Address: staker, U512: amount) {
    
    // Get STAKERS_WITHDRAWALS_REGISTRY URef
    let withdrawals_reg_uref: URef = get_uref(STAKERS_WITHDRAWALS_REGISTRY);

    // Get WITHDRAW_INSTANCES_REGISTRY URef
    let withd_instances_reg_uref: URef = get_uref(WITHDRAW_INSTANCES_REGISTRY);

    // Example of new URef -> Key -> NamedKey
    runtime::put_key(ALLOW_MINTING, storage::new_uref(allow_minting).into());
    
    let contract_whitelist = get_named_arg_with_user_errors::<Vec<ContractHash>>(
        ARG_CONTRACT_WHITELIST,
        NFTCoreError::MissingContractWhiteList,
        NFTCoreError::InvalidContractWhitelist,
    )
    .unwrap_or_revert();
    
        runtime::put_key(
        CONTRACT_WHITELIST,
        storage::new_uref(contract_whitelist).into(),
    );

    // Get current timestamp
    
    // Read protocol's lock period
    //

    // Put 
    sotrage::dictionary_put(WITHDRAW_INSTANCES_REGISTRY, staker, amount)
}

// TODO
// Add ApiError into fn args
fn get_uref(name: &str) -> URef {
    
    let key = runtime::get_key(name)
        .ok_or(ApiError::MissingKey)
        .unwrap_or_revert();
    key.try_into().unwrap_or_revert()

    // let ledger_seed_uref = *runtime::get_key(name)
    // .unwrap_or_revert_with(ApiError::MissingKey)
    //     .as_uref()
    //     .unwrap_or_revert();

}

// fn lock_period_check(Address: sender) -> bool {

    // Read DICT_LOCK_PERIODS for sender

    // Read PROTOCOL_LOCK_PERIOD

    // Get current block_time

    // Check that sender passed required lock_period
// }

// fn claim_token_check(Address: sender, U512: cspr_amount) {

    //
// }
