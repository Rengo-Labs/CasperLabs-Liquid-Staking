use alloc::{string::String, vec};

// use casper_erc20::entry_points;

use casper_types::{
    URef, U512, CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Parameter,
};

pub const PROTOCOL_FEE: &str = "protocol_fee";
pub const PROTOCOL_LOCK_PERIOD: &str = "protocol_lock_period";
pub const ARG_TMP_PURSE: &str = "tmp_purse";
pub const ARG_AMOUNT: &str = "amount";
pub const ENTRY_POINT_INIT: &str = "initialize_contract";
const ENTRY_POINT_DEPOSIT: &str = "deposit";
const ENTRY_POINT_WITHDRAW: &str = "withdraw";
const ENTRY_POINT_CLAIM: &str = "claim";
const ENTRY_POINT_SET_LOCK_PERIOD: &str = "set_lock_period";
const ENTRY_POINT_SET_PROTOCOL_FEE: &str = "set_protocol_fee";

// Entry point: deposit 
pub fn deposit() -> EntryPoint {
    EntryPoint::new(
        String::from(ENTRY_POINT_DEPOSIT),
        vec![
            Parameter::new(ARG_TMP_PURSE, URef::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

// Entry point: withdraw 
pub fn withdraw() -> EntryPoint {
    EntryPoint::new(
        String::from(ENTRY_POINT_WITHDRAW),
        vec![
            Parameter::new(ARG_AMOUNT, U512::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

// Entry point: claim 
pub fn claim() -> EntryPoint {
    EntryPoint::new(
        String::from(ENTRY_POINT_CLAIM),
        vec![
            Parameter::new(ARG_AMOUNT, U512::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

// Entry point: set_lock_period 
pub fn set_lock_period() -> EntryPoint {
    EntryPoint::new(
        String::from(ENTRY_POINT_SET_LOCK_PERIOD),
        vec![
            Parameter::new(PROTOCOL_LOCK_PERIOD, U512::cl_type()),
            ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

// Entry point: set_protocol_fee 
pub fn set_protocol_fee() -> EntryPoint {
    EntryPoint::new(
        String::from(ENTRY_POINT_SET_PROTOCOL_FEE),
        vec![
            Parameter::new(PROTOCOL_FEE, U512::cl_type()),
            ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

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

pub fn hub_contract_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(deposit());
    entry_points.add_entry_point(withdraw());
    entry_points.add_entry_point(claim());
    entry_points.add_entry_point(set_protocol_fee());
    entry_points.add_entry_point(set_lock_period());
    entry_points.add_entry_point(init());
    entry_points
}