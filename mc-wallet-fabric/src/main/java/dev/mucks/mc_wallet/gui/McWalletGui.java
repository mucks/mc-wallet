package dev.mucks.mc_wallet.gui;

import dev.mucks.mc_wallet.McWallet;
import dev.mucks.mc_wallet_lib.McWalletLib;
import io.github.cottonmc.cotton.gui.client.LightweightGuiDescription;
import io.github.cottonmc.cotton.gui.widget.WButton;
import io.github.cottonmc.cotton.gui.widget.WGridPanel;
import io.github.cottonmc.cotton.gui.widget.WLabel;
import io.github.cottonmc.cotton.gui.widget.WText;
import net.minecraft.text.Style;
import net.minecraft.text.Text;
import net.minecraft.util.Formatting;

public class McWalletGui extends LightweightGuiDescription {

    public McWalletGui() {
        WGridPanel root = new WGridPanel();
        setRootPanel(root);

        root.setSize(300, 200);

        WLabel label = new WLabel(Text.of("MC Wallet"));
        root.add(label, 1, 1);

        WButton button = new WButton(Text.of("Create Seed Phrase"));
        root.add(button, 1, 2, 10, 10);

        WText mnemonicText = new WText(Text.of(""));
        mnemonicText.setSize(10, 10);
        root.add(mnemonicText, 1, 4, 10, 10);

        button.setOnClick(() -> {
            String mnemonic = McWallet.MOD_LIB.createMnemonicRust();

            Text tmm = Text.of(mnemonic);
            tmm.getStyle().withColor(Formatting.WHITE);

            mnemonicText.setText(tmm);
        });

        root.validate(this);
    }

}
