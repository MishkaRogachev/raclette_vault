use web3::Web3;

use super::eth_chain::EthChain;

#[derive(Clone)]
pub struct Provider<T: web3::Transport> {
    pub web3: Web3<T>,
    pub chain: EthChain,
}

impl<T: web3::Transport> Provider<T> {
    pub fn new(transport: T, chain: EthChain) -> anyhow::Result<Self> {
        let web3 = Web3::new(transport);
        Ok(Self { web3, chain })
    }
}
