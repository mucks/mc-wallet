use anyhow::Result;
use bip32::{Seed, XPrv};
use serde::{Deserialize, Serialize};

use crate::{coin_type::CoinType, util::gen_private_key};

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

    pub fn new_with_random_index(coin_type: CoinType, seed: &Seed) -> Result<Self> {
        let account_index = rand::random::<u32>();
        Self::new(coin_type, account_index, seed)
    }
}
