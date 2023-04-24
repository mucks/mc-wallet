use anyhow::Result;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CoinType {
    Eth = 60,
    Sol = 501,
    Sui = 784,
}

impl CoinType {
    pub fn address(&self, public_key: &[u8]) -> Result<String> {
        eth_address(public_key)
    }
}

pub fn eth_address(public_key: &[u8]) -> Result<String> {
    let mut hasher = Keccak256::new();
    hasher.update(public_key);
    let result = hasher.finalize();
    if result.len() < 20 {
        return Err(anyhow::anyhow!("invalid public key"));
    }
    Ok(format!("0x{}", hex::encode(&result[12..])))
}
