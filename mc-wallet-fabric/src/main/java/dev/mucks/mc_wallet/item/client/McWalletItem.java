package dev.mucks.mc_wallet.item.client;

import dev.mucks.mc_wallet.gui.McWalletGui;
import dev.mucks.mc_wallet.gui.McWalletScreen;
import net.minecraft.client.MinecraftClient;
import net.minecraft.entity.player.PlayerEntity;
import net.minecraft.item.Item;
import net.minecraft.item.ItemStack;
import net.minecraft.util.Hand;
import net.minecraft.util.TypedActionResult;
import net.minecraft.world.World;

public class McWalletItem extends Item {

    public McWalletItem(Settings settings) {
        super(settings);
    }

    @Override
    public TypedActionResult<ItemStack> use(World world, PlayerEntity user, Hand hand) {
        // user.openHandledScreen(new McWalletScreen(new McWalletGui()));
        MinecraftClient.getInstance().setScreen(new McWalletScreen(new McWalletGui()));
        return super.use(world, user, hand);
    }

}
