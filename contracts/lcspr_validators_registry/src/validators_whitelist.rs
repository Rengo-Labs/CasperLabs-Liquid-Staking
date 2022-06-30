pub struct Validator {
    pub address: PublicKey,
    
    // Values I suppose to be usefull
    /*
    pub whitelisted: bool,
    pub total_delegated: U512,
    pub undelegating: U512,
    pub unlock_deadline: u8
    */
}

impl Validator {

}

pub struct ValidatorResponse {
    pub total_delegated: U512,
    pub address: PublicKey
}

#[no_mangle]
fn call() {
    
    // Get caller of type `Address` (erc20 implemetation)
    let caller = data::get_caller_address();
    
    // Runtime arguments
    let lcspr_hub_contract_package_hash: ContractPackageHash = runtime::get_named_arg(HUB_CONTRACT_PACKAGE_HASH_RUNTIME_ARG_NAME);
    let lcspr_hub_contract_version: ContractVersion = runtime::get_named_arg(HUB_CONTRACT_VERSION_RUNTIME_ARG_NAME);
    
    let validators_to_whitelist: Vec<PublicKey> = runtime::get_named_arg(VALIDATORS_TO_WHITELIST_ARG_NAME);
    let admins_to_set: Vec<Key> = runtime::get_named_arg(ADMINS_TO_SET_ARG_NAME);
    
    // Entry points
    let entry_points: EntryPoints = entry_points::validators_whitelist_entry_points();

    // Named keys
    let named_keys: NamedKeys = NamedKeys::new();

    let owner_key = {
        let owner_uref = storage::new_uref(caller).into_read_write();
        Key::from(owner_uref)
    };

    let hub_contract_package_hash_key = {
        // Type: ContractPackageHash
        let hub_contract_package_hash_uref = storage::new_uref(hub_contract_package_hash).into_read_write();
        Key::from(hub_contract_package_hash_uref)
    };

    let hub_contract_version_key = {
        // Type: ContractVersion
        let hub_contract_version_uref = storage::new_uref(hub_contract_version).into_read_write();
        Key::from(hub_contract_version_uref)
    };

    // TODO
    // Create Dictionaries and coresponding NamedKeys

    named_keys.insert(OWNER_KEY_NAME.to_string(), owner_key);
    named_keys.insert(HUB_CONTRACT_PACKAGE_HASH_KEY_NAME.to_string(), hub_contract_package_hash_key);
    named_keys.insert(HUB_CONTRACT_VERSION_KEY_NAME.to_string(), hub_contract_version_key);
    named_keys.insert(VALIDATORS_WHITELIST_DICTIONARY_KEY_NAME.to_string(), validators_whitelist_dictionary_key);
    named_keys.insert(VALIDATORS_RESPONSE_DICTIONARY_KEY_NAME.to_string(), validators_response_dictionary_key);

    // Install upgradable contract
    let (contract_hash, contract_version) = storage::new_contract(entry_points, named_keys, VALIDATORS_WHITELIST_HASH_NAME, VALIDATORS_WHITELIST_UREF_NAME);

    // TODO
    // Save current stable contract vesrion in context of deployer
    //
    // ContractPackageHash already saved to context on "storage::new_contract" call
    // Put lcspr validators whitelist contract hash as NamedKey in context of Deployer (owner)
    // runtime::put_key(VALIDATORS_WHITELIST_VERSION_KEY_NAME, Key::from(contract_version));

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
            // let validator_to_whitelist: PublicKey = runtime::get_named_arg(VALIDATORS_TO_WHITELIST_ARG_NAME);
            let validators_to_whitelist: Vec<PublicKey> = runtime::get_named_arg(VALIDATORS_TO_WHITELIST_ARG_NAME);
            let admins_to_set: Vec<Key> = runtime::get_named_arg(ADMINS_TO_SET_ARG_NAME);
    
            
            // TODO
            // Create NamedKeys
            // Check deep ERC20 contract installation

            // Mapping PublicKey -> Validator
            // Create a dictionary track the mapping of account hashes to Validator structure.
            let whitelist_dict_uref: URef = storage::new_dictionary(VALIDATORS_WHITELIST_DICTIONARY_KEY_NAME).unwrap_or_revert();
            // Put
            storage::dictionary_put(whitelist_dict_uref, key, value);
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
    //
}

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

    // Return Validator struct if undelegating > 0

    // Remove Validator from whitelist

    REGISTRY.remove(deps.storage, validator_address.as_str().as_bytes());

    let mut validators = query_validators(deps.as_ref())?;
    if validators.is_empty() {
        return Err(StdError::generic_err(
            "Cannot remove the last validator in the registry",
        ));
    }
    validators.sort_by(|v1, v2| v1.total_delegated.cmp(&v2.total_delegated));

    let hub_address = deps.api.addr_humanize(&config.hub_contract)?;

    let query = deps
        .querier
        .query_delegation(hub_address.clone(), validator_address.clone());

    let mut messages: Vec<CosmosMsg> = vec![];
    if let Ok(q) = query {
        let delegated_amount = q;

        let mut redelegations: Vec<(String, Coin)> = vec![];
        if let Some(delegation) = delegated_amount {
            // Terra core returns zero if there is another active redelegation
            // That means we cannot start a new redelegation, so we only remove a validator from
            // the registry.
            // We'll do a redelegation manually later by sending RedelegateProxy to the hub
            if delegation.can_redelegate.amount < delegation.amount.amount {
                return StdResult::Ok(Response::new());
            }

            let (_, delegations) =
                calculate_delegations(delegation.amount.amount, validators.as_slice())?;

            for i in 0..delegations.len() {
                if delegations[i].is_zero() {
                    continue;
                }
                redelegations.push((
                    validators[i].address.clone(),
                    Coin::new(delegations[i].u128(), delegation.amount.denom.as_str()),
                ));
            }

            let regelegate_msg = RedelegateProxy {
                src_validator: validator_address,
                redelegations,
            };
            messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: hub_address.clone().into_string(),
                msg: to_binary(&regelegate_msg)?,
                funds: vec![],
            }));

            let msg = UpdateGlobalIndex {
                airdrop_hooks: None,
            };
            messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: hub_address.into_string(),
                msg: to_binary(&msg)?,
                funds: vec![],
            }));
        }
    }

    let res = Response::new().add_messages(messages);
    Ok(res)

}

#[no_mangle]
pub extern "C" fn get_validators_whitelist(validator: PublicKey) -> Validator {

}

// LIDO's Round Robin
fn calculate_delegations(
    mut amount_to_delegate: U512,
    validators: &[ValidatorResponse],
) -> Result<(U512, Vec<U512>)> {
    
    if validators.is_empty() {
        // TODO
        // Rework error:
        return Err(StdError::generic_err("Empty validators set"));
    }
    
    // TODO
    // Rewok calculations into "checked_add" implementations
    let total_delegated: U512 = validators.iter().map(|v| v.total_delegated).sum();
    let total_coins_to_distribute: U512 = total_delegated + amount_to_delegate;
    let coins_per_validator: U512 = total_coins_to_distribute / validators.len();
    let remaining_coins: U512 = total_coins_to_distribute % validators.len();

    let mut delegations = vec![U512::zero(); validators.len()];
    for (index, validator) in validators.iter().enumerate() {
        let extra_coin = if (index + 1) as U512 <= remaining_coins {
            U512::from(1);
        } else {
            U512::from(0);
        };
        if coins_per_validator + extra_coin < validator.total_delegated {
            continue;
        }
        let mut to_delegate =
            U512::from(coins_per_validator + extra_coin).sub(validator.total_delegated);
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

fn calculate_undelegations(
    mut undelegation_amount: U512,
    mut validators: Vec<ValidatorResponse>,
) -> Result<Vec<U512>> {
    
    if validators.is_empty() {
        // TODO
        // Rework error:
        return Err(StdError::generic_err("Empty validators set"));
    }

    let mut total_delegated: U512 = validators.iter().map(|v| v.total_delegated).sum();

    if undelegation_amount > total_delegated {
        // TODO
        // Rework error:
        return Err(StdError::generic_err(
            "undelegate amount can't be bigger than total delegated amount",
        ));
    }

    let mut undelegations = vec![U512::zero(); validators.len()];

    while !undelegation_amount.is_zero() {
        let total_coins_after_undelegation: U512 = total_delegated.sub(undelegation_amount);
        let coins_per_validator: U512 = total_coins_after_undelegation / validators.len() as U512;
        let remaining_coins: U512 = total_coins_after_undelegation % validators.len() as U512;

        for (index, validator) in validators.iter_mut().enumerate() {
            let extra_coin = if (index + 1) as U512 <= remaining_coins {
                U512::from(1);
            } else {
                U512::from(0);
            };
            let mut to_undelegate = validator.total_delegated.checked_sub(
                coins_per_validator + extra_coin).min(validator.total_delegated)?;
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