use web3::types::{Address, U256};

pub const ERR_INVALID_ADDRESS_LENGTH: &str = "Invalid address length. Ethereum address must be 42 characters long.";
pub const ERR_INVALID_ADDRESS_PREFIX: &str = "Ethereum address must start with '0x'.";
pub const ERR_INVALID_ADDRESS: &str = "Invalid Ethereum address format. It must be a valid hexadecimal string.";

pub fn wei_to_eth(wei: U256) -> f64 {
    let ether = web3::types::U256::exp10(18); // 1 Ether = 10^18 Wei
    let eth_value = wei.as_u128() as f64 / ether.as_u128() as f64;
    eth_value
}

#[allow(dead_code)]
pub fn eth_to_wei(eth: f64) -> U256 {
    let ether = web3::types::U256::exp10(18); // 1 Ether = 10^18 Wei
    let wei_value = (eth * ether.as_u128() as f64) as u128;
    U256::from(wei_value)
}

pub fn str_to_eth_address(address: &str) -> anyhow::Result<Address> {
    if !address.starts_with("0x") {
        return Err(anyhow::anyhow!(ERR_INVALID_ADDRESS_PREFIX));
    }

    if address.len() != 42 {
        return Err(anyhow::anyhow!(ERR_INVALID_ADDRESS_LENGTH));
    }

    match address.parse::<Address>() {
        Ok(valid_address) => Ok(valid_address),
        Err(_) => Err(anyhow::anyhow!(ERR_INVALID_ADDRESS)),
    }
}
