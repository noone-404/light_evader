set -e

rm -fr pc-build-release.zip || true
rm -fr pc-build-release || true

mkdir pc-build-release
mkdir -p pc-build-release/Assets
cp -r assets/* pc-build-release/Assets
mkdir -p pc-build-release/Data
cp -r Data/* pc-build-release/Data

cargo build --release --target x86_64-pc-windows-msvc

cp target/x86_64-pc-windows-msvc/release/light_evader.exe pc-build-release

zip -r pc-build-release.zip pc-build-release
echo "PC build zipped into pc-build-release.zip"
