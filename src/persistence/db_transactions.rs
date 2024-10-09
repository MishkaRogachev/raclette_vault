
use web3::types::{Address, H256};

use super::db::Db;
use crate::core::transaction::TransactionResult;

const ETH_TRANSACTIONS: &[u8] = b"tx_eth";

impl Db {
    pub fn save_transaction(&self, account: Address, transaction: &TransactionResult) -> anyhow::Result<()> {
        let key = transaction_synthetic_id(account, transaction.tx_hash);
        self.upsert(&key, &transaction, false)?;
        Ok(())
    }

    pub fn get_transactions(&self, account: Address, cursor: usize, count: usize ) -> anyhow::Result<Vec<TransactionResult>> {
        let mut prefix = ETH_TRANSACTIONS.to_vec();
        prefix.extend_from_slice(account.as_bytes());

        self.scan_prefix(&prefix, cursor, count, false)
    }
}

fn transaction_synthetic_id(account: Address, tx_hash: H256) -> Vec<u8> {
    let mut key = ETH_TRANSACTIONS.to_vec();
    key.extend_from_slice(account.as_bytes());
    key.extend_from_slice(tx_hash.as_bytes());
    key
}
