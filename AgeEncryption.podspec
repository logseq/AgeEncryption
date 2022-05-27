Pod::Spec.new do |s|
    s.name         = "AgeEncryption"
    s.version      = "1.0.4"
    s.summary      = "Age-encryption sys library"
    s.description  = <<-DESC
    Age-encryption, in Rust, for Swift.
    DESC
    s.homepage     = "https://github.com/andelf/AgeEncryption"
    s.license = { :type => 'Copyright', :text => <<-LICENSE
                  Copyright 2022, Andelf
                  https://opensource.org/licenses/MIT
                  LICENSE
                }
    s.author       = { "Andelf" => "andelf@gmail.com" }
    s.source       = { :git => "https://github.com/andelf/AgeEncryption.git", :tag => "#{s.version}" }
    s.vendored_frameworks = "AgeEncryption.xcframework"
    s.platform = :ios
    s.swift_version = "5.0"
    s.ios.deployment_target  = '12.0'
end
