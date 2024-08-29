use serde::{Serialize, Deserialize};
use web3::types::Address;
use super::key_pair::KeyPair;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Account {
    Watch { address: Address },
    Owned { address: Address, keypair: KeyPair },
}

impl Account {
    pub fn owned_from_keypair(keypair: KeyPair) -> Self {
        let address = keypair.to_address();
        Account::Owned { address, keypair }
    }

    pub fn watch_from_address(address: Address) -> Self {
        Account::Watch { address }
    }

    pub fn address(&self) -> &Address {
        match self {
            Account::Watch { address } => address,
            Account::Owned { address, .. } => address,
        }
    }
}
