set -e

# Makes a release zip of the game for distribution
rm web-release.zip || true
rm -rf web-release
mkdir -p web-release

# Copy necessary files to web-release directory
mkdir -p web-release/Assets
cp -r assets/* web-release/Assets
mkdir -p web-release/Data
cp -r Data/* web-release/Data

# Build the WASM target
cargo build --release --target wasm32-unknown-unknown

# Copy necessary files to web-release directory
cp target/wasm32-unknown-unknown/release/light_evader.wasm web-release
cp web/*js web-release
cp web/index.html web-release

# Copy assets
mkdir -p web-release/assets
cp -r assets/* web-release/assets

# Remove unwanted system files
rm -rf web-release/**/.DS_Store
rm -rf web-release/**/*/.DS_Store

# Zip for release
zip -r web-release.zip web-release
echo "Web release zipped into web-release.zip"
