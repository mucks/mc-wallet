use anyhow::Result;
use bip32::{Seed, XPrv};
use serde::{Deserialize, Serialize};

use crate::CoinType;

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub coin_type: CoinType,
    pub account_id: u32,
}

impl Account {
    pub fn new(coin_type: CoinType, account_index: u32) -> Self {
        Self {
            coin_type,
            account_id: account_index,
        }
    }

    pub fn new_with_random_index(coin_type: CoinType) -> Self {
        let account_index = rand::random::<u32>();
        Self {
            coin_type,
            account_id: account_index,
        }
    }

    pub fn child_path(&self) -> String {
        format!("m/44'/{}'/{}'/0/0", self.coin_type as u64, self.account_id)
    }

    pub fn private_key(&self, seed: &Seed) -> Result<XPrv> {
        let child_path = self.child_path();
        let child_xprv = XPrv::derive_from_path(seed, &child_path.parse()?)?;
        Ok(child_xprv)
    }
}
