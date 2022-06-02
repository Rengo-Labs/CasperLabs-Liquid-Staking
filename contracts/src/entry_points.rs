use alloc::{string::String, vec};

// use casper_erc20::entry_points;

use casper_types::{
    URef, U512, CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Parameter,
};

// "Liquid staking Hub" contract's entry points
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

pub fn hub_entry_points() -> EntryPoints {
    // let mut contract_entry_points = entry_points::default();
    let mut hub_entry_points = EntryPoints::new();
    hub_entry_points.add_entry_point(deposit());
    hub_entry_points.add_entry_point(withdraw());
    hub_entry_points
}