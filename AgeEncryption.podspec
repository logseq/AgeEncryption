Pod::Spec.new do |s|
    s.name         = "AgeEncryption"
    s.version      = "1.0.6"
    s.summary      = "Age-encryption sys library"
    s.description  = <<-DESC
    Age-encryption, in Rust, for Swift FFI.
    DESC
    s.homepage     = "https://github.com/andelf/AgeEncryption"
    s.license = { :type => 'Copyright', :text => <<-LICENSE
                  Copyright 2022, Andelf
                  https://opensource.org/licenses/MIT
                  LICENSE
                }
    s.author       = { "Andelf" => "andelf@gmail.com" }
    s.source       = { :http => "https://github.com/andelf/AgeEncryption/releases/download/#{s.version}/AgeEncryption.xcframework.zip" }

    s.vendored_frameworks = "AgeEncryption.xcframework"
    s.requires_arc = true
    s.static_framework    = true

    s.platform              = :ios
    s.swift_version         = "5.1"
    s.ios.deployment_target = '12.0'
end
