package dev.mucks.mc_wallet.gui;

import dev.mucks.mc_wallet.McWallet;
import io.github.cottonmc.cotton.gui.client.LightweightGuiDescription;
import io.github.cottonmc.cotton.gui.widget.WButton;
import io.github.cottonmc.cotton.gui.widget.WGridPanel;
import io.github.cottonmc.cotton.gui.widget.WLabel;
import io.github.cottonmc.cotton.gui.widget.WTextField;
import net.minecraft.client.MinecraftClient;
import net.minecraft.text.Text;

public class UnlockWalletGui extends LightweightGuiDescription {
    public UnlockWalletGui() {
        WGridPanel root = new WGridPanel();
        root.setSize(300, 200);
        setRootPanel(root);

        WLabel label = new WLabel(Text.of("Login MC Wallet"));
        root.add(label, 1, 1);

        WTextField password = new WTextField();
        root.add(password, 1, 2, 10, 10);

        WButton unlockBtn = new WButton(Text.of("Unlock Wallet"));
        root.add(unlockBtn, 1, 3, 10, 10);

        unlockBtn.setOnClick(() -> {
            try {
                McWallet.MOD_LIB.unlockWallet(password.getText());
                MinecraftClient.getInstance().setScreen(new McWalletScreen(new WalletGui()));
            } catch (Exception e) {
                System.out.println(e);
                return;
            }
        });

        root.validate(this);

    }

}
