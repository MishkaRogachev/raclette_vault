
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Token {
    pub name: String,
    pub symbol: String,
    pub address: String,
    pub decimals: u16,
}

impl Token {
    pub fn new(name: &str, symbol: &str, address: &str, decimals: u16) -> Self {
        Self {
            name: name.to_string(),
            symbol: symbol.to_string(),
            address: address.to_string(),
            decimals,
        }
    }
}
