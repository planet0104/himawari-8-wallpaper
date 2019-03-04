set PATH=%PATH%;F:\ndk-standalone-16-arm\bin
cargo build --target armv7-linux-androideabi
copy target\armv7-linux-androideabi\debug\libwallpaper.so ..\h8w-android\app\src\main\jniLibs\armeabi-v7a\libwallpaper.so