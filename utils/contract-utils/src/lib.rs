#![no_std]
#![feature(once_cell)]

extern crate alloc;

pub mod admin_control;
pub mod contract_context;
pub mod contract_storage;
pub mod data;
pub mod error;
pub mod address;
pub mod balances;

pub use admin_control::AdminControl;
pub use contract_context::ContractContext;
pub use contract_storage::{ContractStorage, OnChainContractStorage};
pub use data::{get_key, key_to_str, set_key, Dict};
pub use address::Address;
pub use error:Error;