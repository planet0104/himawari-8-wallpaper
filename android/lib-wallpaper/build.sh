#cargo:rerun-if-env-changed=ARMV7_LINUX_ANDROIDEABI_OPENSSL_LIB_DIR
#cargo:rerun-if-env-changed=OPENSSL_LIB_DIR
#cargo:rerun-if-env-changed=ARMV7_LINUX_ANDROIDEABI_OPENSSL_INCLUDE_DIR
#cargo:rerun-if-env-changed=OPENSSL_INCLUDE_DIR
#cargo:rerun-if-env-changed=ARMV7_LINUX_ANDROIDEABI_OPENSSL_DIR
#cargo:rerun-if-env-changed=OPENSSL_DIR

# export OPENSSL_DIR="/usr/local/openssl"
export OPENSSL_DIR="/openssl_android/lib/armeabi-v7a"
export RUST_BACKTRACE=1
export OPENSSL_STATIC="TRUE"
export OPENSSL_INCLUDE_DIR="~/openssl_android/lib/armeabi-v7a/include"

cargo build --target armv7-linux-androideabi
cp target/armv7-linux-androideabi/debug/libwallpaper.so ../h8w-android/app/src/main/jniLibs/armeabi-v7a/libwallpaper.so