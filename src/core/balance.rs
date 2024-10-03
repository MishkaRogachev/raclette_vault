use std::collections::HashMap;

use super::eth_chain::EthChain;
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BalanceValue {
    pub value: f64,
    pub usd_value: f64,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Balance {
    pub currency: String,
    pub chain_values: HashMap<EthChain, BalanceValue>,
}

pub type Balances = Vec<Balance>;

impl BalanceValue {
    pub fn new(value: f64, usd_value: f64) -> Self {
        Self { value, usd_value }
    }
}

impl Balance {
    pub fn new(currency: &str, chain: EthChain, value: f64, usd_value: f64) -> Self {
        let mut chain_values = HashMap::new();
        chain_values.insert(chain, BalanceValue::new(value, usd_value));
        Self { currency: currency.to_string(), chain_values }
    }

    pub fn summary(&self) -> BalanceValue {
        self.chain_values.values().fold(BalanceValue::new(0.0, 0.0), |acc, v| {
            BalanceValue::new(acc.value + v.value, acc.usd_value + v.usd_value)
        })
    }

    pub fn from_test_network(&self) -> bool {
        self.chain_values.keys().any(|k| k.is_test_network())
    }

    pub fn extend_balances(mut balances: Vec<Self>, other_balances: &Vec<Self>) -> Vec<Self> {
        for other in other_balances {
            if let Some(balance) = balances.iter_mut().find(|b| b.currency == other.currency) {
                balance.chain_values.extend(other.chain_values.clone());
            } else {
                balances.push(other.clone());
            }
        }
        balances
    }
}
