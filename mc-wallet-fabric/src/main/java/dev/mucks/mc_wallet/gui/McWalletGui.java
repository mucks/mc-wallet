package dev.mucks.mc_wallet.gui;

import io.github.cottonmc.cotton.gui.client.LightweightGuiDescription;
import io.github.cottonmc.cotton.gui.widget.WGridPanel;
import io.github.cottonmc.cotton.gui.widget.WLabel;
import net.minecraft.text.Text;

public class McWalletGui extends LightweightGuiDescription {
    public McWalletGui() {
        WGridPanel root = new WGridPanel();
        setRootPanel(root);

        root.setSize(300, 200);

        WLabel label = new WLabel(Text.of("Hello World!"));
        root.add(label, 1, 1);
    }

}
