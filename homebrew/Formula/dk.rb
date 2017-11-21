class Dk < Formula
  desc ""
  homepage "https://github.com/aa-neg/deploy-kite-cli"
  url "https://github.com/aa-neg/deploy-kite-cli/blob/master/homebrew/Release/dk-0.0.2.tar.gz?raw=true"
  sha256 "97e4d2c7749b64422961a254a22eb44d86ebcd62048eb1cd9e3aae0e0868fe52"

  def install
    bin.install "dk"
    etc.install "config.json"
  end
end
