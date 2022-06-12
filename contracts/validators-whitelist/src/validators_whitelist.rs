// TODO 
// Adjust to Casper network
// LIDO's mappings
// pub static REGISTRY: Map<&[u8], Validator> = Map::new("validators_registry");

pub struct Config {
    pub owner: ContractHash,
    pub hub_contract_hash: ContractHash,
    pub hub_contract_package_hash: ContractPackageHash
}
pub struct Validator {
    pub address: PublicKey,
    pub whitelisted: bool,
    pub total_delegated: U512,
    pub undelegating: U512,
    pub unlock_deadline: u8
}

impl Validator {

}

pub struct ValidatorResponse {
    // #[serde(default)]
    pub total_delegated: U512,
    pub address: PublicKey
}

#[no_mangle]
fn call() {
    
    // Runtime arguments
    let lcspr_hub_contract_hash: ContractHash = runtime::get_named_arg(LIQUID_STAKING_HUB_HASH_RUNTIME_ARG_NAME);
    let lcspr_hub_contract_package_hash: ContractPackageHash = runtime::get_named_arg(LIQUID_STAKING_HUB_CONTRACT_PACKAGE_HASH_RUNTIME_ARG_NAME);
    let lcspr_hub_contract_version: ContractVersion = runtime::get_named_arg(LIQUID_STAKING_HUB_CONTRACT_VERSION_RUNTIME_ARG_NAME);
    
    let validators_to_whitelist: Vec<PublicKey> = runtime::get_named_arg(VALIDATORS_TO_WHITELIST_ARG_NAME);
    let admins_to_set: Vec<Key> = runtime::get_named_arg(ADMINS_TO_SET_ARG_NAME);
    
    // Entry points
    let entry_points: EntryPoints = entry_points::validators_whitelist_entry_points();

    // Named keys
    let named_keys: NamedKeys = named_keys::default(
        lcspr_hub_contract_hash,
        lcspr_contract_package_hash,
        lcspr_hub_contract_version,
    );

    // Install upgradable contract
    let (contract_hash, contract_version) = storage::new_contract(entry_points, named_keys, VALIDATORS_WHITELIST_HASH_NAME, VALIDATORS_WHITELIST_UREF_NAME);

    // Put lcspr validators whitelist contract hash as NamedKey in context of Deployer (owner)
    runtime::put_key(VALIDATORS_WHITELIST_CONTRACT_KEY_NAME, Key::from(contract_hash));

    // TODO
    // Runtime arguments for "initialize_contract" function
    let runtime_arguments: RuntimeArgs = RuntimeArgs::new();

    // "init" function call
    // To set main CSPR purse of "Hub" contract
    let _: () = runtime::call_contract(contract_hash, "initialize_contract", runtime_arguments);

}

#[no_mangle]
pub extern "C" fn initialize_contract() {
    
    
    let value: Option<bool> = get_key("initialized");
    match value {
        Some(_) => {}
        None => {

            set_key("initialized", true);

            // TODO
            // Runtime arguments
            let validator_to_whitelist: PublicKey = runtime::get_named_arg(VALIDATOR_TO_WHITELIST_ARG_NAME);            
            
            // TODO
            // Create NamedKeys
            // Check deep ERC20 contract installation

            // Mapping PublicKey -> Validator
            // Create a dictionary track the mapping of account hashes to Validator structure.
            let whitelist_dict_uref: URef = storage::new_dictionary(VALIDATORS_WHITELIST_DICTIONARY_KEY_NAME).unwrap_or_revert();
            // Put
            storage::dictionary_put(whitelist_dict_uref, key, value);

            // Create a dictionary track the mapping of "Validator structure" to unstaking period / amount
            storage::new_dictionary(VALIDATORS_UNSTAKE_DICTIONARY_KEY_NAME).unwrap_or_revert();
        }
    }

}

// TODO
// Access control: contract owner, DAO contract
#[no_mangle]
pub extern "C" fn update_config(hub_contract_public_key: PublicKey, hub_contract_hash:ContractHash, hub_contract_package_hash:ContractPackageHash) {
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

// TODO
// Re-work function
fn get_validator(validator: PublicKey) -> Option<Validator, Error> {
}

/*
fn get_validator(validator: PublicKey) -> Option<Validator, Error> {

    let _validator: Validator = {
        address = validator,
        whitelisted = true,
        total_delegated =  U512::from(0),
        undelegating =U512::from(0),
        unlock_deadline: u8::from(0),
    };
}
*/

#[no_mangle]
pub extern "C" fn add_validators(validator: PublicKey) {

    // Check if Validator is already listed
    // let mut _validator: Option<Validator, Error> = get_validator(validator);
    // Return if it is
    if type_of(get_validator(validator)) == PublicKey {

    }

    // Add validator to whitelist

}

#[no_mangle]
pub extern "C" fn remove_validators(validator: PublicKey) -> Validator {

    // Check Validator's "total_delegated" amount

    // Undelegate "total_delegated" amount if total_delegated > 0

    // Check Validator's "undelegating" amount and lock period

    // Return Validator struct iff undelegating > 0

    // Remove Validator from whitelist

}

#[no_mangle]
pub extern "C" fn get_validators_whitelist(validator: PublicKey) -> Validator {

}

// LIDO's Round Robin
pub fn calculate_delegations(
    mut amount_to_delegate: U512,
    validators: &[ValidatorResponse],
) -> Result<(U512, Vec<U512>)> {
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