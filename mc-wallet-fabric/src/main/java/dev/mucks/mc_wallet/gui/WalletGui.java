package dev.mucks.mc_wallet.gui;

import java.util.ArrayList;
import java.util.function.BiConsumer;

import io.github.cottonmc.cotton.gui.client.LightweightGuiDescription;
import io.github.cottonmc.cotton.gui.widget.WButton;
import io.github.cottonmc.cotton.gui.widget.WGridPanel;
import io.github.cottonmc.cotton.gui.widget.WLabel;
import io.github.cottonmc.cotton.gui.widget.WListPanel;
import net.minecraft.client.MinecraftClient;
import net.minecraft.text.Text;

public class WalletGui extends LightweightGuiDescription {
    public WalletGui() {
        WGridPanel root = new WGridPanel();
        root.setSize(300, 200);
        setRootPanel(root);

        WLabel label = new WLabel(Text.of("MC Wallet"));
        root.add(label, 1, 1);

        WLabel accountsLabel = new WLabel(Text.of("Accounts"));
        root.add(accountsLabel, 1, 2);

        WButton createAccountBtn = new WButton(Text.of("Create Account"));
        root.add(createAccountBtn, 1, 3, 10, 10);

        createAccountBtn.setOnClick(() -> {
            MinecraftClient.getInstance().setScreen(new McWalletScreen(new CreateAccountGui()));
        });

        root.validate(this);
    }
}
