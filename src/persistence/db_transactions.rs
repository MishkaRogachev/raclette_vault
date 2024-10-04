use std::any;

use web3::types::Address;

use super::db::Db;

use crate::core::transaction::TransactionResult;

const ACCOUNT_TRANSACTIONS: &[u8] = b"active_networks";

impl Db {
    pub fn save_transaction(&self, account: Address, transaction: &TransactionResult) -> anyhow::Result<()> {
        // TODO: implement
        Ok(())
    }
}