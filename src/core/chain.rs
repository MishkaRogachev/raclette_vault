
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Chain {
    EthereumMainnet,
    EthereumSepolia,
    OptimismMainnet,
    OptimismSepolia,
    ArbitrumMainnet,
    ArbitrumSepolia,
}

pub const MAINNET_CHAINS: [Chain; 3] = [
    Chain::EthereumMainnet,
    Chain::OptimismMainnet,
    Chain::ArbitrumMainnet,
];

pub const TESTNET_CHAINS: [Chain; 3] = [
    Chain::EthereumSepolia,
    Chain::OptimismSepolia,
    Chain::ArbitrumSepolia,
];

impl Chain {
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
        TESTNET_CHAINS.contains(self)
    }
}
