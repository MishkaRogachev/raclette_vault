use std::collections::HashMap;
use web3::types::Address;

use super::eth_chain::EthChain;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TokenOnChainData {
    pub contract_address: Address,
    pub decimals: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Token {
    pub name: String,
    pub symbol: String,
    pub chain_data: HashMap<EthChain, TokenOnChainData>,
}

pub type TokenList = Vec<Token>;

impl Token {
    #[allow(dead_code)]
    pub fn new(name: &str, symbol: &str) -> Self {
        Self {
            name: name.to_string(),
            symbol: symbol.to_string(),
            chain_data: HashMap::new(),
        }
    }

    pub fn with_chain_data(mut self, chain: EthChain, contract_address: Address, decimals: u16) -> Self {
        self.chain_data.insert(
            chain,
            TokenOnChainData {
                contract_address,
                decimals,
            },
        );
        self
    }

    pub fn get_chain_data(&self, chain: &EthChain) -> Option<&TokenOnChainData> {
        self.chain_data.get(chain)
    }

    pub fn is_same_token(&self, other: &Self) -> bool {
        self.symbol == other.symbol
    }

    pub fn merge_chain_data(&mut self, other: &Self) {
        for (chain, data) in &other.chain_data {
            if !self.chain_data.contains_key(chain) {
                self.chain_data.insert(chain.clone(), data.clone());
            }
        }
    }

    #[allow(dead_code)]
    pub fn flatten_token_lists<I>(lists: I) -> TokenList
    where I: IntoIterator<Item = TokenList>,
    {
        let mut result = TokenList::new();
        for list in lists {
            for token in list {
                if let Some(existing_token) = result.iter_mut().find(|t| t.is_same_token(&token)) {
                    existing_token.merge_chain_data(&token);
                } else {
                    result.push(token);
                }
            }
        }
        result
    }
}
