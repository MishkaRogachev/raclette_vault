
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Chain {
    EthereumMainnet,
    Optimism,
    Arbitrum,
    Goerli,
    Kovan,
    Rinkeby,
}

impl Chain {
    pub fn get_display_name(&self) -> &str {
        match self {
            Chain::EthereumMainnet => "Ethereum Mainnet",
            Chain::Optimism => "Optimism",
            Chain::Arbitrum => "Arbitrum",
            Chain::Goerli => "Goerli Testnet",
            Chain::Kovan => "Kovan Testnet",
            Chain::Rinkeby => "Rinkeby Testnet",
        }
    }

    pub fn is_test_network(&self) -> bool {
        match self {
            Chain::EthereumMainnet => false,
            Chain::Optimism => false,
            Chain::Arbitrum => false,
            Chain::Goerli => true,
            Chain::Kovan => true,
            Chain::Rinkeby => true,
        }
    }
}
