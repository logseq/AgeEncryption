#!/bin/bash

set -ex


rm -rvf AgeEncryption.xcframework

cargo build --release --target aarch64-apple-ios

xcodebuild -create-xcframework \
  -library ./target/aarch64-apple-ios/release/libage.a \
  -headers ./include/ \
  -output AgeEncryption.xcframework


# FIXME: seems Cocoapods cannot handle this.
#  -library ./libage_macos.a \
#  -headers ./include/ \
#  -library ./libage_iossimulator.a \
#  -headers ./include/ \
#  -library ./libage_maccatalyst.a \
#  -headers ./include/ \
