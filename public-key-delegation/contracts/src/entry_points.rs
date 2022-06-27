use alloc::{string::String, vec};

use casper_types::{
    U512, PublicKey, system::auction, CLType, CLTyped, Parameter,
    EntryPointAccess, EntryPointType, EntryPoints, EntryPoint,
};

pub const ENTRY_POINT_INIT: &str = "initialize_contract";
pub const ENTRY_POINT_DELEGATE: &str = "delegate_to";
pub const ENTRY_POINT_UNDELEGATE: &str = "undelegate_from";

pub fn get_entry_points() -> EntryPoints {

    // Create contract's entry points
    let mut entry_points = EntryPoints::new();
    
    // Entry point: initialize_contract
    entry_points.add_entry_point(
        EntryPoint::new(
            String::from(ENTRY_POINT_INIT),
            vec![],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    );
    
    // Entry point: delegate_to
    entry_points.add_entry_point(
        EntryPoint::new(
            String::from(ENTRY_POINT_DELEGATE),
            vec![
                Parameter::new(auction::ARG_VALIDATOR, PublicKey::cl_type()),
                Parameter::new(auction::ARG_AMOUNT, U512::cl_type()),
                ],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    );
    
    // Entry point: undelegate_from
    entry_points.add_entry_point(
        EntryPoint::new(
            String::from(ENTRY_POINT_UNDELEGATE),
            vec![
                Parameter::new(auction::ARG_VALIDATOR, PublicKey::cl_type()),
                Parameter::new(auction::ARG_AMOUNT, U512::cl_type()),
                ],
            CLType::Unit,
            EntryPointAccess::Public,
            EntryPointType::Contract,
        )
    );
    
    // Return entry points
    entry_points

}