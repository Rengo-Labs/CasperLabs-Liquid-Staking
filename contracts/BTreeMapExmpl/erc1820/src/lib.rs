//! A library for developing ERC20 tokens for the Casper network.
//!
//! The main functionality is provided via the [`ERC20`] struct, and is intended to be consumed by a
//! smart contract written to be deployed on the Casper network.
//!
//! To create an example ERC20 contract which uses this library, use the cargo-casper tool:
//!
//! ```bash
//! cargo install cargo-casper
//! cargo casper --erc20 <PATH TO NEW PROJECT>
//! ```

#![warn(missing_docs)]
#![no_std]

extern crate alloc;
extern crate casper_types;
extern crate casper_contract;
extern crate once_cell;

mod address;
pub mod constants;
pub mod entry_points;
mod error;
mod implementers_registry;
mod managers_registry;
mod detail;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::convert::TryInto;

use once_cell::unsync::OnceCell;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::Bytes,
    {contracts::NamedKeys, EntryPoints, Key, URef, U256},
    ContractHash, RuntimeArgs, account::AccountHash
};

pub use address::Address;

use constants::{
    ERC1820_REGISTRY_CONTRACT_NAME, IMPLEMENTERS_REGISTRY_KEY_NAME, MANAGERS_REGISTRY_KEY_NAME
};
pub use error::Error;

/// Struct
#[derive(Default)]
pub struct ERC1820 {
    implementer_uref: OnceCell<URef>,
    manager_uref: OnceCell<URef>
}

impl ERC1820 {
    fn new(implementer_uref: URef, manager_uref: URef) -> Self {
        Self {
            implementer_uref: implementer_uref.into(),
            manager_uref: manager_uref.into()
        }
    }

    fn implementer_registry_uref(&self) -> URef {
        *self.implementer_uref.get_or_init(implementers_registry::implementers_registry)
    }

    fn managers_registry_uref(&self) -> URef {
        *self.manager_uref.get_or_init(managers_registry::managers_registry)
    }

    /// Installs the ERC1820 contract with the default set of entry points.
    ///
    /// This should be called from within `fn call()` of your contract.
    pub fn install() -> Result<ERC1820, Error> {
        let default_entry_points = entry_points::default();
        ERC1820::install_custom(
            ERC1820_REGISTRY_CONTRACT_NAME,
            default_entry_points,
        )
    }

    /// Returns the name of the token.
    pub fn set_interface_implementer(
        &self,
        account: Address,
        i_hash: String,
        implementer: Address
    ) -> Result<(), Error> {
        implementers_registry::create_or_update_implementer(
            self.implementer_registry_uref(),
            account,
            i_hash,
            implementer
        );
        Ok(())
    }

    /// Returns the symbol of the token.
    pub fn get_interface_implementer(&self, account: Address, i_hash: String) -> Result<Address, Error> {
        let result = implementers_registry::get_implementer(
            self.implementer_registry_uref(),
            account,
            i_hash
        );

        Ok(result)
    }

    /// it adds a new manager for performing operations
    pub fn set_manager(&self, account: Address, new_manager: Address) -> Result<(), Error> {
        managers_registry::set_manager(
            self.managers_registry_uref(),
            account,
            new_manager
        );

        Ok(())
    }

    /// it returns a manager for the parameter account
    pub fn get_manager(&self, account: Address) -> Result<String, Error> {
        let manager = managers_registry::get_manager(
            self.implementer_registry_uref(),
            account
        );

        Ok("Hola Munndo".to_string())
    }

    /// it returns an interface hash for an interface name
    pub fn interface_hash(&self, interface_name: String) {

    }

    /// it updates erc165 cache
    pub fn update_erc165_cache(&self) {

    }

    ///
    pub fn implements_erc165_interface(&self) {

    }

    ///
    pub fn implements_erc165_interface_no_cache(&self) {

    }

    /// Installs the ERC20 contract with a custom set of entry points.
    ///
    /// # Warning
    ///
    /// Contract developers should use [`ERC20::install`] instead, as it will create the default set
    /// of ERC20 entry points. Using `install_custom` with a different set of entry points might
    /// lead to problems with integrators such as wallets, and exchanges.
    #[doc(hidden)]
    pub fn install_custom(
        contract_key_name: &str,
        entry_points: EntryPoints,
    ) -> Result<ERC1820, Error> {
        let implementer_uref = storage::new_dictionary(IMPLEMENTERS_REGISTRY_KEY_NAME)
            .unwrap_or_revert();
        let manager_uref = storage::new_dictionary(MANAGERS_REGISTRY_KEY_NAME)
            .unwrap_or_revert();

        let mut named_keys = NamedKeys::new();

        let implementer_key = {
            runtime::remove_key(IMPLEMENTERS_REGISTRY_KEY_NAME);
            Key::from(implementer_uref)
        };

        let manager_key = {
            runtime::remove_key(MANAGERS_REGISTRY_KEY_NAME);
            Key::from(manager_uref)
        };

        named_keys.insert(IMPLEMENTERS_REGISTRY_KEY_NAME.to_string(), implementer_key);
        named_keys.insert(MANAGERS_REGISTRY_KEY_NAME.to_string(), manager_key);

        let (contract_hash, _version) =
            storage::new_locked_contract(entry_points, Some(named_keys), None, None);

        // Hash of the installed contract will be reachable through named keys.
        runtime::put_key(contract_key_name, Key::from(contract_hash));

        Ok(ERC1820::new(
            implementer_uref,
            manager_uref
        ))
    }
}