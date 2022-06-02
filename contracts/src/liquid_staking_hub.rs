#![no_std]
#![no_main]

/*#![feature(default_alloc_error_handler)]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");
*/

extern crate alloc;

mod entry_points;
mod helpers;

use crate::helpers::{ get_immediate_caller_address, get_key, get_main_purse, set_key, set_main_purse };

/* 
use alloc::string::String;
use core::panic::PanicInfo;
*/

use casper_contract::{
    contract_api::{runtime, system},
    unwrap_or_revert::UnwrapOrRevert,
};

// TODO
// Filter out unnecessary constatns and types
use casper_erc20::{
    constants::{
        ADDRESS_RUNTIME_ARG_NAME, AMOUNT_RUNTIME_ARG_NAME, DECIMALS_RUNTIME_ARG_NAME,
        NAME_RUNTIME_ARG_NAME, OWNER_RUNTIME_ARG_NAME, RECIPIENT_RUNTIME_ARG_NAME,
        SPENDER_RUNTIME_ARG_NAME, SYMBOL_RUNTIME_ARG_NAME, TOTAL_SUPPLY_RUNTIME_ARG_NAME,
    },
    Address, ERC20,
};
use casper_types::{
    runtime_args, system::auction, PublicKey,
    ContractHash, HashAddr, Key, RuntimeArgs,
    CLValue, URef, U256, U512, EntryPoints
};

const CONTRACT_KEY_NAME: &str = "liquid_staking_hub";

/*
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
*/

#[no_mangle]
pub extern "C" fn deposit() {
    
    // Get staker's temporary purse from pre_deposit contract 
    let tmp_purse: URef = runtime::get_named_arg("tmp_purse");

    let cspr_amount: U512 = system::get_purse_balance(tmp_purse).unwrap_or_revert();
    let cspr_amount_u256: U256 = U256::from(cspr_amount.as_u128());

    let contract_main_purse: URef = get_main_purse();

    let main_purse_balance: U512 =
        system::get_purse_balance(contract_main_purse).unwrap_or_revert();

    // Recieve CSPR from staker into Hub contract
    let _ = system::transfer_from_purse_to_purse(tmp_purse, contract_main_purse, cspr_amount, None);

    let main_purse_balance_after: U512 =
        system::get_purse_balance(contract_main_purse).unwrap_or_revert();
    assert_eq!(main_purse_balance + cspr_amount, main_purse_balance_after);

    // Get account of the staker who called the contract
    let sender = get_immediate_caller_address().unwrap_or_revert();

    // TODO
    // Call "mint" function of ERC20

    // Issue CSWAP tokens to the staker
    ERC20::default()
        .mint(sender, cspr_amount_u256)
        .unwrap_or_revert();

    // Update CSPR balance for Hub contract
    set_key("cspr_balance", main_purse_balance_after,);
    
}

#[no_mangle]
pub extern "C" fn withdraw() {
    
    // Amount of CSWAP tokens to withdraw
    let cspr_amount: U512 = runtime::get_named_arg("cspr_amount");
    let cspr_amount_u256: U256 = U256::from(cspr_amount.as_u128());

    // Get account of a staker who called the contract
    let sender = get_immediate_caller_address().unwrap_or_revert();

    let balance: U256 = ERC20::default().balance_of(sender);

    let contract_main_purse = get_main_purse();
    let main_purse_balance: U512 =
        system::get_purse_balance(contract_main_purse).unwrap_or_revert();

    if balance >= cspr_amount_u256 && cspr_amount <= main_purse_balance {
        system::transfer_from_purse_to_account(
            contract_main_purse,
            *sender.as_account_hash().unwrap_or_revert(),
            cspr_amount,
            None,
        )
        .unwrap_or_revert();
        
        // TODO
        // Call "mint" function of ERC20
    
        ERC20::default()
            .burn(sender, cspr_amount_u256)
            .unwrap_or_revert();

        let main_purse_balance_after: U512 =
            system::get_purse_balance(contract_main_purse).unwrap_or_revert();
        assert_eq!(main_purse_balance - cspr_amount, main_purse_balance_after);

        // Update CSPR balance for Hub contract
        set_key("cspr_balance", main_purse_balance_after,);
    }
}

// TODO
// Adopt function to Protcol's structure
#[no_mangle]
fn delegate(delegator: PublicKey, validator: PublicKey, amount: U512) {
    let contract_hash = system::get_auction();
    let args = runtime_args! {
        auction::ARG_DELEGATOR => delegator,
        auction::ARG_VALIDATOR => validator,
        auction::ARG_AMOUNT => amount,
    };
    runtime::call_contract::<U512>(contract_hash, auction::METHOD_DELEGATE, args);
}

// TODO
// Adopt function to Protcol's structure
#[no_mangle]
fn undelegate(delegator: PublicKey, validator: PublicKey, amount: U512) {
    let contract_hash = system::get_auction();
    let args = runtime_args! {
        auction::ARG_DELEGATOR => delegator,
        auction::ARG_VALIDATOR => validator,
        auction::ARG_AMOUNT => amount,
    };
    let _amount: U512 = runtime::call_contract(contract_hash, auction::METHOD_UNDELEGATE, args);
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
            set_main_purse(system::create_purse());
            set_key("initialized", true);
        }
    }
}

#[no_mangle]
fn call() {
    /*
    let name: String = runtime::get_named_arg(NAME_RUNTIME_ARG_NAME);
    let symbol: String = runtime::get_named_arg(SYMBOL_RUNTIME_ARG_NAME);
    let decimals = runtime::get_named_arg(DECIMALS_RUNTIME_ARG_NAME);
    let initial_supply = runtime::get_named_arg(TOTAL_SUPPLY_RUNTIME_ARG_NAME);

    let _ = ERC20::install_custom(
        name,
        symbol,
        decimals,
        initial_supply,
        CONTRACT_KEY_NAME,
        entry_points::default(),
    );
    */

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
pub extern "C" fn add_validator() {

}

#[no_mangle]
pub extern "C" fn remove_validator() {

}

#[no_mangle]
pub extern "C" fn set_manual_validator_fraction() {

}
