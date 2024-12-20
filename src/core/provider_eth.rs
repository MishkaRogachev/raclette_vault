use web3::{
    contract::{Contract, Options},
    signing::SecretKey,
    types::*,
};

use super::{balance::{Balance, Balances}, eth_utils, provider::Provider, token::{Token, TokenList}, transaction::TransactionFees};

const ETH: &str = "ETH";

const CHAINLINK_ABI: &[u8] = include_bytes!("../../abi/chainlink.json");
const ERC20_BALANCE_ABI: &[u8] = include_bytes!("../../abi/erc20_balance.json");
const ERC20_TOKENS_ABI: &[u8] = include_bytes!("../../abi/erc20_tokens.json");

#[allow(dead_code)]
#[derive(Debug)]
struct PriceFeedData {
    round_id: U256,
    answer: i128,
    started_at: U256,
    updated_at: U256,
    answered_in_round: U256,
}

impl<T: web3::Transport> Provider<T> {
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

    pub async fn get_eth_usd_rate(&self) -> anyhow::Result<f64> {
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

    pub async fn get_eth_balance(&self, account: Address) -> anyhow::Result<Balance> {
        let wei = self.web3.eth().balance(account, None).await?;
        let eth = eth_utils::wei_to_eth(wei);
        let eth_usd_rate = self.get_eth_usd_rate().await?;
        Ok(Balance::new(ETH, self.chain, eth, eth_usd_rate * eth))
    }

    pub async fn get_token_balances(&self, account: Address, tokens: &TokenList) -> anyhow::Result<Balances> {
        let mut balances = Vec::new();
        let eth_usd_rate = self.get_eth_usd_rate().await?;

        for token in tokens {
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
            let balance = Balance::new(&token.symbol, self.chain ,balance_f64, balance_f64 * eth_usd_rate);
            balances.push(balance);
        }
        Ok(balances)
    }

    pub async fn estimate_transaction_fees(&self, transaction: TransactionParameters, from: Address) -> anyhow::Result<TransactionFees> {
        let gas_limit = match self.web3.eth().estimate_gas(
            CallRequest {
                from: Some(from),
                to: transaction.to,
                gas: None,
                gas_price: None,
                value: Some(transaction.value),
                data: Some(transaction.data),
                ..Default::default()
            },
            None
        )
        .await {
            Ok(gas) => gas,
            Err(_) => {
                // TODO: handle specific errors
                return Ok(TransactionFees::NotEnoughFunds { currency: ETH.to_string() });
            }
        };

        let gas_price: U256 = self.web3.eth().gas_price().await?;
        let wei = gas_limit * gas_price;

        Ok(TransactionFees::Estimated { currency: ETH.to_string(), amount: eth_utils::wei_to_eth(wei) })
    }

    pub async fn send_transaction(&self, transaction: TransactionParameters, sender: H160, secret_key: &SecretKey) -> anyhow::Result<H256> {
        let nonce = self.web3.eth().transaction_count(sender, None).await?;

        let signed = self.web3.accounts()
            .sign_transaction(TransactionParameters {
                nonce: Some(nonce), // Explicitly set the nonce
                ..transaction
            }, secret_key)
            .await?;
        let tx_hash = self.web3.eth()
            .send_raw_transaction(signed.raw_transaction)
            .await?;
        Ok(tx_hash)
    }

    pub async fn get_transaction(&self, tx_hash: H256) -> anyhow::Result<Option<Transaction>> {
        match self.web3.eth().transaction(TransactionId::Hash(tx_hash)).await {
            Ok(receipt) => Ok(receipt),
            Err(err) => Err(anyhow::anyhow!("Failed to get transaction: {}", err)),
        }
    }

    #[allow(dead_code)]
    pub async fn get_transaction_receipt(&self, tx_hash: H256) -> anyhow::Result<Option<TransactionReceipt>> {
        match self.web3.eth().transaction_receipt(tx_hash).await {
            Ok(receipt) => Ok(receipt),
            Err(err) => Err(anyhow::anyhow!("Failed to get transaction receipt: {}", err)),
        }
    }
}
