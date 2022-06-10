use alloc::{string::String, vec};

use casper_types::{
    URef, U512, CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Parameter,
};

pub fn add_validators() -> EntryPoint {
    EntryPoint::new(
        String::from("add_validators"),
        // TODO
        // Update arguements
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn remove_validators() -> EntryPoint {
    EntryPoint::new(
        String::from("remove_validators"),
        // TODO
        // Update arguements
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn update_config() -> EntryPoint {
    EntryPoint::new(
        String::from("update_config"),
        // TODO
        // Update arguements
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn initialize_contract() -> EntryPoint {
    EntryPoint::new(
        String::from("initialize_contract"),
        // TODO
        // Update arguements
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn validators_whitelist_entry_points() -> EntryPoints {
    let mut hub_entry_points = EntryPoints::new();
    hub_entry_points.add_entry_point(add_validators());
    hub_entry_points.add_entry_point(remove_validators());
    hub_entry_points.add_entry_point(update_config());
    hub_entry_points.add_entry_point(initialize_contract());
    hub_entry_points
}