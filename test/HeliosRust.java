class HeliosRust {
    // This declares that the static `hello` method will be provided
    // a native library.
    private static native String getBalance();
    private static native String getLatestBlockNumber();
    private static native String getLatestCheckpoint();

    static {
        // This actually loads the shared object that we'll be creating.
        // The actual location of the .so or .dll may differ based on your
        // platform.
        System.loadLibrary("helios_test_lib");
    }

    // The rest is just regular ol' Java!
    public static void main(String[] args) {
        // String output = HeliosRust.getBalance();
        // System.out.println("[Java|Kotlin] The balance is :" + output);

        // String getBlockByNumber = HeliosRust.getLatestBlockNumber();
        // System.out.println("[Java|Kotlin] The latest block number is :" + getBlockByNumber);
    
        String latestCheckpoint = HeliosRust.getLatestCheckpoint();
        System.out.println("[Java|Kotlin] The latest Checkpoint is :" + latestCheckpoint);
    }
}