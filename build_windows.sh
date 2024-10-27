set -e

rm -fr windows-build-release.zip || true
rm -fr windows-build-release || true

mkdir windows-build-release
mkdir -p windows-build-release/Assets
cp -r assets/* windows-build-release/Assets
mkdir -p windows-build-release/Data
cp -r Data/* windows-build-release/Data

cargo build --release --target x86_64-pc-windows-gnu

cp target/x86_64-pc-windows-gnu/release/light_evader.exe windows-build-release

zip -r windows-build-release.zip windows-build-release
echo "Windows build zipped into windows-build-release.zip"
