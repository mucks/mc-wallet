/*
 * This Java source file was generated by the Gradle 'init' task.
 */
package dev.mucks.mc_wallet_lib;

import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;

public class McWalletLib {

    static {
        System.load(getRustLibraryPath().toString());
    }

    private static Path getRustLibraryPath() {
        Path localDirPath = Paths.get(System.getProperty("user.dir"));
        String libRelativePath = "../../mc-wallet-lib-rs/target/release/libmc_wallet_lib_rs.so";
        Path libPath = localDirPath.resolve(libRelativePath);
        return libPath;
    }

    public static void createAccount(CoinType coinType) {
        createAccountRust(coinType.asInteger());
    }

    public static String createWallet(String password) {
        return createWalletRust(password, false);
    }

    public static boolean isWalletCreated() {
        return isWalletCreatedRust();
    }

    public static String unlockWallet(String password) {
        return unlockWalletRust(password);
    }

    public static boolean isWalletUnlocked() {
        return isWalletUnlockedRust();
    }

    static native ArrayList<Account> getAccountsRust(boolean test);

    static native void createAccountRust(int coinType);

    static native String createWalletRust(String password, boolean test);

    static native boolean isWalletCreatedRust();

    static native String unlockWalletRust(String password);

    static native boolean isWalletUnlockedRust();

}
