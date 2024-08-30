use serde::{Serialize, Deserialize};
use web3::types::Address;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum AccountType {
    Watch,
    Owned,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Account {
    pub name: String,
    pub account_type: AccountType,
    address: Address
}

impl Account {
    pub fn new(name: String, account_type: AccountType, address: Address) -> Self {
        Account {
            name,
            account_type,
            address
        }
    }
}
