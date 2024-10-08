
use web3::types::{Address, H256};

use super::db::Db;
use crate::core::transaction::TransactionResult;

const ETH_TRANSACTIONS: &[u8] = b"tx_eth";

impl Db {
    pub fn save_transaction(&self, account: Address, transaction: &TransactionResult) -> anyhow::Result<()> {
        // TODO: probably, there is no sense to encrypt transaction data
        let key = transaction_synthetic_id(account, transaction.tx_hash);
        self.insert(&key, &transaction)?;
        Ok(())
    }

    pub fn get_transactions(&self, account: Address, cursor: usize, count: usize ) -> anyhow::Result<Vec<TransactionResult>> {
        let mut prefix = ETH_TRANSACTIONS.to_vec();
        prefix.extend_from_slice(account.as_bytes());

        let mut transactions = Vec::new();
        let iter = self.scan_prefix(&prefix).skip(cursor).take(count);

        for result in iter {
            let (_key, decrypted_value) = result?;
            let transaction: TransactionResult = serde_json::from_slice(&decrypted_value)?;
            transactions.push(transaction);
        }
        Ok(transactions)
    }
}

fn transaction_synthetic_id(account: Address, tx_hash: H256) -> Vec<u8> {
    let mut key = ETH_TRANSACTIONS.to_vec();
    key.extend_from_slice(account.as_bytes());
    key.extend_from_slice(tx_hash.as_bytes());
    key
}