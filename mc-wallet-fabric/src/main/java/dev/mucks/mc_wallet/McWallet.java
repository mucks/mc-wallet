package dev.mucks.mc_wallet;

import net.fabricmc.api.ModInitializer;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import dev.mucks.mc_wallet.item.ModItemGroup;
import dev.mucks.mc_wallet.item.ModItems;
import dev.mucks.mc_wallet_lib.McWalletLib;

public class McWallet implements ModInitializer {
	// This logger is used to write text to the console and the log file.
	// It is considered best practice to use your mod id as the logger's name.
	// That way, it's clear which mod wrote info, warnings, and errors.
	public static final Logger LOGGER = LoggerFactory.getLogger("mc-wallet");
	public static final String MOD_ID = "mc-wallet";
	public static final McWalletLib MOD_LIB = new McWalletLib();

	@Override
	public void onInitialize() {
		LOGGER.info("Initializing " + MOD_ID);

		ModItemGroup.registerItemGroups();
		ModItems.registerModItems();

	}
}