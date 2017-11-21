rm -rf /tmp/dk-brew-build
mkdir /tmp/dk-brew-build
cargo build --release
cp ./target/release/dk /tmp/dk-brew-build
cp ./config.json /tmp/dk-brew-build

cd /tmp/dk-brew-build

tar -cvzf dk-0.0.1.tar.gz .
cp dk-0.0.1.tar.gz ~/Code/Tooling/deploy-kite-cli/homebrew/Release