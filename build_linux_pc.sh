set -e

rm -fr linux-build-release.zip || true
rm -fr linux-build-release || true

mkdir linux-build-release
mkdir -p linux-build-release/Assets
cp -r assets/* linux-build-release/Assets
mkdir -p linux-build-release/Data
cp -r Data/* linux-build-release/Data

cargo build --release --target x86_64-unknown-linux-gnu

cp target/x86_64-unknown-linux-gnu/release/light_evader linux-build-release

zip -r linux-build-release.zip linux-build-release
echo "PC build zipped into linux-build-release.zip"
