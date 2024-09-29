use web3::{contract::{Contract, Options}, transports::Http, types::{Address, U256}, Web3};

use super::chain::Chain;

const CHAINLINK_ABI: &[u8] = include_bytes!("../../abi/chainlink.json");

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Balance {
    pub value: f64,
    pub usd_value: f64,
    pub currency: String,
    pub from_test_network: bool,
}
// TODO: balance timestamp

#[derive(Clone)]
pub struct Provider {
    web3: Web3<Http>,
    chain: Chain,
}

#[allow(dead_code)]
#[derive(Debug)]
struct PriceFeedData {
    round_id: U256,
    answer: i128,
    started_at: U256,
    updated_at: U256,
    answered_in_round: U256,
}

impl Balance {
    pub fn new(value: f64, usd_value: f64, currency: &str, from_test_network: bool) -> Self {
        Self { value, usd_value, currency: currency.to_string(), from_test_network }
    }

    pub fn to_string(&self) -> String {
        format!("{:.6} {} ({:.2} USD)", self.value, self.currency, self.usd_value)
    }
}

impl Chain {
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
    pub fn get_chainlink_contract_address(&self) -> Address {
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

impl Provider {
    pub fn new(endpoint_url: &str, chain: Chain) -> anyhow::Result<Self> {
        let transport = Http::new(&chain.finalize_endpoint_url(endpoint_url))?;
        let web3 = Web3::new(transport);
        Ok(Self {web3, chain})
    }

    pub async fn get_eth_balance(&self, account: Address) -> anyhow::Result<Balance> {
        let wei =  self.web3.eth().balance(account, None).await?;
        let eth = wei_to_eth(wei);
        let usd_value = self.get_eth_usd_rate().await? * eth;

        Ok(Balance::new(eth, usd_value, "ETH", self.chain.is_test_network()))
    }

    async fn get_eth_usd_rate(&self) -> anyhow::Result<f64> {
        let contrcat_address = self.chain.get_chainlink_contract_address();
        let contract = Contract::from_json(self.web3.eth(), contrcat_address, CHAINLINK_ABI)?;

        let result: PriceFeedData = contract
        .query("latestRoundData", (), None, Options::default(), None)
        .await
        .map(|(round_id, answer, started_at, updated_at, answered_in_round)| PriceFeedData {
            round_id,
            answer,
            started_at,
            updated_at,
            answered_in_round,
        })?;

        Ok(result.answer as f64 / 10f64.powi(8))
    }
}

fn wei_to_eth(wei: U256) -> f64 {
    let ether = web3::types::U256::exp10(18); // 1 Ether = 10^18 Wei
    let eth_value = wei.as_u128() as f64 / ether.as_u128() as f64;
    eth_value
}
