class Passgen < Formula
  desc "A secure password generator CLI tool for macOS"
  homepage "https://github.com/morshedulmunna/passgen"
  version "0.1.0"
  license "MIT"

  if OS.mac?
    url "https://github.com/morshedulmunna/passgen/releases/download/v#{version}/passgen-x86_64-apple-darwin.tar.gz"
    sha256 "PLACEHOLDER_SHA256" # This will need to be updated after release
  elsif OS.linux?
    url "https://github.com/morshedulmunna/passgen/releases/download/v#{version}/passgen-x86_64-unknown-linux-gnu.tar.gz"
    sha256 "PLACEHOLDER_SHA256" # This will need to be updated after release
  end

  def install
    bin.install "passgen"
  end

  test do
    system "#{bin}/passgen", "--version"
  end
end 