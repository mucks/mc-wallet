use anyhow::Result;
use bip32::{Seed, XPrv};
use serde::{Deserialize, Serialize};

use crate::{
    coin_type::CoinType,
    storage::{self, Storage},
    util::gen_private_key,
};

pub fn create_account(seed: &Seed, coin_type: CoinType, storage: &mut dyn Storage) -> Result<()> {
    let rnd_index = storage.get_new_account_index()?;
    let account = Account::new(coin_type, rnd_index, &seed)?;
    storage.add_account(account)?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub coin_type: CoinType,
    pub account_id: u32,
    pub address: String,
    pub public_key_hex: String,
    pub child_path: String,
}

impl Account {
    pub fn new(coin_type: CoinType, account_index: u32, seed: &Seed) -> Result<Self> {
        let child_path = format!("m/44'/{}'/{}'/0/0", coin_type as u64, account_index);
        let priv_key = gen_private_key(&child_path, seed)?;
        let public_key_bytes = priv_key.public_key().to_bytes();
        let public_key_hex = hex::encode(public_key_bytes);

        Ok(Self {
            coin_type,
            account_id: account_index,
            address: coin_type.address(&public_key_bytes)?,
            public_key_hex,
            child_path,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_account() -> Result<()> {
        let mut storage = storage::MemStorage::new();
        let res = storage.create_seed("password")?;
        create_account(&res.seed, CoinType::Eth, &mut storage)?;
        Ok(())
    }
}
