#!/bin/bash

set -ex

rm -rvf AgeEncryption.xcframework AgeEncryption.xcframework.zip

# iOS
cargo build --release --target aarch64-apple-ios

# simulator
cargo build --release --target x86_64-apple-ios
cargo build --release --target aarch64-apple-ios-sim

lipo -create ./target/x86_64-apple-ios/release/libage.a \
    ./target/aarch64-apple-ios-sim/release/libage.a \
    -output libage.a

#lipo -create \
#  ./target/x86_64-apple-ios-macabi/release/libage.a \
#  ./target/aarch64-apple-ios-macabi/release/libage.a \
#  -output libage_maccatalyst.a

xcodebuild -create-xcframework \
  -library ./target/aarch64-apple-ios/release/libage.a \
  -headers ./include/ \
  -library ./libage.a \
  -headers ./include/ \
  -output AgeEncryption.xcframework


zip -r AgeEncryption.xcframework.zip AgeEncryption.xcframework


# FIXME: seems Cocoapods cannot handle this.
#  -library ./libage_macos.a \
#  -headers ./include/ \
#  -library ./libage_iossimulator.a \
#  -headers ./include/ \
#  -library ./libage_maccatalyst.a \
#  -headers ./include/ \
