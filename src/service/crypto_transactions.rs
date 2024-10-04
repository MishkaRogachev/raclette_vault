
use web3::{signing::SecretKey, types::{TransactionParameters, U64}};

use crate::core::{eth_utils, transaction::{TransactionRequest, TransactionResult}};
use super::crypto::Crypto;

const ERR_NO_TRANSACTION_FOUND: &str = "No transaction found";

impl Crypto {
    pub async fn send_transaction(&self, request: TransactionRequest, secret_key: &SecretKey) -> anyhow::Result<TransactionResult> {
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

        let tx_hash = provider.send_transaction(transaction, &secret_key).await?;

        let receipt = provider.get_transaction_receipt(tx_hash).await?
            .ok_or_else(|| anyhow::anyhow!(ERR_NO_TRANSACTION_FOUND))?;

        let successed = receipt.status == Some(U64::one());
        let response = TransactionResult {
            tx_hash,
            block_number: receipt.block_number,
            from: Some(receipt.from),
            to: receipt.to,
            amount: request.value,
            fee: eth_utils::wei_to_eth(receipt.effective_gas_price.unwrap_or_default() * receipt.gas_used.unwrap_or_default()),
            chain: request.chain,
            successed,
        };

        self.db.save_transaction(receipt.from, &response)?;

        Ok(response)
    }
}
