package dev.mucks.mc_wallet.gui;

import dev.mucks.mc_wallet.McWallet;
import io.github.cottonmc.cotton.gui.client.LightweightGuiDescription;
import io.github.cottonmc.cotton.gui.widget.WButton;
import io.github.cottonmc.cotton.gui.widget.WGridPanel;
import io.github.cottonmc.cotton.gui.widget.WLabel;
import io.github.cottonmc.cotton.gui.widget.WTextField;
import net.minecraft.text.Text;
import net.minecraft.client.MinecraftClient;

public class CreateWalletGui extends LightweightGuiDescription {
    public CreateWalletGui() {
        WGridPanel root = new WGridPanel();
        root.setSize(300, 200);
        setRootPanel(root);

        WLabel label = new WLabel(Text.of("Create MC Wallet"));
        root.add(label, 1, 1);

        WTextField password = new WTextField();
        root.add(password, 1, 2, 10, 10);

        WTextField repeatPassword = new WTextField();
        root.add(repeatPassword, 1, 3, 10, 10);

        WButton createWalletBtn = new WButton(Text.of("Create Wallet"));
        createWalletBtn.setEnabled(false);
        root.add(createWalletBtn, 1, 5, 10, 10);

        password.setChangedListener((str) -> {
            this.onPasswordTextChange(password, repeatPassword, createWalletBtn);
        });

        repeatPassword.setChangedListener((str) -> {
            this.onPasswordTextChange(password, repeatPassword, createWalletBtn);
        });

        createWalletBtn.setOnClick(() -> {
            String mnemonic = McWallet.MOD_LIB.createWallet(password.getText());
            MinecraftClient.getInstance().setScreen(new McWalletScreen(new ViewSeedPhraseGui(mnemonic)));
        });

        root.validate(this);

    }

    private void onPasswordTextChange(WTextField password, WTextField repeatPassword, WButton createWalletBtn) {
        if (!password.getText().isEmpty() && password.getText().equals(repeatPassword.getText())) {
            createWalletBtn.setEnabled(true);
        } else {
            createWalletBtn.setEnabled(false);
        }
    }
}
