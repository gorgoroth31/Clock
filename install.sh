#!/bin/bash
echo "Installing npm packages"
npm install
echo "Building application"
npm run tauri build -- --no-bundle
npm run tauri bundle -- --bundles appimage
mv src-tauri/target/release/bundle/appimage/clock.AppDir/AppRun src-tauri/target/release/bundle/appimage/clock.AppDir/Clock.AppImage

APP_NAME="clock-app"
APPIMAGE="src-tauri/target/release/bundle/appimage/clock.AppDir/Clock.AppImage"

INSTALL_DIR="$HOME/.local/share/applications"

mkdir -p "$INSTALL_DIR"
cp "$APPIMAGE" "$INSTALL_DIR/$APP_NAME"

mkdir -p "$INSTALL_DIR"

sed "s|Exec=.*|Exec=$HOME/.local/share/applications/clock-app|" \
  clock.desktop > "$INSTALL_DIR/clock.desktop"
