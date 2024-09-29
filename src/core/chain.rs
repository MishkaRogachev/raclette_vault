
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

    pub fn finalize_endpoint_url(&self, endpoint_url: &str) -> String {
        let chain_name = match self {
            Chain::EthereumMainnet => "mainnet",
            Chain::EthereumSepolia => "sepolia",
            Chain::OptimismMainnet => "optimism-mainnet",
            Chain::OptimismSepolia => "optimism-sepolia",
            Chain::ArbitrumMainnet => "arbitrum-mainnet",
            Chain::ArbitrumSepolia => "arbitrum-sepolia",
        };
        format!("https://{}.{}", chain_name, endpoint_url)
    }

    // NOTE from https://docs.chain.link/data-feeds/price-feeds/addresses
    pub fn get_chainlink_contract_address(&self) -> web3::types::Address {
        match self {
            Chain::EthereumMainnet => "0x5f4ec3df9cbd43714fe2740f5e3616155c5b8419",
            Chain::EthereumSepolia => "0x694AA1769357215DE4FAC081bf1f309aDC325306",
            Chain::OptimismMainnet => "0x13e3Ee699D1909E989722E753853AE30b17e08c5",
            Chain::OptimismSepolia => "0x61Ec26aA57019C486B10502285c5A3D4A4750AD7",
            Chain::ArbitrumMainnet => "0x639Fe6ab55C921f74e7fac1ee960C0B6293ba612",
            Chain::ArbitrumSepolia => "0xd30e2101a97dcbAeBCBC04F14C3f624E67A35165",
        }
        .parse()
        .unwrap()
    }
}
