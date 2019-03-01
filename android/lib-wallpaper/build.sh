#~/.bashrc 中配置
# export ANDROID_NDK="$HOME/android-ndk-r19b"
# export ANDROID_TOOLCHAIN="$HOME/ndk-standalone-16-arm"
# export PATH="$PATH:$ANDROID_TOOLCHAIN/bin"

#.cargo/config中配置
# [target.arm-linux-androideabi]
# ar = "~/ndk-standalone-16-arm/bin/arm-linux-androideabi-ar"
# linker = "~/ndk-standalone-16-arm/bin/arm-linux-androideabi-gcc"

#参考 https://github.com/sunsheng/rust-android-https/blob/master/build.sh
#编译好的openssl下载 https://github.com/leenjewel/openssl_for_ios_and_android
export OPENSSL_INCLUDE_DIR=/home/planet/armeabi-v7a/include
export OPENSSL_LIB_DIR=/home/planet/armeabi-v7a/lib

OPENSSL_STATIC=yes cargo build --target armv7-linux-androideabi
cp target/armv7-linux-androideabi/debug/libwallpaper.so ../h8w-android/app/src/main/jniLibs/armeabi-v7a/libwallpaper.so