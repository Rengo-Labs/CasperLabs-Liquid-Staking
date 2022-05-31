#![no_std]
#![no_main]
#![feature(default_alloc_error_handler)]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;

mod entry_points;
mod helpers;

use crate::helpers::get_immediate_caller_address;
// use crate::helpers::get_key;
use crate::helpers::get_main_purse;
use crate::helpers::set_key;
// use crate::helpers::set_main_purse;

use alloc::string::String;

use casper_contract::{
    contract_api::{runtime, system},
    unwrap_or_revert::UnwrapOrRevert,
};
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
    CLValue, URef, U256, U512
};

const CONTRACT_KEY_NAME: &str = "liquid_staking_hub";

#[no_mangle]
pub extern "C" fn name() {
    let name = ERC20::default().name();
    runtime::ret(CLValue::from_t(name).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn symbol() {
    let symbol = ERC20::default().symbol();
    runtime::ret(CLValue::from_t(symbol).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn decimals() {
    let decimals = ERC20::default().decimals();
    runtime::ret(CLValue::from_t(decimals).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn total_supply() {
    let total_supply = ERC20::default().total_supply();
    runtime::ret(CLValue::from_t(total_supply).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn balance_of() {
    let address: Address = runtime::get_named_arg(ADDRESS_RUNTIME_ARG_NAME);
    let balance = ERC20::default().balance_of(address);
    runtime::ret(CLValue::from_t(balance).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn transfer() {
    let recipient: Address = runtime::get_named_arg(RECIPIENT_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);

    ERC20::default()
        .transfer(recipient, amount)
        .unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn approve() {
    let spender: Address = runtime::get_named_arg(SPENDER_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);

    ERC20::default().approve(spender, amount).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn allowance() {
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    let spender: Address = runtime::get_named_arg(SPENDER_RUNTIME_ARG_NAME);
    let val = ERC20::default().allowance(owner, spender);
    runtime::ret(CLValue::from_t(val).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn transfer_from() {
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    let recipient: Address = runtime::get_named_arg(RECIPIENT_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    ERC20::default()
        .transfer_from(owner, recipient, amount)
        .unwrap_or_revert();
}

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

#[no_mangle]
fn call() {
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

    let key: Key = runtime::get_key(CONTRACT_KEY_NAME).unwrap_or_revert();
    let hash: HashAddr = key.into_hash().unwrap_or_revert();
    let contract_hash = ContractHash::new(hash);

    let _: () = runtime::call_contract(contract_hash, "init", RuntimeArgs::new());
}