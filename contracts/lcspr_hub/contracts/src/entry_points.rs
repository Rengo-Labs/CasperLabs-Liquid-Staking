use alloc::{string::String, vec};

// use casper_erc20::entry_points;

use casper_types::{
    URef, U512, CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Parameter,
};

// Entry point: deposit 
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

// Entry point: withdraw 
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

// Entry point: claim 
pub fn claim() -> EntryPoint {
    EntryPoint::new(
        String::from("claim"),
        vec![
            Parameter::new("cspr_amount", U512::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

// Entry point: set_lock_period 
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

// Entry point: set_protocol_fee 
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

// Entry point: manual_reward_distribution 
// pub fn manual_reward_distribution() -> EntryPoint {
//     EntryPoint::new(
//         String::from("manual_reward_distribution"),
//         // TODO
//         // Update arguements
//         vec![],
//         CLType::Unit,
//         EntryPointAccess::Public,
//         EntryPointType::Contract,
//     )
// }

pub fn hub_contract_entry_points() -> EntryPoints {
    let mut hub_entry_points = EntryPoints::new();
    hub_entry_points.add_entry_point(deposit());
    hub_entry_points.add_entry_point(withdraw());
    hub_entry_points.add_entry_point(claim());
    hub_entry_points.add_entry_point(set_protocol_fee());
    hub_entry_points.add_entry_point(set_lock_period());
    // hub_entry_points.add_entry_point(manual_reward_distribution());
    hub_entry_points
}