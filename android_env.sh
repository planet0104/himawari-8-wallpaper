#!/bin/bash
_ANDROID_NDK="android-ndk-r19b"
_ANDROID_EABI="arm-linux-androideabi-4.9"
_ANDROID_ARCH=arch-arm
_ANDROID_API="android-18"

export ANDROID_NDK_ROOT=/home/planet/android-ndk-r19b

########Check:

########$ANDROID_NDK_ROOT/toolchains/$_ANDROID_EABI
########/home/planet/android-ndk-r19b/toolchains/arm-linux-androideabi-4.9

########~/android-ndk-r19b/build$ cd tools/
########./make-standalone-toolchain.sh --platform=18 --arch=arm --install-dir=~/ndk-standalone-18-arm --verbose

export ANDROID_TOOLCHAIN=/home/planet/ndk-standalone-18-arm/bin

#######export PATH="$ANDROID_TOOLCHAIN":"$PATH"

export ANDROID_SYSROOT="$ANDROID_NDK_ROOT/platforms/$_ANDROID_API/$_ANDROID_ARCH"
export CROSS_SYSROOT="$ANDROID_SYSROOT"
export NDK_SYSROOT="$ANDROID_SYSROOT"

export MACHINE=armv7
export RELEASE=2.6.37
export SYSTEM=android
export ARCH=arm
export CROSS_COMPILE="arm-linux-androideabi-"

export ANDROID_NDK_SYSROOT="$ANDROID_SYSROOT"
export ANDROID_API="$_ANDROID_API"

export ANDROID_DEV="$ANDROID_NDK_ROOT/platforms/$_ANDROID_API/$_ANDROID_ARCH/usr"
export HOSTCC=gcc

export PATH="$ANDROID_TOOLCHAIN":"$PATH"

#End.
echo "ANDROID_NDK_ROOT: $ANDROID_NDK_ROOT"
echo "ANDROID_ARCH: $_ANDROID_ARCH"
echo "ANDROID_EABI: $_ANDROID_EABI"
echo "ANDROID_API: $ANDROID_API"
echo "ANDROID_SYSROOT: $ANDROID_SYSROOT"
echo "ANDROID_TOOLCHAIN: $ANDROID_TOOLCHAIN"
echo "FIPS_SIG: $FIPS_SIG"
echo "CROSS_COMPILE: $CROSS_COMPILE"
echo "ANDROID_DEV: $ANDROID_DEV"
