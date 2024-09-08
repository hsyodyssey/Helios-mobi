class HeliosRust {
    // This declares that the static `hello` method will be provided
    // a native library.
    private static native String getBalance();



    static {
        // This actually loads the shared object that we'll be creating.
        // The actual location of the .so or .dll may differ based on your
        // platform.
        System.loadLibrary("helios_test_lib");
    }

    // The rest is just regular ol' Java!
    public static void main(String[] args) {
        String output = HeliosRust.getBalance();
        System.out.println("[Java|Kotlin] The balance is :" + output);
    }
}