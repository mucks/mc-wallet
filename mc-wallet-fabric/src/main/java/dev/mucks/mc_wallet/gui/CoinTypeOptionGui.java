package dev.mucks.mc_wallet.gui;

import io.github.cottonmc.cotton.gui.widget.WLabel;
import io.github.cottonmc.cotton.gui.widget.WPlainPanel;
import io.github.cottonmc.cotton.gui.widget.WToggleButton;
import net.minecraft.text.Text;

public class CoinTypeOptionGui extends WPlainPanel {
    WLabel name;
    WLabel symbol;
    WToggleButton toggle;

    public CoinTypeOptionGui() {
        toggle = new WToggleButton(Text.literal(""));
        this.add(toggle, 0, 2, 18, 18);
        name = new WLabel(Text.literal("Ethereum"));
        this.add(name, 40, 2, 60, 18);
        symbol = new WLabel(Text.literal("ETH"));
        this.add(symbol, 100, 2, 20, 18);

        this.setSize(200, 18);
    }
}
