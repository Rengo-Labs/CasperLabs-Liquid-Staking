use alloc::{string::String, vec};

use casper_types::{
    URef, U512, CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Parameter,
};

pub const ENTRY_POINT_INIT: &str = "initialize_contract";

// Entry point: initialize_contract 
pub fn init() -> EntryPoint {
    EntryPoint::new(
        String::from(ENTRY_POINT_INIT),
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn reward_dispatcher_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(init());
    entry_points
}