use bip32::{Seed, XPrv};

pub fn mc_wallet_dir() -> anyhow::Result<std::path::PathBuf> {
    let config_dir =
        dirs::config_dir().ok_or_else(|| anyhow::anyhow!("could not get config dir!"))?;
    let mc_wallet_dir = config_dir.join("mc-wallet");
    Ok(mc_wallet_dir)
}

pub fn gen_private_key(child_path: &str, seed: &Seed) -> anyhow::Result<XPrv> {
    let child_xprv = XPrv::derive_from_path(seed, &child_path.parse()?)?;
    Ok(child_xprv)
}

pub fn clone_seed(seed: &Seed) -> Seed {
    let seed_bytes = seed.as_bytes();
    Seed::new(*seed_bytes)
}
