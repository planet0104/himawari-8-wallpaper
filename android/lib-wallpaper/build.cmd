set ANDROID_NDK="F:\android-ndk-r18b"
set ANDROID_TOOLCHAIN=F:\ndk-standalone-16-arm\bin
::设置要临时加入到path环境变量中的路径
set PATH=%PATH%;%ANDROID_TOOLCHAIN%
cargo build --target arm-linux-androideabi
copy target\arm-linux-androideabi\debug\libwallpaper.so ..\h8w-android\app\src\main\jniLibs\armeabi\libwallpaper.so