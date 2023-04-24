package dev.mucks.mc_wallet.gui;

import io.github.cottonmc.cotton.gui.client.LightweightGuiDescription;
import io.github.cottonmc.cotton.gui.widget.WButton;
import io.github.cottonmc.cotton.gui.widget.WGridPanel;
import io.github.cottonmc.cotton.gui.widget.WText;
import net.minecraft.text.Text;
import net.minecraft.util.Formatting;

public class ViewSeedPhraseGui extends LightweightGuiDescription {
    private Text mnemonicText(String mnemonic) {
        String[] mnemonicWords = mnemonic.split(" ");
        String formattedMnemonic = "";

        for (int i = 0; i < mnemonicWords.length; i++) {
            formattedMnemonic += String.format("%d. %s ", i + 1, mnemonicWords[i]);
        }

        Text tmm = Text.of(formattedMnemonic);
        tmm.getStyle().withColor(Formatting.WHITE);
        return tmm;
    }

    public ViewSeedPhraseGui(String mnemonic) {
        WGridPanel root = new WGridPanel();
        root.setSize(300, 200);
        setRootPanel(root);

        WText mnemonicText = new WText(mnemonicText(mnemonic));
        mnemonicText.setSize(10, 10);
        root.add(mnemonicText, 1, 1, 10, 10);

        WButton seedConfirmButton = new WButton(Text.of("I have successfully backed up my seed phrase!"));
        root.add(seedConfirmButton, 1, 6, 10, 10);

        root.validate(this);
    }
}
