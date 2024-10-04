use web3::types::{Address, H256, U64};

use super::eth_chain::EthChain;

pub struct TransactionRequest {
    pub from: Address,
    pub to: Address,
    pub amount: f64,
    pub currency: String,
    pub chain: EthChain,
}

pub struct TransactionResult {
    pub tx_hash: H256,
    pub block_number: Option<U64>,
    pub from: Option<Address>,
    pub to: Option<Address>,
    pub amount: f64,
    pub fee: f64,
    pub chain: EthChain,
    pub successed: bool,
}
