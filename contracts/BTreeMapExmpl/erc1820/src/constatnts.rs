//! Constants used by the ERC20 contract.
///
pub const IMPLEMENTERS_REGISTRY_KEY_NAME: &str = "implementers";
///
pub const MANAGERS_REGISTRY_KEY_NAME: &str = "managers";
///
pub const ERC1820_REGISTRY_CONTRACT_NAME: &str = "erc1820_registry";

/// Entry points
pub const SET_INTERFACE_ENTRY_POINT: &str = "set_interface_implementer";
///
pub const GET_INTERFACE_ENTRY_POINT: &str = "get_interface_implementer";
///
pub const SET_MANAGER_ENTRY_POINT: &str = "set_manager";
///
pub const GET_MANAGER_ENTRY_POINT: &str = "get_manager";
///
pub const INTERFACE_HASH_ENTRY_POINT: &str = "interface_hash";
///
pub const UPDATE_ERC165_CACHE_ENTRY_POINT: &str = "update_erc165_cache";
///
pub const IMPLEMENTS_ERC165_INTERFACE_ENTRY_POINT: &str = "implements_erc165_interface";
///
pub const IMPLEMENTS_ERC165_INTERFACE_NO_CACHE_ENTRY_POINT: &str = "implements_erc165_interface_no_cache";

/// Entry point's parameter
pub const ACCOUNT_RUNTIME_ARG_NAME: &str = "account";
///
pub const NEW_MANAGER_RUNTIME_ARG_NAME: &str = "new_manager";
///
pub const I_HASH_RUNTIME_ARG_NAME: &str = "interface_hash";
///
pub const I_NAME_RUNTIME_ARG_NAME: &str = "interface_name";
///
pub const IMPLEMENTER_RUNTIME_ARG_NAME: &str = "implementer";
///
pub const I_ID_RUNTIME_ARG_NAME: &str = "interface_id";


