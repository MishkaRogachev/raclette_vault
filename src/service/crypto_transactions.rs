use web3::{signing::SecretKey, types::TransactionParameters};

use crate::core::{eth_chain, eth_utils, transaction::*};
use super::crypto::Crypto;

const ERR_NO_TRANSACTION_FOUND: &str = "No transaction found";

impl Crypto {
    pub async fn estimate_transaction_fees(&self, request: TransactionRequest) -> anyhow::Result<TransactionFees> {
        if request.currency != "ETH" { // TODO: token transactions
            return Err(anyhow::anyhow!("Non-ETH transactions are not supported yet"));
        }

        let provider = self.providers.get(&request.chain).ok_or_else(|| 
            anyhow::anyhow!(format!("No provider for chain {}", request.chain)))?;

        let transaction = TransactionParameters {
            to: Some(request.to),
            value: eth_utils::eth_to_wei(request.amount),
            ..Default::default()
        };

        provider.estimate_transaction_fees(transaction, request.from).await
    }

    pub async fn send_transaction(&self, request: TransactionRequest, secret_key: &SecretKey) -> anyhow::Result<TransactionResult> {
        if request.currency != "ETH" { // TODO: token transactions
            return Err(anyhow::anyhow!("Non-ETH transactions are not supported yet"));
        }

        let provider = self.providers.get(&request.chain).ok_or_else(|| 
            anyhow::anyhow!(format!("No provider for chain {}", request.chain)))?;

        let transaction = TransactionParameters {
            to: Some(request.to),
            value: eth_utils::eth_to_wei(request.amount),
            ..Default::default()
        };

        let tx_hash = provider.send_transaction(transaction, &secret_key).await?;
        let tx = provider.get_transaction(tx_hash).await?
            .ok_or_else(|| anyhow::anyhow!(ERR_NO_TRANSACTION_FOUND))?;

        let transaction = to_transaction_result(&tx, request.chain);
        if let Some(account) = tx.from {
            self.db.save_transaction(account, &transaction)?;
        }

        Ok(transaction)
    }
}

fn to_transaction_result(transaction: &web3::types::Transaction, chain: eth_chain::EthChain) -> TransactionResult {
    let status = if transaction.block_number.is_some() {
        TransactionStatus::Successed
    } else {
        TransactionStatus::Pending
    };

    let fee = transaction
        .gas_price
        .map_or(0.0, |gas_price| eth_utils::wei_to_eth(transaction.gas * gas_price));

    TransactionResult {
        hash: transaction.hash,
        block_number: transaction.block_number,
        from: transaction.from,
        to: transaction.to,
        amount: eth_utils::wei_to_eth(transaction.value),
        fee,
        chain,
        status,
    }
}