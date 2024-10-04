
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum EthChain {
    EthereumMainnet,
    EthereumSepolia,
    OptimismMainnet,
    OptimismSepolia,
    ArbitrumMainnet,
    ArbitrumSepolia,
}

pub const MAINNET_CHAINS: [EthChain; 3] = [
    EthChain::EthereumMainnet,
    EthChain::OptimismMainnet,
    EthChain::ArbitrumMainnet,
];

pub const TESTNET_CHAINS: [EthChain; 3] = [
    EthChain::EthereumSepolia,
    EthChain::OptimismSepolia,
    EthChain::ArbitrumSepolia,
];

impl EthChain {
    pub fn get_display_name(&self) -> &str {
        match self {
            EthChain::EthereumMainnet => "Ethereum Mainnet",
            EthChain::EthereumSepolia => "Ethereum Sepolia",
            EthChain::OptimismMainnet => "Optimism Mainnet",
            EthChain::OptimismSepolia => "Optimism Sepolia",
            EthChain::ArbitrumMainnet => "Arbitrum Mainnet",
            EthChain::ArbitrumSepolia => "Arbitrum Sepolia",
        }
    }

    pub fn is_test_network(&self) -> bool {
        TESTNET_CHAINS.contains(self)
    }

    #[allow(dead_code)]
    pub fn get_chain_id(&self) -> u64 {
        match self {
            EthChain::EthereumMainnet => 1,
            EthChain::EthereumSepolia => 2,
            EthChain::OptimismMainnet => 10,
            EthChain::OptimismSepolia => 11,
            EthChain::ArbitrumMainnet => 42161,
            EthChain::ArbitrumSepolia => 421611,
        }
    }

    pub fn finalize_endpoint_url(&self, endpoint_url: &str) -> String {
        let chain_name = match self {
            EthChain::EthereumMainnet => "mainnet",
            EthChain::EthereumSepolia => "sepolia",
            EthChain::OptimismMainnet => "optimism-mainnet",
            EthChain::OptimismSepolia => "optimism-sepolia",
            EthChain::ArbitrumMainnet => "arbitrum-mainnet",
            EthChain::ArbitrumSepolia => "arbitrum-sepolia",
        };
        format!("https://{}.{}", chain_name, endpoint_url)
    }

    // NOTE from https://docs.chain.link/data-feeds/price-feeds/addresses
    pub fn get_chainlink_contract_address(&self) -> web3::types::Address {
        match self {
            EthChain::EthereumMainnet => "0x5f4ec3df9cbd43714fe2740f5e3616155c5b8419",
            EthChain::EthereumSepolia => "0x694AA1769357215DE4FAC081bf1f309aDC325306",
            EthChain::OptimismMainnet => "0x13e3Ee699D1909E989722E753853AE30b17e08c5",
            EthChain::OptimismSepolia => "0x61Ec26aA57019C486B10502285c5A3D4A4750AD7",
            EthChain::ArbitrumMainnet => "0x639Fe6ab55C921f74e7fac1ee960C0B6293ba612",
            EthChain::ArbitrumSepolia => "0xd30e2101a97dcbAeBCBC04F14C3f624E67A35165",
        }
        .parse()
        .unwrap()
    }
}

impl std::fmt::Display for EthChain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chain_name = match self {
            EthChain::EthereumMainnet => "Ethereum Mainnet",
            EthChain::EthereumSepolia => "Ethereum Sepolia",
            EthChain::OptimismMainnet => "Optimism Mainnet",
            EthChain::OptimismSepolia => "Optimism Sepolia",
            EthChain::ArbitrumMainnet => "Arbitrum Mainnet",
            EthChain::ArbitrumSepolia => "Arbitrum Sepolia",
        };
        write!(f, "{}", chain_name)
    }
}
