use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use web3::transports::Http;

use crate::{
    core::{balance::Balances, eth_chain::EthChain, provider::Provider, token::TokenList},
    persistence::db::Db
};

#[derive(Clone)]
pub struct Crypto {
    pub db: Arc<Db>,
    pub endpoint_url: String,
    pub token_list: TokenList,
    pub providers: HashMap<EthChain, Provider<Http>>,
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

    fn set_active_networks_impl(&mut self, chains: Vec<EthChain>) -> anyhow::Result<()> {
        let old_chains = self.providers.keys().cloned().collect::<Vec<_>>();
        for chain in &old_chains {
            if !chains.contains(&chain) {
                self.providers.remove(&chain);
            }
        }
        for chain in &chains {
            if !old_chains.contains(chain) {
                let transport = Http::new(&chain.finalize_endpoint_url(&self.endpoint_url))?;
                let provider = Provider::new(transport, chain.clone())?;
                self.providers.insert(chain.clone(), provider);
            }
        }
        Ok(())
    }

    pub async fn save_active_networks(&mut self, chains: Vec<EthChain>) -> anyhow::Result<()> {
        self.set_active_networks_impl(chains.clone())?;
        self.db.save_active_networks(&chains)?;
        self.invalidate_balances().await;
        Ok(())
    }

    pub fn load_active_networks(&mut self) -> anyhow::Result<()> {
        let chains = self.db.get_active_networks()?;
        self.set_active_networks_impl(chains)
    }

    pub fn get_active_networks(&self) -> Vec<EthChain> {
        self.providers.keys().cloned().collect()
    }

    pub fn in_testnet(&self) -> bool {
        self.providers.keys().any(|chain| chain.is_test_network())
    }
}
