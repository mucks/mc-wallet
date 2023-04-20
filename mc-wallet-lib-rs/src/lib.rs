use std::fmt::Display;

use anyhow::{anyhow, Result};
use bip32::{Mnemonic, Seed, XPrv};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};

mod crypto;
mod java;

pub fn create_mnemonic() -> String {
    let mnemonic = Mnemonic::random(OsRng, Default::default());
    mnemonic.phrase().to_string()
}

pub fn create_config_dir() -> Result<()> {
    let config_dir = dirs::config_dir().ok_or_else(|| anyhow!("could not get config dir!"))?;
    let mc_wallet_dir = config_dir.join("mc-wallet");
    std::fs::create_dir_all(mc_wallet_dir)?;
    Ok(())
}

pub fn create_and_save_seed(
    mnemonic: &str,
    password: &str,
    encryption_password: &str,
) -> Result<()> {
    let seed = create_seed(mnemonic, password)?;
    save_seed(&seed, encryption_password)?;
    Ok(())
}

pub fn create_seed(mnemonic: &str, password: &str) -> Result<Seed> {
    let mnemonic = Mnemonic::new(mnemonic, Default::default())?;
    let seed = mnemonic.to_seed(password);
    Ok(seed)
}

pub fn save_seed(seed: &Seed, encryption_password: &str) -> Result<()> {
    let seed_file = mc_wallet_dir()?.join("seed");
    let encrypted_seed = crypto::encrypt(encryption_password.as_bytes(), seed.as_bytes())?;
    let encrypted_seed_hex = hex::encode(encrypted_seed);
    std::fs::write(seed_file, encrypted_seed_hex)?;
    Ok(())
}

pub fn mc_wallet_dir() -> Result<std::path::PathBuf> {
    let config_dir = dirs::config_dir().ok_or_else(|| anyhow!("could not get config dir!"))?;
    let mc_wallet_dir = config_dir.join("mc-wallet");
    Ok(mc_wallet_dir)
}

pub fn get_seed(encryption_password: &str) -> Result<Seed> {
    let seed_file = mc_wallet_dir()?.join("seed");
    let encrypted_seed_hex = std::fs::read_to_string(seed_file)?;
    let encrypted_seed = hex::decode(encrypted_seed_hex)?;
    let seed_bytes = crypto::decrypt(encryption_password.as_bytes(), &encrypted_seed)?;
    let seed_bytes_fixed: [u8; 64] = seed_bytes
        .as_slice()
        .try_into()
        .map_err(|_| anyhow!("could not convert seed bytes to fixed size array!"))?;

    Ok(Seed::new(seed_bytes_fixed))
}

pub fn create_eth_account(seed: &Seed) -> Result<()> {
    let child_path = "m/44'/60'/0'/0/0";
    create_account(seed, child_path)
}

pub fn create_account(seed: &Seed, child_path: &str) -> Result<()> {
    let child_xprv = XPrv::derive_from_path(&seed, &child_path.parse()?)?;

    Ok(())
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CoinType {
    ETH = 60,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub coin_type: CoinType,
    pub account: u64,
}

impl Account {
    pub fn child_path(&self) -> String {
        format!("m/44'/{}'/{}'/0/0", self.coin_type as u64, self.account)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Storage {
    pub accounts: Vec<Account>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_mnemonic() {
        let mnemonic = create_mnemonic();
        assert_eq!(mnemonic.split_whitespace().count(), 24);
    }

    #[test]
    fn test_create_config_dir() {
        create_config_dir().expect("could not create config dir");
    }

    #[test]
    fn test_create_seed() {
        let mnemonic = create_mnemonic();
        let seed = create_seed(&mnemonic, "password").expect("could not create seed");
        assert_eq!(seed.as_bytes().len(), 64);
    }

    #[test]
    fn save_and_get_seed() {
        let mnemonic = create_mnemonic();
        let seed = create_seed(&mnemonic, "password").expect("could not create seed");
        save_seed(&seed, "enc_password").expect("could not save seed");
        let seed2 = get_seed("enc_password").expect("could not get seed");
        assert_eq!(seed.as_bytes(), seed2.as_bytes());
    }
}
