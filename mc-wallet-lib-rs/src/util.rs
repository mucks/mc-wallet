pub fn mc_wallet_dir() -> anyhow::Result<std::path::PathBuf> {
    let config_dir =
        dirs::config_dir().ok_or_else(|| anyhow::anyhow!("could not get config dir!"))?;
    let mc_wallet_dir = config_dir.join("mc-wallet");
    Ok(mc_wallet_dir)
}
