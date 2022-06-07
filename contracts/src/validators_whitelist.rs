#![no_main]

mod helpers;
mod entry_points;

use validators_whitelist::data::{self, get_all_pairs, Pairs, Whitelists};
use validators_whitelist::alloc::string::ToString;
use validators_whitelist::data::{self, get_all_pairs, Pairs, Whitelists};
use alloc::collections::BTreeMap;
use alloc::{string::String, vec::Vec};
use casper_contract::contract_api::runtime;
use casper_contract::contract_api::storage;
use contract_utils::{ContractContext, ContractStorage};

use validators_whitelist::helpers::{ get_immediate_caller_address, get_key, get_main_purse, set_key, set_main_purse };

use casper_contract::{
    contract_api::{runtime, system},
    unwrap_or_revert::UnwrapOrRevert,
};

use casper_types::{
    runtime_args, system::auction, PublicKey,
    ContractHash, HashAddr, Key, RuntimeArgs,
    CLValue, URef, U256, U512, EntryPoints,
    ContractPackageHash, ApiError
};

const CONTRACT_KEY_NAME: &str = "validators_whitelist_liquid_casper";

// LIDO's mapping
pub static REGISTRY: Map<&[u8], Validator> = Map::new("validators_registry");

pub struct Config {
    pub owner: CanonicalAddr,
    pub hub_contract: CanonicalAddr,
}

pub struct Validator {
    pub address: String,
}

pub struct ValidatorResponse {
    #[serde(default)]
    pub total_delegated: Uint128,

    pub address: String,
}

#[no_mangle]
fn call() {
    
    let entry_points: EntryPoints = entry_points::validators_list_entry_points();

    // TODO
    // Install upgradable contract

    let key: Key = runtime::get_key(CONTRACT_KEY_NAME).unwrap_or_revert();
    let hash: HashAddr = key.into_hash().unwrap_or_revert();
    let contract_hash = ContractHash::new(hash);

    // "init" function call
    // To set main CSPR purse of "Hub" contract
    let _: () = runtime::call_contract(contract_hash, "init", RuntimeArgs::new());

}

// TODO
// Access control: contract owner, DAO contract
// Unite into "Update_config" function
#[no_mangle]
pub extern "C" fn set_hub_contract(hub_contract_hash:ContractHash, hub_contract_package_hash:ContractPackageHash) {
    let value: Option<bool> = get_key("hub_contract_connected");
    match value {
        Some(_) => {}
        None => {
            set_key("hub_contract_connected", true);
            set_key("hub_contract_hash", hub_contract_hash);
            set_key("hub_contract_package_hash", hub_contract_package_hash);
        }
    }
}

#[no_mangle]
pub extern "C" fn add_validators() {

}

#[no_mangle]
pub extern "C" fn remove_validators() {

}

// get_validators_for_delegation
// get_whitelist_delegators

// update_config
// Update Owner and Hub_contract

/*
#[no_mangle]
pub extern "C" fn set_manual_validator_fraction() {

}
*/

// Rengo uniswap "factory.rs" code
fn create_pair(&mut self, token_a: Key, token_b: Key, pair_hash: Key) {
    let white_lists: Whitelists = Whitelists::instance();
    let white_list_user: Key = white_lists.get(&self.get_caller());
    if white_list_user
        != Key::from_formatted_str(
            "account-hash-0000000000000000000000000000000000000000000000000000000000000000",
        )
        .unwrap()
    {
        if token_a == token_b {
            runtime::revert(Error::UniswapV2FactoryIdenticalAddresses);
        }
        let token0: Key;
        let token1: Key;
        let address_0: Key = Key::from_formatted_str(
            "hash-0000000000000000000000000000000000000000000000000000000000000000",
        )
        .unwrap();
        if token_a < token_b {
            token0 = token_a;
            token1 = token_b;
        } else {
            token0 = token_b;
            token1 = token_a;
        }
        // in before 0 address was hash-0000000000000000000000000000000000000000000000000000000000000000
        if token0 == address_0 {
            runtime::revert(Error::UniswapV2FactoryZeroAddress);
        }
        let pair_0_1_key: Key = self.get_pair(token0, token1);
        let pair_1_0_key: Key = self.get_pair(token1, token0);
        if pair_0_1_key != address_0 {
            runtime::revert(Error::UniswapV2FactoryPairExists);
        }
        if pair_1_0_key != address_0 {
            runtime::revert(Error::UniswapV2FactoryPairExists);
        }
        //convert Key to ContractHash
        let pair_hash_add_array = match pair_hash {
            Key::Hash(package) => package,
            _ => runtime::revert(ApiError::UnexpectedKeyVariant),
        };
        let pair_contract_hash = ContractHash::new(pair_hash_add_array);
        let _ret: () = runtime::call_contract(
            pair_contract_hash,
            "initialize",
            runtime_args! {"token0" => token0, "token1" => token1, "factory_hash" => data::get_hash() },
        );
        // handling the pair creation by updating the storage
        self.set_pair(token0, token1, pair_hash);
        self.set_pair(token1, token0, pair_hash);
        let mut pairs: Vec<Key> = get_all_pairs();
        pairs.push(pair_hash);
        self.set_all_pairs(pairs);
        self.emit(&FACTORYEvent::PairCreated {
            token0: token0,
            token1: token1,
            pair: pair_hash,
            all_pairs_length: (get_all_pairs().len()).into(),
        });
    } else {
        runtime::revert(Error::UniswapV2FactoryNotInWhiteList);
    }
}

fn get_pair(&mut self, token0: Key, token1: Key) -> Key {
    Pairs::instance().get(&token0, &token1)
}

fn set_pair(&mut self, token0: Key, token1: Key, value: Key) {
    Pairs::instance().set(&token0, &token1, value);
}

// LIDO Calculations:
/*
use crate::registry::ValidatorResponse;
use cosmwasm_std::{StdError, StdResult, Uint128};
use std::ops::Sub;
*/
pub fn calculate_delegations(
    mut amount_to_delegate: Uint128,
    validators: &[ValidatorResponse],
) -> StdResult<(Uint128, Vec<Uint128>)> {
    if validators.is_empty() {
        return Err(StdError::generic_err("Empty validators set"));
    }
    let total_delegated: u128 = validators.iter().map(|v| v.total_delegated.u128()).sum();
    let total_coins_to_distribute = Uint128::from(total_delegated) + amount_to_delegate;
    let coins_per_validator = total_coins_to_distribute.u128() / validators.len() as u128;
    let remaining_coins = total_coins_to_distribute.u128() % validators.len() as u128;

    let mut delegations = vec![Uint128::zero(); validators.len()];
    for (index, validator) in validators.iter().enumerate() {
        let extra_coin = if (index + 1) as u128 <= remaining_coins {
            1u128
        } else {
            0u128
        };
        if coins_per_validator + extra_coin < validator.total_delegated.u128() {
            continue;
        }
        let mut to_delegate =
            Uint128::from(coins_per_validator + extra_coin).sub(validator.total_delegated);
        if to_delegate > amount_to_delegate {
            to_delegate = amount_to_delegate
        }
        delegations[index] = to_delegate;
        amount_to_delegate = amount_to_delegate.checked_sub(to_delegate)?;
        if amount_to_delegate.is_zero() {
            break;
        }
    }
    Ok((amount_to_delegate, delegations))
}

pub fn calculate_undelegations(
    mut undelegation_amount: Uint128,
    mut validators: Vec<ValidatorResponse>,
) -> StdResult<Vec<Uint128>> {
    if validators.is_empty() {
        return Err(StdError::generic_err("Empty validators set"));
    }

    let mut total_delegated: Uint128 = validators.iter().map(|v| v.total_delegated).sum();

    if undelegation_amount > total_delegated {
        return Err(StdError::generic_err(
            "undelegate amount can't be bigger than total delegated amount",
        ));
    }

    let mut undelegations = vec![Uint128::zero(); validators.len()];

    while !undelegation_amount.is_zero() {
        let total_coins_after_undelegation = total_delegated.sub(undelegation_amount);
        let coins_per_validator = total_coins_after_undelegation.u128() / validators.len() as u128;
        let remaining_coins = total_coins_after_undelegation.u128() % validators.len() as u128;

        for (index, validator) in validators.iter_mut().enumerate() {
            let extra_coin = if (index + 1) as u128 <= remaining_coins {
                1u128
            } else {
                0u128
            };
            let mut to_undelegate = validator.total_delegated.checked_sub(
                Uint128::from(coins_per_validator + extra_coin).min(validator.total_delegated),
            )?;
            if to_undelegate > undelegation_amount {
                to_undelegate = undelegation_amount
            }

            undelegations[index] += to_undelegate;
            undelegation_amount = undelegation_amount.checked_sub(to_undelegate)?;
            total_delegated = total_delegated.checked_sub(to_undelegate)?;
            validator.total_delegated = validator.total_delegated.checked_sub(to_undelegate)?;

            if undelegation_amount.is_zero() {
                break;
            }
        }
    }
    Ok(undelegations)
}