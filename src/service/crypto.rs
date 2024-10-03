use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use crate::{
    core::{balance::{Balance, Balances}, chain::Chain, provider::Provider, token::TokenList},
    persistence::db::Db
};

const BALANCES_FETCH_PROVIDER_DELAY: std::time::Duration = std::time::Duration::from_millis(100);

#[derive(Clone)]
pub struct Crypto {
    pub db: Arc<Db>,
    pub endpoint_url: String,
    pub token_list: TokenList,
    pub providers: HashMap<Chain, Provider>,
    pub account_balances: Arc<RwLock<HashMap<web3::types::Address, Balances>>>,
}

impl Crypto {
    pub fn new(db: Arc<Db>, endpoint_url: &str) -> Self {
        let token_list: TokenList = serde_json::from_slice(include_bytes!("../../token_list.json")).unwrap();
        Self {
            db,
            endpoint_url: endpoint_url.to_string(),
            token_list,
            providers: HashMap::new(),
            account_balances: Arc::new(RwLock::new(HashMap::new())),
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

    pub async fn get_balances(&self, account: web3::types::Address) -> Option<Balances> {
        let balances = self.account_balances.read().await;
        if let Some(balance) = balances.get(&account) {
            return Some(balance.clone());
        }
        None
    }

    pub async fn fetch_balances(&self, accounts: Vec<web3::types::Address>) {
        let account_balances = self.account_balances.clone();
        let providers = self.providers.clone();
        let token_list = self.token_list.clone();

        tokio::spawn(async move {
            for account in accounts.iter() {
                let mut sum_balances: Balances = Vec::new();
                for (_, provider) in providers.iter() {
                    let balance = provider.get_eth_balance(*account).await;
                    match balance {
                        Ok(balance) => {
                            sum_balances = Balance::extend_balances(sum_balances, &vec![balance]);
                        }
                        Err(_err) => {
                            log::error!("Failed to fetch ETH balance for {}", account);
                        }
                    }

                    let token_balances = provider.get_token_balances(*account, &token_list).await;
                    match token_balances {
                        Ok(token_balances) => {
                            sum_balances = Balance::extend_balances(sum_balances, &token_balances);
                        }
                        Err(_err) => {
                            log::error!("Failed to fetch token balances for {}", account);
                        }
                    }
                    tokio::time::sleep(BALANCES_FETCH_PROVIDER_DELAY).await;
                }
                account_balances.write().await.insert(*account, sum_balances);
            }
        });
    }

    pub async fn invalidate_balances(&self) {
        let mut balances = self.account_balances.write().await;
        balances.clear();
    }
}
