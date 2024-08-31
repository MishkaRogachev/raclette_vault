use serde::{Serialize, Deserialize};
use web3::types::Address;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum WalletAccountType {
    Watch,
    Owned,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct WalletAccount {
    pub name: String,
    pub account_type: WalletAccountType,
    address: Address
}

impl WalletAccount {
    pub fn new(name: String, account_type: WalletAccountType, address: Address) -> Self {
        Self {
            name,
            account_type,
            address
        }
    }
}
