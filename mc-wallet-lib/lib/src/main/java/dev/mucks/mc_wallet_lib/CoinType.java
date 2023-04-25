package dev.mucks.mc_wallet_lib;

public enum CoinType {
    ETH, SUI, SOL;

    public int asInteger() {
        switch (this) {
            case ETH:
                return 66;
            case SUI:
                return 501;
            case SOL:
                return 784;
            default:
                throw new IllegalArgumentException("Unknown coin type");
        }
    }
}
