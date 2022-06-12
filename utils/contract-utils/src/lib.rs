#![no_std]
#![feature(once_cell)]

extern crate alloc;

mod admin_control;
mod contract_context;
mod contract_storage;
mod data;
mod address;

pub use admin_control::AdminControl;
pub use contract_context::ContractContext;
pub use contract_storage::{ContractStorage, OnChainContractStorage};
pub use data::{get_key, key_to_str, set_key, Dict};
pub use address::Address;