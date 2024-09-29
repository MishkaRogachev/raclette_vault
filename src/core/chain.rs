
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Chain {
    EthereumMainnet,
    EthereumSepolia,
    OptimismMainnet,
    OptimismSepolia,
    ArbitrumMainnet,
    ArbitrumSepolia,
}

impl Chain {
    #[allow(dead_code)]
    pub fn get_display_name(&self) -> &str {
        match self {
            Chain::EthereumMainnet => "Ethereum Mainnet",
            Chain::EthereumSepolia => "Ethereum Sepolia",
            Chain::OptimismMainnet => "Optimism Mainnet",
            Chain::OptimismSepolia => "Optimism Sepolia",
            Chain::ArbitrumMainnet => "Arbitrum Mainnet",
            Chain::ArbitrumSepolia => "Arbitrum Sepolia",
        }
    }

    pub fn is_test_network(&self) -> bool {
        match self {
            Chain::EthereumMainnet => false,
            Chain::EthereumSepolia => true,
            Chain::OptimismMainnet => false,
            Chain::OptimismSepolia => true,
            Chain::ArbitrumMainnet => false,
            Chain::ArbitrumSepolia => true,
        }
    }
}
