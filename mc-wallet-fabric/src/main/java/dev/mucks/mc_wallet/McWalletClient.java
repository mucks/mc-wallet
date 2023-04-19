package dev.mucks.mc_wallet;

import dev.mucks.mc_wallet.gui.McWalletGui;
import dev.mucks.mc_wallet.gui.McWalletScreen;
import net.fabricmc.api.ClientModInitializer;
import net.fabricmc.fabric.api.client.screen.v1.ScreenEvents;
import net.fabricmc.fabric.api.client.screen.v1.Screens;
import net.minecraft.client.gui.screen.ingame.InventoryScreen;
import net.minecraft.client.gui.widget.ButtonWidget;
import net.minecraft.text.Text;

public class McWalletClient implements ClientModInitializer {
    @Override
    public void onInitializeClient() {
        McWallet.LOGGER.info("Initializing Client for " + McWallet.MOD_ID);

        ScreenEvents.AFTER_INIT.register((client, screen, scaledWidth, scaledHeight) -> {
            if (screen instanceof InventoryScreen) {
                Screens.getButtons(screen).add(ButtonWidget.builder(Text.literal("wallet"), (btn) -> {
                    client.setScreen(new McWalletScreen(new McWalletGui()));
                }).build());
            }

        });
    }

}
