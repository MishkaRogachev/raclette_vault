use web3::{transports::Http, types::{Address, U256}, Web3};

use super::chain::Chain;

pub struct Provider {
    web3: Web3<Http>,
}

impl Provider {
    pub fn new(infura_token: &str, chain: Chain) -> anyhow::Result<Self> {
        let transport = Http::new(&chain.get_infura_url(infura_token))?;
        let web3 = Web3::new(transport);
        Ok(Self {web3})
    }

    pub async fn get_eth_balance(&self, account: Address) -> anyhow::Result<U256> {
        let balance =  self.web3.eth().balance(account, None).await?;
        Ok(balance)
    }
}
