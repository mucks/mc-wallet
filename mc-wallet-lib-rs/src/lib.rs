use anyhow::{anyhow, Result};
use bip32::Mnemonic;
use rand_core::OsRng;

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
}
