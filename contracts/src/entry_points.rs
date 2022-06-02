use alloc::{string::String, vec};

// use casper_erc20::entry_points;

use casper_types::{
    URef, U512, CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Parameter,
};

// "liquid_staking_hub" entry points
pub fn deposit() -> EntryPoint {
    EntryPoint::new(
        String::from("deposit"),
        vec![
            Parameter::new("tmp_purse", URef::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn withdraw() -> EntryPoint {
    EntryPoint::new(
        String::from("withdraw"),
        vec![
            Parameter::new("cspr_amount", U512::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn set_lock_period() -> EntryPoint {
    EntryPoint::new(
        String::from("set_lock_period"),
        // TODO
        // Update arguements
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn set_protocol_fee() -> EntryPoint {
    EntryPoint::new(
        String::from("set_protocol_fee"),
        // TODO
        // Update arguements
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn manual_reward_distribution() -> EntryPoint {
    EntryPoint::new(
        String::from("manual_reward_distribution"),
        // TODO
        // Update arguements
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn hub_contract_entry_points() -> EntryPoints {
    let mut hub_entry_points = EntryPoints::new();
    hub_entry_points.add_entry_point(deposit());
    hub_entry_points.add_entry_point(withdraw());
    hub_entry_points.add_entry_point(set_protocol_fee());
    hub_entry_points.add_entry_point(set_lock_period());
    hub_entry_points.add_entry_point(manual_reward_distribution());
    hub_entry_points
}

// "validators_list" entry points
pub fn add_validator() -> EntryPoint {
    EntryPoint::new(
        String::from("add_validator"),
        // TODO
        // Update arguements
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn remove_validator() -> EntryPoint {
    EntryPoint::new(
        String::from("remove_validator"),
        // TODO
        // Update arguements
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn set_manual_validator_fraction() -> EntryPoint {
    EntryPoint::new(
        String::from("set_manual_validator_fraction"),
        // TODO
        // Update arguements
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn validators_list_entry_points() -> EntryPoints {
    let mut hub_entry_points = EntryPoints::new();
    hub_entry_points.add_entry_point(add_validator());
    hub_entry_points.add_entry_point(remove_validator());
    hub_entry_points.add_entry_point(set_manual_validator_fraction());
    hub_entry_points
}