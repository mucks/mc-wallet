package dev.mucks.mc_wallet.gui;

import java.util.ArrayList;
import java.util.function.BiConsumer;

import io.github.cottonmc.cotton.gui.client.LightweightGuiDescription;
import io.github.cottonmc.cotton.gui.widget.WButton;
import io.github.cottonmc.cotton.gui.widget.WGridPanel;
import io.github.cottonmc.cotton.gui.widget.WListPanel;
import net.minecraft.text.Text;

public class CreateAccountGui extends LightweightGuiDescription {
    public CreateAccountGui() {
        WGridPanel root = new WGridPanel();
        root.setSize(300, 200);
        setRootPanel(root);

        ArrayList<CoinType> data = new ArrayList<>();
        data.add(new CoinType("Ethereum", "ETH"));
        data.add(new CoinType("Solana", "SOL"));
        data.add(new CoinType("SUI", "Sui"));

        BiConsumer<CoinType, CoinTypeOptionGui> configurator = (CoinType coin, CoinTypeOptionGui coinType) -> {
            coinType.name.setText(Text.literal(coin.name));
            coinType.symbol.setText(Text.literal(coin.symbol));
        };

        WListPanel<CoinType, CoinTypeOptionGui> list = new WListPanel<>(data, CoinTypeOptionGui::new, configurator);
        list.setListItemHeight(2 * 18);
        root.add(list, 1, 4, 7, 6);

        WButton createAccountBtn = new WButton(Text.of("Create Account"));
        root.add(createAccountBtn, 1, 80, 10, 10);

        createAccountBtn.setOnClick(() -> {

        });

        root.validate(this);
    }

}
