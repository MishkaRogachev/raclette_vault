use std::collections::HashMap;

use crate::core::{chain::Chain, provider::{Provider, Balance}};

#[derive(Clone)]
pub struct Crypto {
    endpoint_url: String,
    providers: HashMap<Chain, Provider>,
}

impl Crypto {
    pub fn new(endpoint_url: &str) -> Self {
        Self {
            endpoint_url: endpoint_url.to_string(),
            providers: HashMap::new()
        }
    }

    pub fn add_chain(&mut self, chain: Chain) -> anyhow::Result<bool> {
        if self.providers.contains_key(&chain) {
            return Ok(false);
        }
        let provider = Provider::new(&self.endpoint_url, chain.clone())?;
        self.providers.insert(chain, provider);
        Ok(true)
    }

    #[allow(dead_code)]
    pub fn set_chains(&mut self, chains: Vec<Chain>) -> anyhow::Result<()> {
        let old_chains = self.providers.keys().cloned().collect::<Vec<_>>();
        for chain in &old_chains {
            if !chains.contains(&chain) {
                self.providers.remove(&chain);
            }
        }

        for chain in &chains {
            if !old_chains.contains(chain) {
                self.add_chain(chain.clone())?;
            }
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn get_active_chains(&self) -> Vec<Chain> {
        self.providers.keys().cloned().collect()
    }

    pub async fn get_eth_balance(&self, account: web3::types::Address) -> anyhow::Result<Balance> {
        let mut summary= Balance::new(0.0, 0.0, "ETH", false);
        for provider in self.providers.values() {
            if let Ok(balance) = provider.get_eth_balance(account).await {
                summary.value += balance.value;
                summary.usd_value += balance.usd_value;
                summary.from_test_network = summary.from_test_network || balance.from_test_network;
            }
        }
        Ok(summary)
    }
}