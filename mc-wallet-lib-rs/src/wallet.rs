use anyhow::Result;

use crate::{
    state::state_unlock_wallet,
    storage::{FileStorage, MemStorage, Storage},
};

pub fn create_wallet(password: &str) -> Result<String> {
    let mut storage = FileStorage::get_from_file()?;
    let res = storage.create_seed(password)?;
    Ok(res.mnemonic)
}

pub fn test_create_wallet(password: &str) -> Result<String> {
    let mut storage = MemStorage::new();
    let res = storage.create_seed(password)?;
    Ok(res.mnemonic)
}

pub fn is_wallet_created() -> bool {
    let storage = FileStorage::get_from_file();
    if let Ok(storage) = storage {
        return storage.is_created();
    }
    false
}

pub fn unlock_wallet(password: &str) -> Result<()> {
    let storage = FileStorage::get_from_file()?;
    state_unlock_wallet(password, &storage)?;
    Ok(())
}
