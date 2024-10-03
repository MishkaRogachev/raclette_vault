use web3::types::U256;

pub fn wei_to_eth(wei: U256) -> f64 {
    let ether = web3::types::U256::exp10(18); // 1 Ether = 10^18 Wei
    let eth_value = wei.as_u128() as f64 / ether.as_u128() as f64;
    eth_value
}

pub fn eth_to_wei(eth: f64) -> U256 {
    let ether = web3::types::U256::exp10(18); // 1 Ether = 10^18 Wei
    let wei_value = (eth * ether.as_u128() as f64) as u128;
    U256::from(wei_value)
}
