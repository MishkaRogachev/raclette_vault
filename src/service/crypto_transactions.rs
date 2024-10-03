use web3::{signing::SecretKey, types::{Address, TransactionParameters, H256, U64}};

use crate::core::{chain::Chain, eth_utils};
use super::crypto::Crypto;

const ERR_NO_TRANSACTION_FOUND: &str = "No transaction found";

pub struct TransactionRequest {
    secret_key: SecretKey,
    from: Address,
    to: Address,
    value: f64,
    currency: String,
    chain: Chain,
}

pub struct TransactionResponse {
    pub tx_hash: H256,
    pub block_number: Option<U64>,
    pub from: Option<Address>,
    pub to: Option<Address>,
    pub amount: f64,
    pub fee: f64,
    pub chain: Chain,
    pub successed: bool,
}


impl Crypto {
    pub async fn send_transaction(&self, request: TransactionRequest) -> anyhow::Result<TransactionResponse> {
        if request.currency != "ETH" { // TODO: token transactions
            return Err(anyhow::anyhow!("Non-ETH transactions are not supported yet"));
        }

        let provider = self.providers.get(&request.chain).ok_or_else(|| 
            anyhow::anyhow!(format!("No provider for chain {}", request.chain)))?;

        let transaction = TransactionParameters {
            to: Some(request.to),
            value: eth_utils::eth_to_wei(request.value),
            ..Default::default()
        };

        let tx_hash = provider.send_transaction(transaction, &request.secret_key).await?;

        let receipt = provider.get_transaction_receipt(tx_hash).await?
            .ok_or_else(|| anyhow::anyhow!(ERR_NO_TRANSACTION_FOUND))?;

        let successed = receipt.status == Some(U64::one());
        Ok(TransactionResponse {
            tx_hash,
            block_number: receipt.block_number,
            from: Some(receipt.from),
            to: receipt.to,
            amount: request.value,
            fee: eth_utils::wei_to_eth(receipt.effective_gas_price.unwrap_or_default() * receipt.gas_used.unwrap_or_default()),
            chain: request.chain,
            successed,
        })
    }
}
