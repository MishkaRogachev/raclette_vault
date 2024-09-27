use web3::{contract::{Contract, Options}, transports::Http, types::{Address, U256}, Web3};

use super::chain::Chain;

const CHAINLINK_ABI: &[u8] = include_bytes!("../../abi/chainlink.json");

pub struct Balance {
    pub value: f64,
    pub usd_value: f64,
    pub currency: String,
}

pub struct Provider {
    web3: Web3<Http>,
    chain: Chain,
}

#[derive(Debug)]
struct PriceFeedData {
    round_id: U256,
    answer: i128,
    started_at: U256,
    updated_at: U256,
    answered_in_round: U256,
}

impl Balance {
    pub fn new(value: f64, usd_value: f64, currency: String) -> Self {
        Self {value, usd_value, currency}
    }

    pub fn to_string(&self) -> String {
        format!("{:.6} {} ({:.2} USD)", self.value, self.currency, self.usd_value)
    }
}

impl Chain {
    pub fn get_infura_url(&self, infura_token: &str) -> String {
        let chain_name = match self {
            Chain::EthereumMainnet => "mainnet",
            Chain::Optimism => "optimism",
            Chain::Arbitrum => "arbitrum",
            Chain::Goerli => "goerli",
            Chain::Kovan => "kovan",
            Chain::Rinkeby => "rinkeby",
        };
        format!("https://{}.infura.io/v3/{}", chain_name, infura_token)
    }

    pub fn get_chainlink_contract_address(&self) -> Address {
        match self {
            Chain::EthereumMainnet => "0x5f4ec3df9cbd43714fe2740f5e3616155c5b8419",
            Chain::Optimism => "0x13e3Ee699D1909E989722E753853AE30b17e08c5",
            Chain::Arbitrum => "0x639Fe6ab55C921f74e7fac1ee960C0B6293ba612",
            Chain::Goerli => "0xD4a33860578De61DBAbDc8BFdb98FD742fA7028e",
            Chain::Kovan => "0x9326BFA02ADD2366b30bacB125260Af641031331",
            Chain::Rinkeby => "0x8A753747A1Fa494EC906cE90E9f37563A8AF630e",
        }
        .parse()
        .unwrap()
    }
}

impl Provider {
    pub fn new(infura_token: &str, chain: Chain) -> anyhow::Result<Self> {
        let transport = Http::new(&chain.get_infura_url(infura_token))?;
        let web3 = Web3::new(transport);
        Ok(Self {web3, chain})
    }

    pub async fn get_eth_balance(&self, account: Address) -> anyhow::Result<Balance> {
        let wei =  self.web3.eth().balance(account, None).await?;
        let eth = wei_to_eth(wei);
        let usd_value = self.get_eth_usd_rate().await? * eth;

        Ok(Balance::new(eth, usd_value, "ETH".to_string()))
    }

    async fn get_eth_usd_rate(&self) -> anyhow::Result<f64> {
        let contrcat_address = self.chain.get_chainlink_contract_address();
        let contract = Contract::from_json(self.web3.eth(), contrcat_address, CHAINLINK_ABI).unwrap();

        let result: PriceFeedData = contract
        .query("latestRoundData", (), None, Options::default(), None)
        .await
        .map(|(round_id, answer, started_at, updated_at, answered_in_round)| PriceFeedData {
            round_id,
            answer,
            started_at,
            updated_at,
            answered_in_round,
        })
        .unwrap();

        Ok(result.answer as f64 / 10f64.powi(8))
    }
}

fn wei_to_eth(wei: U256) -> f64 {
    let ether = web3::types::U256::exp10(18); // 1 Ether = 10^18 Wei
    let eth_value = wei.as_u128() as f64 / ether.as_u128() as f64;
    eth_value
}
