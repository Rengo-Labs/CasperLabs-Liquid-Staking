#![no_std]
#![no_main]

/*#![feature(default_alloc_error_handler)]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");
*/

extern crate alloc;

mod entry_points;
mod helpers;

use helpers::{
    get_immediate_caller_address, get_key, set_key,
    get_delegation_purse, set_delegation_purse,
    get_withdraw_purse, set_withdraw_purse,
};

use casper_contract::{
    contract_api::{runtime, system},
    unwrap_or_revert::UnwrapOrRevert,
};

use casper_types::{
    runtime_args, system::auction, PublicKey,
    ContractHash, HashAddr, Key, RuntimeArgs,
    CLValue, URef, U256, U512, EntryPoints,
    Error
};

const CONTRACT_KEY_NAME: &str = "liquid_staking_hub";

// Named constants for Delegation / Undelegation arguments
const ARG_AMOUNT: &str = "amount";
const ARG_VALIDATOR: &str = "validator";
const ARG_DELEGATOR: &str = "delegator";
const ARG_TMP_PURSE: &str = "tmp_purse";

// Named Keys
const KEY_DELEGATION_BALANCE: &str = "delegation_purse_balance";
const KEY_WITHDRAW_BALANCE: &str = "withdraw_purse_balance";

// Named constant for method `delegate`.
const METHOD_DELEGATE: &str = "delegate";
// Named constant for method `undelegate`.
const METHOD_UNDELEGATE: &str = "undelegate";

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
    assert_eq!(hub_delegation_balance + cspr_amount, hub_delegation_balance_after);

    // Get account of the staker who called the contract
    let sender = get_immediate_caller_address().unwrap_or_revert();

    // Mint lCSPR for sender 
    // TODO
    // Call "mint" function of ERC20
    ERC20::default()
        .mint(sender, cspr_amount_u256)
        .unwrap_or_revert();

    // Update CSPR balance of Hub's delegation purse
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

    let hub_main_purse = get_delegation_purse();
    let hub_purse_balance: U512 =
        system::get_purse_balance(hub_main_purse).unwrap_or_revert();

    if balance >= cspr_amount_u256 {
        
        // TODO
        // Call "burn" function of ERC20
        ERC20::default()
            .burn(sender, cspr_amount_u256)
            .unwrap_or_revert();

    }

    if cspr_amount <= hub_purse_balance {
        
        system::transfer_from_purse_to_account(
            hub_main_purse,
            *sender.as_account_hash().unwrap_or_revert(),
            cspr_amount,
            None,
        )
        .unwrap_or_revert();

        let hub_purse_balance_after: U512 =
            system::get_purse_balance(hub_main_purse).unwrap_or_revert();
        assert_eq!(hub_purse_balance - cspr_amount, hub_purse_balance_after);

        // Update CSPR balance for Hub contract
        set_key(KEY_CSPR_BALANCE, hub_purse_balance_after);
    }
}

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
    let sender = get_immediate_caller_address().unwrap_or_revert();

    // Check that staker passed locked period

    // Checl that staker has CSPR tokens to claim
    
    // Check that amount to claim is equal or lower then balance of Hub's withdrawal purse
    if cspr_amount <= hub_withdraw_balance {
        
        // Transfer CSPR to staker
        system::transfer_from_purse_to_account(
            hub_withdraw_purse,
            *sender.as_account_hash().unwrap_or_revert(),
            cspr_amount,
            None,
        )
        .unwrap_or_revert();

        // Check balances after transfer
        let hub_withdraw_balance_after: U512 =
            system::get_purse_balance(hub_withdraw_purse).unwrap_or_revert();
        assert_eq!(hub_withdraw_balance - cspr_amount, hub_withdraw_balance_after);

        // Update CSPR balance for Hub's withdraw contract
        set_key(KEY_WITHDRAW_BALANCE, hub_withdraw_balance_after);
    }

}

// Function call:
// delegate(delegator, validator, amount);
#[no_mangle]
fn delegate(delegator: PublicKey, validator: PublicKey, amount: U512) -> U512 {
    let contract_hash = system::get_auction();
    let args = runtime_args! {
        auction::ARG_DELEGATOR => delegator,
        auction::ARG_VALIDATOR => validator,
        auction::ARG_AMOUNT => amount,
    };
    
    // Adds a new delegator to delegators or increases its current stake. If the target validator
    // is missing, the function call returns an error and does nothing.
    //
    // The function transfers motes from the source purse to the delegator's bonding purse.
    //
    // This entry point returns the number of tokens currently delegated to a given validator.
    let staked_amount: U512 = runtime::call_contract(contract_hash, METHOD_DELEGATE, args);

    staked_amount
}

// Function call:
// undelegate(delegator, validator, amount);
#[no_mangle]
fn undelegate(delegator: PublicKey, validator: PublicKey, amount: U512) -> U512 {
    let contract_hash = system::get_auction();
    let args = runtime_args! {
        auction::ARG_DELEGATOR => delegator,
        auction::ARG_VALIDATOR => validator,
        auction::ARG_AMOUNT => amount,
    };
    
    // Removes specified amount of motes (or the value from the collection altogether, if the
    // remaining amount is 0) from the entry in delegators map for given validator and creates a
    // new unbonding request to the queue.
    //
    // The arguments are the delegator's key, the validator's key, and the amount.
    //
    // Returns the remaining bid amount after the stake was decreased.
    let staked_amount: U512 = runtime::call_contract(contract_hash, METHOD_UNDELEGATE, args);
    
    staked_amount
}

#[no_mangle]
fn call() {
    
    let entry_points: EntryPoints = entry_points::hub_contract_entry_points();

    // TODO
    // Install custom upgradable contract

    let key: Key = runtime::get_key(CONTRACT_KEY_NAME).unwrap_or_revert();
    let hash: HashAddr = key.into_hash().unwrap_or_revert();
    let contract_hash = ContractHash::new(hash);

    // "init" function call
    // To set main CSPR purse of "Hub" contract
    let _: () = runtime::call_contract(contract_hash, "init", RuntimeArgs::new());

}

// TODO
// Implement Access control
// Only for "liquid_staking_hub"
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

// Administrative functions
#[no_mangle]
pub extern "C" fn set_lock_period() {

}

#[no_mangle]
pub extern "C" fn set_protocol_fee() {

}

#[no_mangle]
pub extern "C" fn add_whitelist_validators() {

}

#[no_mangle]
pub extern "C" fn remove_whitelist_validators() {

}

#[no_mangle]
pub extern "C" fn set_manual_validator_fraction() {

}

#[no_mangle]
pub extern "C" fn manual_reward_distribution() {

}

#[no_mangle]
fn distribute_rewards() {
    
}