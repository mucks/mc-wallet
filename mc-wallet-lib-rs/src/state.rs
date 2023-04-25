use std::sync::Mutex;

use anyhow::Result;
use bip32::{Seed, XPrv};
use lazy_static::lazy_static;

use crate::{
    storage::{self, Storage},
    util::clone_seed,
};

pub struct State {
    pub seed: Option<Seed>,
}

lazy_static! {
    static ref STATE: Mutex<State> = Mutex::new(State { seed: None });
}

pub fn get_seed_from_state() -> Result<Seed> {
    let state = STATE
        .lock()
        .map_err(|err| anyhow::anyhow!("could not lock state: {err}"))?;

    if let Some(seed) = &state.seed {
        return Ok(clone_seed(seed));
    }

    Err(anyhow::anyhow!("wallet is locked"))
}

pub fn lock_wallet() -> Result<()> {
    let mut state = STATE
        .lock()
        .map_err(|err| anyhow::anyhow!("could not lock state: {err}"))?;
    state.seed = None;
    Ok(())
}

pub fn state_unlock_wallet(encryption_password: &str, storage: &dyn Storage) -> Result<()> {
    let mut state = STATE
        .lock()
        .map_err(|err| anyhow::anyhow!("could not lock state: {err}"))?;
    let seed = storage.get_seed(encryption_password)?;
    state.seed = Some(seed);
    Ok(())
}

pub fn is_wallet_unlocked() -> bool {
    if let Ok(state) = STATE.lock() {
        return state.seed.is_some();
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::storage::{MemStorage, Storage};

    use super::*;

    #[test]
    fn test_unlock_wallet() -> Result<()> {
        let id = uuid::Uuid::new_v4().to_string();
        let mut storage = MemStorage::new();
        let password = "encryption_password";
        let res = storage.create_seed(password);

        state_unlock_wallet(password, &storage)?;
        assert!(is_wallet_unlocked());
        lock_wallet()?;
        assert!(!is_wallet_unlocked());
        Ok(())
    }
}
