use crate::core::eth_chain::EthChain;
use super::db::Db;

const ACTIVE_NETWORKS: &[u8] = b"active_networks";

impl Db {
    pub fn save_active_networks(&self, chains: &[EthChain]) -> anyhow::Result<()> {
        let serialized_chains = serde_json::to_vec(chains)?;
        self.upsert(ACTIVE_NETWORKS, &serialized_chains, true)
    }

    pub fn get_active_networks(&self) -> anyhow::Result<Vec<EthChain>> {
        let serialized_chains: Option<Vec<u8>> = self.get(ACTIVE_NETWORKS, true)?;
        if let Some(serialized_chains) = serialized_chains {
            let chains: Vec<EthChain> = serde_json::from_slice(&serialized_chains)?;
            return Ok(chains);
        }
        Ok(vec![])
    }
}