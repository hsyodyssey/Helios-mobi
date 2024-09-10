## How to intergrate

### Step 1: Configure Cargo build env
Before you complie the rust library, please create the config file with the following code in `~/.cargo/config` file.
```
[target.aarch64-linux-android]
linker = "/home/siyuan/tool/android-ndk/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android34-clang"

[target.x86_64-linux-android]
linker = "/home/siyuan/tool/android-ndk/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android34-clang"
```

### Step 2: Configure rustup target
`rustup target add aarch64-linux-android x86_64-linux-android`


### Step 3: Build so file
`cargo build --release --target aarch64-linux-android`
`cargo build --release --target x86_64-linux-android`

## Tips
### The function name