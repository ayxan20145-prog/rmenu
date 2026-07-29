#!/bin/sh
set -e

NAME="rmenu"

PROJECT_DIR="$(cd "$(dirname "$0")/.." && pwd)"

echo "Building $NAME..."
cd "$PROJECT_DIR"
cargo build --release

echo "Installing $NAME..."
sudo install -Dm755 "$PROJECT_DIR/target/release/$NAME" "/usr/local/bin/$NAME"

echo "Done!"
