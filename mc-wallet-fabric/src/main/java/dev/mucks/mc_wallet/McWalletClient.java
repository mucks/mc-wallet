package dev.mucks.mc_wallet;

import dev.mucks.mc_wallet.gui.McWalletGui;
import net.fabricmc.api.ClientModInitializer;

public class McWalletClient implements ClientModInitializer {
    @Override
    public void onInitializeClient() {
        McWallet.LOGGER.info("Initializing Client for " + McWallet.MOD_ID);

        McWalletGui.init();
    }

}
