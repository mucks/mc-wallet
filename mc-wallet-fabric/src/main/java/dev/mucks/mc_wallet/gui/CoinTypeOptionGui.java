package dev.mucks.mc_wallet.gui;

import io.github.cottonmc.cotton.gui.widget.WLabel;
import io.github.cottonmc.cotton.gui.widget.WPlainPanel;
import net.minecraft.text.Text;

public class CoinTypeOptionGui extends WPlainPanel {
    WLabel label;
    WLabel cost;

    public CoinTypeOptionGui() {
        label = new WLabel(Text.literal("Foo"));
        this.add(label, 18 + 4, 2, 18, 18);
        cost = new WLabel(Text.literal("1000 Xp"));
        this.add(cost, 2, 20, 2 * 18, 18);

        this.setSize(4 * 18, 2 * 18);
    }
}
