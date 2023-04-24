use std::sync::Mutex;

use anyhow::Result;
use bip32::{Seed, XPrv};
use lazy_static::lazy_static;

use crate::{get_seed, util::clone_seed};

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

pub fn unlock_wallet(encryption_password: &str) -> Result<()> {
    let mut state = STATE
        .lock()
        .map_err(|err| anyhow::anyhow!("could not lock state: {err}"))?;
    let seed = get_seed(encryption_password)?;
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
    use super::*;
    use crate::{create_and_save_seed, create_mnemonic};

    #[test]
    fn test_unlock_wallet() {
        let mnemonic = create_mnemonic();
        create_and_save_seed(&mnemonic, "password", "encryption_password").unwrap();
        unlock_wallet("encryption_password").unwrap();
        assert!(is_wallet_unlocked());
        lock_wallet().unwrap();
        assert!(!is_wallet_unlocked());
    }
}
