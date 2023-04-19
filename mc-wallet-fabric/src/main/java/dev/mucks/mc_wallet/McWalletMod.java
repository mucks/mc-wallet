package dev.mucks.mc_wallet;

import net.fabricmc.api.ModInitializer;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import dev.mucks.mc_wallet_lib.McWalletLib;

public class McWalletMod implements ModInitializer {
	// This logger is used to write text to the console and the log file.
	// It is considered best practice to use your mod id as the logger's name.
	// That way, it's clear which mod wrote info, warnings, and errors.
	public static final Logger LOGGER = LoggerFactory.getLogger("mc-wallet");

	@Override
	public void onInitialize() {
		McWalletLib lib = new McWalletLib();
		String mnemonic = lib.createMnemonicRust();

		LOGGER.info("Mnemonic: " + mnemonic);

	}
}