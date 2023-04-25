use std::path::PathBuf;

use crate::{account::Account, crypto, util::mc_wallet_dir};
use anyhow::{anyhow, Result};
use bip32::{Mnemonic, Seed};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};

fn storage_file() -> Result<PathBuf> {
    Ok(mc_wallet_dir()?.join("storage.json"))
}

pub struct CreateSeedResponse {
    pub mnemonic: String,
    pub seed: Seed,
    pub encrypted_seed_hex: String,
}

fn create_seed(password: &str) -> Result<CreateSeedResponse> {
    let mnemonic = Mnemonic::random(OsRng, Default::default());
    let seed = mnemonic.to_seed(password);
    let mnemonic = mnemonic.phrase().to_string();
    let encrypted_seed = crypto::encrypt(password.as_bytes(), seed.as_bytes())?;
    let encrypted_seed_hex = hex::encode(encrypted_seed);

    Ok(CreateSeedResponse {
        mnemonic,
        seed,
        encrypted_seed_hex,
    })
}

fn decrypt_seed(password: &str, encrypted_seed_hex: &str) -> Result<Seed> {
    let encrypted_seed = hex::decode(encrypted_seed_hex)?;
    let seed = crypto::decrypt(password.as_bytes(), &encrypted_seed)?;
    let seed_bytes_fixed: [u8; 64] = seed
        .as_slice()
        .try_into()
        .map_err(|_| anyhow!("could not convert seed bytes to fixed size array!"))?;
    Ok(Seed::new(seed_bytes_fixed))
}

pub trait Storage {
    fn add_account(&mut self, account: Account) -> Result<()>;
    fn get_new_account_index(&self) -> Result<u32>;
    fn create_seed(&mut self, password: &str) -> Result<CreateSeedResponse>;
    fn get_seed(&self, password: &str) -> Result<Seed>;
    fn is_created(&self) -> bool;
}

pub struct MemStorage {
    pub accounts: Vec<Account>,
    pub encrypted_seed_hex: Option<String>,
}

impl Storage for MemStorage {
    fn get_new_account_index(&self) -> Result<u32> {
        get_new_account_index(&self.accounts)
    }

    fn add_account(&mut self, account: Account) -> Result<()> {
        self.accounts.push(account);
        Ok(())
    }

    fn create_seed(&mut self, password: &str) -> Result<CreateSeedResponse> {
        let res = create_seed(password)?;
        self.encrypted_seed_hex = Some(res.encrypted_seed_hex.clone());
        Ok(res)
    }

    fn get_seed(&self, password: &str) -> Result<Seed> {
        let encrypted_seed_hex = self
            .encrypted_seed_hex
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("no seed"))?;
        decrypt_seed(password, encrypted_seed_hex)
    }

    fn is_created(&self) -> bool {
        self.encrypted_seed_hex.is_some()
    }
}

impl MemStorage {
    pub fn new() -> Self {
        Self {
            accounts: vec![],
            encrypted_seed_hex: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileStorage {
    pub accounts: Vec<Account>,
    pub encrypted_seed_hex: Option<String>,
}

fn get_new_account_index(accounts: &[Account]) -> Result<u32> {
    let mut index = 0;
    for account in accounts {
        if account.account_id > index {
            index = account.account_id;
        }
    }
    Ok(index + 1)
}

impl Storage for FileStorage {
    fn add_account(&mut self, account: Account) -> Result<()> {
        self.accounts.push(account);
        self.save_to_file()?;
        Ok(())
    }

    fn get_new_account_index(&self) -> Result<u32> {
        get_new_account_index(&self.accounts)
    }

    fn create_seed(&mut self, password: &str) -> Result<CreateSeedResponse> {
        let res = create_seed(password)?;
        self.accounts = vec![];
        self.encrypted_seed_hex = Some(res.encrypted_seed_hex.clone());
        self.save_to_file()?;
        Ok(res)
    }

    fn get_seed(&self, password: &str) -> Result<Seed> {
        let encrypted_seed_hex = self
            .encrypted_seed_hex
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("no seed"))?;
        decrypt_seed(password, encrypted_seed_hex)
    }

    fn is_created(&self) -> bool {
        self.encrypted_seed_hex.is_some()
    }
}

impl FileStorage {
    fn create_config_dir() -> Result<()> {
        let config_dir = dirs::config_dir().ok_or_else(|| anyhow!("could not get config dir!"))?;
        let mc_wallet_dir = config_dir.join("mc-wallet");
        std::fs::create_dir_all(mc_wallet_dir)?;
        Ok(())
    }

    fn init() -> Result<()> {
        Self::create_config_dir()?;
        let storage_file = storage_file()?;
        if !storage_file.exists() {
            let storage = FileStorage {
                accounts: vec![],
                encrypted_seed_hex: None,
            };
            let storage_json = serde_json::to_string_pretty(&storage)?;
            std::fs::write(storage_file, storage_json)?;
        }
        Ok(())
    }

    pub fn get_from_file() -> Result<Self> {
        Self::init()?;
        let storage_file = storage_file()?;
        let storage_json = std::fs::read_to_string(storage_file)?;
        let storage: FileStorage = serde_json::from_str(&storage_json)?;
        Ok(storage)
    }

    pub fn save_to_file(&self) -> Result<()> {
        Self::init()?;
        let storage_file = storage_file()?;
        let storage_json = serde_json::to_string_pretty(&self)?;
        std::fs::write(storage_file, storage_json)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_seed() {
        let mut storage = MemStorage::new();
        let res = storage
            .create_seed("password")
            .expect("could not create seed");
    }

    #[test]
    fn save_and_get_seed() {
        let mut storage = MemStorage::new();
        let res = storage
            .create_seed("password")
            .expect("could not create seed");
        let seed = storage
            .get_seed("password")
            .expect("could not get seed from storage");
        assert_eq!(res.seed.as_bytes(), seed.as_bytes());
    }
}
