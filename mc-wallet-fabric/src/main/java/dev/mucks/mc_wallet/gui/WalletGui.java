package dev.mucks.mc_wallet.gui;

import io.github.cottonmc.cotton.gui.client.LightweightGuiDescription;
import io.github.cottonmc.cotton.gui.widget.WGridPanel;
import io.github.cottonmc.cotton.gui.widget.WLabel;
import net.minecraft.text.Text;

public class WalletGui extends LightweightGuiDescription {
    public WalletGui() {
        WGridPanel root = new WGridPanel();
        root.setSize(300, 200);
        setRootPanel(root);

        WLabel label = new WLabel(Text.of("MC Wallet"));
        root.add(label, 1, 1);

    }
}
