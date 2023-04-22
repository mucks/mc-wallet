package dev.mucks.mc_wallet.gui;

import dev.mucks.mc_wallet.McWallet;
import net.fabricmc.fabric.api.client.screen.v1.ScreenEvents;
import net.fabricmc.fabric.api.client.screen.v1.Screens;
import net.minecraft.client.MinecraftClient;
import net.minecraft.client.gui.screen.ingame.InventoryScreen;
import net.minecraft.client.gui.widget.ButtonWidget;
import net.minecraft.text.Text;

public class McWalletGui {

    public static void init() {
        ScreenEvents.AFTER_INIT.register((client, screen, scaledWidth, scaledHeight) -> {
            if (screen instanceof InventoryScreen) {
                Screens.getButtons(screen).add(ButtonWidget.builder(Text.literal("wallet"), (btn) -> {
                    onWalletButtonClick(client);
                }).build());
            }
        });
    }

    private static void onWalletButtonClick(MinecraftClient client) {

        if (McWallet.MOD_LIB.isWalletCreated()) {
            client.setScreen(new McWalletScreen(new WalletGui()));
        } else {
            client.setScreen(new McWalletScreen(new CreateWalletGui()));
        }
    }
}
