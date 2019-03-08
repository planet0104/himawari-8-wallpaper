export ANDROID_NDK="$HOME/android-ndk-r19b"
export ANDROID_TOOLCHAIN="$HOME/ndk-standalone-16-x86"
export PATH="$PATH:$ANDROID_TOOLCHAIN/bin"
cargo build --target i686-linux-android --release
# cp target/i686-linux-android/release/libwallpaper.so ../h8w-android/app/src/main/jniLibs/x86/libwallpaper.so