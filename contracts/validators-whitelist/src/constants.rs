/// Contracts
//
pub const VALIDATORS_WHITELIST_CONTRACT_KEY_NAME: &str = "liquid_casper_validators_whitelist";
//
pub const VALIDATORS_WHITELIST_HASH_NAME: &str = "validators_whitelist_hash";

pub const VALIDATORS_WHITELIST_PACKAGE_HASH_NAME: &str = "validators_whitelist_contract_package_hash";
//
pub const VALIDATORS_WHITELIST_UREF_NAME: &str = "validators_whitelist_uref";

/// Named keys
//
pub const OWNER_KEY_NAME: &str = "owner";

/// Dictionaries
//
pub const VALIDATORS_WHITELIST_DICTIONARY_KEY_NAME:  &str = "white_list";
//
pub const VALIDATORS_UNSTAKE_LIST_DICTIONARY_KEY_NAME:  &str = "unstake_list";

/// Entry points

pub const ADD_VALIDATORS_ENTRY_POINT_NAME:  &str = "add_validators";

pub const REMOVE_VALIDATORS_ENTRY_POINT_NAME:  &str = "remove_validators";

pub const UPDATE_CONFIG_ENTRY_POINT_NAME:  &str = "update_config";

pub const INITIALIZE_CONTRACT_ENTRY_POINT_NAME:  &str = "initialize_contract";

// pub const SET_HUB_CONTRACT_ENTRY_POINT_NAME:  &str = "set_hub_contract";

// pub const SET_DAO_CONTRACT_ENTRY_POINT_NAME:  &str = "set_dao_contract";

/// Runtime argument names

pub const AMOUNT_KEY_NAME:  &str = "amount";
//
pub const LIQUID_STAKING_HUB_HASH_RUNTIME_ARG_NAME:  &str = "liquid_staking_hub_contract_hash";
//
pub const LIQUID_STAKING_HUB_CONTRACT_PACKAGE_HASH_RUNTIME_ARG_NAME:  &str = "liquid_staking_hub_contract_package_hash";
//
pub const DAO_CONTRACT_HASH_RUNTIME_ARG_NAME:  &str = "dao_contract_hash";
//
pub const DAO_CONTRACT_PACKAGE_HASH_RUNTIME_ARG_NAME:  &str = "dao_contract_package_hash";
//
pub const VALIDATORS_TO_WHITELIST_ARG_NAME:  &str = "validator_to_whitelist";
//
pub const ADMINS_TO_SET_ARG_NAME: &str = "admins_to_set";