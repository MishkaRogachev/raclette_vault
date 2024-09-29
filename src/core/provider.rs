use web3::{contract::{Contract, Options}, transports::Http, types::{Address, U256}, Web3};

use super::{balance::{Balance, Balances}, chain::Chain, token::Token};

const CHAINLINK_ABI: &[u8] = include_bytes!("../../abi/chainlink.json");
const ERC20_ABI: &[u8] = include_bytes!("../../abi/erc20.json");

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

impl Provider {
    pub fn new(endpoint_url: &str, chain: Chain) -> anyhow::Result<Self> {
        let transport = Http::new(&chain.finalize_endpoint_url(endpoint_url))?;
        let web3 = Web3::new(transport);
        Ok(Self {web3, chain})
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

    pub async fn get_balances(&self, account: Address, tokens: &Vec<Token>) -> anyhow::Result<Balances> {
        let mut balances = Vec::new();
        let eth_usd_rate = self.get_eth_usd_rate().await?;

        for token in tokens {
            let balance = if token.symbol == "ETH" {
                // Handle ETH balance
                let wei = self.web3.eth().balance(account, None).await?;
                let eth = wei_to_eth(wei);
                Balance::new(eth, eth_usd_rate * eth, &token.symbol, self.chain.is_test_network())
            } else {
                // Handle ERC-20 token balances
                let token_address = token.address.parse::<Address>()?;
                let contract = Contract::from_json(self.web3.eth(), token_address, ERC20_ABI)?;

                let balance: U256 = contract
                    .query("balanceOf", (account,), None, Options::default(), None)
                    .await?;

                let balance_f64 = balance.as_u128() as f64 / 10f64.powi(token.decimals as i32);
                let usd_value = balance_f64 * self.get_eth_usd_rate().await?;
                Balance::new(balance_f64, usd_value, &token.symbol, self.chain.is_test_network())
            };

            balances.push(balance);
        }

        Ok(balances)
    }
}

fn wei_to_eth(wei: U256) -> f64 {
    let ether = web3::types::U256::exp10(18); // 1 Ether = 10^18 Wei
    let eth_value = wei.as_u128() as f64 / ether.as_u128() as f64;
    eth_value
}
