class Dk < Formula
  desc ""
  homepage "https://github.com/aa-neg/deploy-kite-cli"
  url "https://github.com/aa-neg/deploy-kite-cli/blob/master/homebrew/Release/dk-0.0.2.tar.gz?raw=true"
  sha256 "61cc8ce6613591899a91536ad466c3e2ff9265d8164eafba03562deb21336511"

  def install
    bin.install "dk"
    etc.install "config.json"
  end
end
