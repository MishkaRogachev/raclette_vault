
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Balance {
    pub value: f64,
    pub usd_value: f64,
    pub currency: String,
    pub from_test_network: bool,
}

pub type Balances = Vec<Balance>;

impl Balance {
    pub fn new(value: f64, usd_value: f64, currency: &str, from_test_network: bool) -> Self {
        Self { value, usd_value, currency: currency.to_string(), from_test_network }
    }

    pub fn extend_balances(balances: &Vec<Self>, new_balances: &Vec<Self>) -> Vec<Self> {
        let mut extended_balances = balances.clone();
        for new_balance in new_balances {
            if let Some(balance) = extended_balances.iter_mut().find(|b| b.currency == new_balance.currency) {
                balance.value += new_balance.value;
                balance.usd_value += new_balance.usd_value;
            } else {
                extended_balances.push(new_balance.clone());
            }
        }
        extended_balances
    }
}
