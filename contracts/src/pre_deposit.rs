#![no_main]

use casper_contract::{
    contract_api::{account, runtime, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_erc20::Address;

use casper_types::RuntimeArgs;
use casper_types::{runtime_args, ApiError, ContractHash, HashAddr, Key, URef, U256, U512};

pub enum DepositError {
    ExceedUserLimit = 0,
    ExceedContractLimit = 1,
}

impl From<DepositError> for ApiError {
    fn from(error: DepositError) -> ApiError {
        ApiError::User(error as u16)
    }
}

#[no_mangle]
fn call() {
    
    // This function is used to accept CSPR from the user and pass to the
    // Main contract that going to swap it to WCSPR
    // We need it because casper don't allow us to access `get_main_purse` from main contract

    // CSPR tokens amount to transfer
    let cspr_amount: U512 = runtime::get_named_arg("cspr_amount");
    let cspr_amount_u256: U256 = U256::from(cspr_amount.as_u128());

    // Staking contract hash address passed as an argument to this contract
    let wcspr_contract_key: Key = runtime::get_named_arg("cswap_hub_contract_hash_key");
    let _wcspr_contract_hash: HashAddr = wcspr_contract_key.into_hash().unwrap_or_revert();
    let wcspr_contract_hash: ContractHash = ContractHash::new(_wcspr_contract_hash);

    // Get Address (AccountHash) of the user who called the contract
    let sender: Address = Address::from(runtime::get_caller());

    // Purse with CSPR tokens of the user who call the contract
    let sender_purse: URef = account::get_main_purse();

    // Here we put tokens we want to transfer to the contract
    let tmp_purse: URef = system::create_purse();

    // Move from sender to tmp purse, so we can use tmp_purse in the Staking contract
    let _ = system::transfer_from_purse_to_purse(sender_purse, tmp_purse, cspr_amount, None);

    runtime::call_contract(
        wcspr_contract_hash,
        "deposit",
        runtime_args! {
            "tmp_purse" => tmp_purse
        },
    )
}