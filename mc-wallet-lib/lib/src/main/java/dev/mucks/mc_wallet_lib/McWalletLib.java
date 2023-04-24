/*
 * This Java source file was generated by the Gradle 'init' task.
 */
package dev.mucks.mc_wallet_lib;

import java.nio.file.Path;
import java.nio.file.Paths;

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

    public native int createConfigDir();

    public native String createMnemonic();

    public native void createAndSaveSeed(String mnemonic, String seedPassword, String encryptionPassword);

    public native String getSeedHex(String encryptionPassword);

    public native String createWallet(String walletPassword);

    public native boolean isWalletCreated();

    public native String unlockWallet(String walletPassword);

    public native boolean isWalletUnlocked();

}
