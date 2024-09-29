use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use crate::{core::{chain::Chain, provider::{Balance, Provider}}, persistence::db::Db};

#[derive(Clone)]
pub struct Crypto {
    db: Arc<Db>,
    endpoint_url: String,
    providers: HashMap<Chain, Provider>,
    balances: Arc<RwLock<HashMap<web3::types::Address, Balance>>>,
}

impl Crypto {
    pub fn new(db: Arc<Db>, endpoint_url: &str) -> Self {
        Self {
            db,
            endpoint_url: endpoint_url.to_string(),
            providers: HashMap::new(),
            balances: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn set_active_networks_impl(&mut self, chains: Vec<Chain>) -> anyhow::Result<()> {
        let old_chains = self.providers.keys().cloned().collect::<Vec<_>>();
        for chain in &old_chains {
            if !chains.contains(&chain) {
                self.providers.remove(&chain);
            }
        }
        for chain in &chains {
            if !old_chains.contains(chain) {
                let provider = Provider::new(&self.endpoint_url, chain.clone())?;
                self.providers.insert(chain.clone(), provider);
            }
        }
        Ok(())
    }

    pub async fn save_active_networks(&mut self, chains: Vec<Chain>) -> anyhow::Result<()> {
        self.set_active_networks_impl(chains.clone())?;
        self.db.save_active_networks(&chains)?;
        self.invalidate_balances().await;
        Ok(())
    }

    pub fn load_active_networks(&mut self) -> anyhow::Result<()> {
        let chains = self.db.get_active_networks()?;
        self.set_active_networks_impl(chains)
    }

    pub fn get_active_networks(&self) -> Vec<Chain> {
        self.providers.keys().cloned().collect()
    }

    pub fn in_testnet(&self) -> bool {
        self.providers.keys().any(|chain| chain.is_test_network())
    }

    pub async fn get_eth_balance(&self, account: web3::types::Address) -> Option<Balance> {
        let balances = self.balances.read().await;
        if let Some(balance) = balances.get(&account) {
            return Some(balance.clone());
        }
        None
    }

    pub async fn fetch_eth_balances(&mut self, accounts: Vec<web3::types::Address>) {
        let balances = self.balances.clone();
        let providers = self.providers.values().cloned().collect::<Vec<_>>();

        tokio::spawn(async move {
            let mut summaries: HashMap<web3::types::Address, Balance> = HashMap::new();

            for provider in providers {
                for account in accounts.iter() {
                    let response = provider.get_eth_balance(*account).await;
                    match response {
                        Ok(balance) => {
                            let summary = summaries.entry(*account).or_insert_with(
                                || Balance::new(0.0, 0.0, "ETH", false));
                            summary.value += balance.value;
                            summary.usd_value += balance.usd_value;
                            summary.from_test_network = summary.from_test_network || balance.from_test_network;
                        }
                        Err(_err) => {
                            // eprintln!("Failed to fetch balance for {}: {:?}", account, err);
                        }
                    }
                }
            }
            balances.write().await.extend(summaries);
        });
    }

    pub async fn invalidate_balances(&mut self) {
        let mut balances = self.balances.write().await;
        balances.clear();
    }
}