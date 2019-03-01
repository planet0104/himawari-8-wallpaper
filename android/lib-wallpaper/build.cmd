set OPENSSL_INCLUDE_DIR=C:\Users\days888\Documents\GitHub\himawari-8-wallpaper\android\lib-wallpaper\lib_openssl_armeabi-v7a\include
set OPENSSL_LIB_DIR=C:\Users\days888\Documents\GitHub\himawari-8-wallpaper\android\lib-wallpaper\lib_openssl_armeabi-v7a\lib
set OPENSSL_STATIC=yes
cargo build --target armv7-linux-androideabi
copy target\armv7-linux-androideabi\debug\libwallpaper.so ..\h8w-android\app\src\main\jniLibs\armeabi-v7a\libwallpaper.so