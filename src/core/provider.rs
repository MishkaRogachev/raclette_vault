use web3::{
    contract::{Contract, Options},
    transports::Http,
    types::{Address, U256},
    Web3
};

use super::{balance::{Balance, Balances}, chain::Chain, token::{Token, TokenList}};

const CHAINLINK_ABI: &[u8] = include_bytes!("../../abi/chainlink.json");
const ERC20_BALANCE_ABI: &[u8] = include_bytes!("../../abi/erc20_balance.json");
const ERC20_TOKENS_ABI: &[u8] = include_bytes!("../../abi/erc20_tokens.json");

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

    #[allow(dead_code)]
    pub async fn get_token_metadata(&self, contract_address: Address) -> anyhow::Result<Token> {
        let contract = Contract::from_json(self.web3.eth(), contract_address, ERC20_TOKENS_ABI)?;

        let name: String = contract
            .query("name", (), None, Options::default(), None)
            .await?;
        let symbol: String = contract
            .query("symbol", (), None, Options::default(), None)
            .await?;
        let decimals: U256 = contract
            .query("decimals", (), None, Options::default(), None)
            .await?;

        Ok(Token::new(&name, &symbol).with_chain_data(self.chain, contract_address, decimals.as_u64() as u16))
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

    pub async fn get_balances(&self, account: Address, tokens: &TokenList) -> anyhow::Result<Balances> {
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
                let token_chain_data = match token.get_chain_data(&self.chain) {
                    Some(token_chain_data) => token_chain_data,
                    None => continue,
                };
                let contract = match Contract::from_json(self.web3.eth(), token_chain_data.contract_address, ERC20_BALANCE_ABI) {
                    Ok(contract) => contract,
                    Err(_) => {
                        log::warn!("Failed to create contract for token {} on {}", token.symbol, self.chain);
                        continue
                    },
                };

                let balance: U256 = match contract
                    .query("balanceOf", (account,), None, Options::default(), None).await {
                    Ok(balance) => balance,
                    Err(_) => {
                        log::warn!("Failed to get balance for token {} on {}", token.symbol, self.chain);
                        continue
                    },
                };

                let balance_f64 = balance.as_u128() as f64 / 10f64.powi(token_chain_data.decimals as i32);
                Balance::new(balance_f64, balance_f64 * eth_usd_rate, &token.symbol, self.chain.is_test_network())
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
