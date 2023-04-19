package dev.mucks.mc_wallet.item;

import dev.mucks.mc_wallet.McWallet;
import net.fabricmc.fabric.api.itemgroup.v1.FabricItemGroup;
import net.minecraft.item.ItemGroup;
import net.minecraft.text.Text;
import net.minecraft.util.Identifier;

public class ModItemGroup {
    public static ItemGroup MC_WALLET;

    public static void registerItemGroups() {
        MC_WALLET = FabricItemGroup.builder(new Identifier(McWallet.MOD_ID, "mc_wallet"))
                .displayName(Text.translatable("itemgroup.mc_wallet")).build();
    }

}
