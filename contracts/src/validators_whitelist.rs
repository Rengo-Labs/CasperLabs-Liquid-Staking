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
pub extern "C" fn remove_validator() {

}

#[no_mangle]
pub extern "C" fn set_manual_validator_fraction() {

}

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