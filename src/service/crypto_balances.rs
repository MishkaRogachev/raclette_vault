use crate::core::{balance::{Balance, Balances}, eth_chain::EthChain};
use super::crypto::Crypto;

const BALANCES_FETCH_PROVIDER_DELAY: std::time::Duration = std::time::Duration::from_millis(100);

impl Crypto {
    pub async fn get_eth_usd_rate(&self, chain: EthChain) -> anyhow::Result<f64> {
        let provider = self.providers.get(&chain).ok_or_else(|| anyhow::anyhow!("No provider for chain {}", chain))?;
        provider.get_eth_usd_rate().await
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