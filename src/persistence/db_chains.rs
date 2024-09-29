use crate::core::chain::Chain;
use super::db::Db;

const ACTIVE_NETWORKS: &[u8] = b"active_networks";

impl Db {
    pub fn save_active_networks(&self, chains: &[Chain]) -> anyhow::Result<()> {
        let serialized_chains = serde_json::to_vec(chains)?;
        self.insert(ACTIVE_NETWORKS, &serialized_chains)
    }

    pub fn get_active_networks(&self) -> anyhow::Result<Vec<Chain>> {
        let serialized_chains: Option<Vec<u8>> = self.get(ACTIVE_NETWORKS)?;
        if let Some(serialized_chains) = serialized_chains {
            let chains: Vec<Chain> = serde_json::from_slice(&serialized_chains)?;
            return Ok(chains);
        }
        Ok(vec![])
    }
}