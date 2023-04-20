use crate::{account::Account, util::mc_wallet_dir};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Storage {
    pub accounts: Vec<Account>,
}

impl Storage {
    pub fn get_from_file() -> Result<Self> {
        let storage_file = mc_wallet_dir()?.join("storage.json");
        let storage_json = std::fs::read_to_string(storage_file)?;
        let storage: Storage = serde_json::from_str(&storage_json)?;
        Ok(storage)
    }

    pub fn save_to_file(&self) -> Result<()> {
        let storage_file = mc_wallet_dir()?.join("storage.json");
        let storage_json = serde_json::to_string_pretty(&self)?;
        std::fs::write(storage_file, storage_json)?;
        Ok(())
    }
}
