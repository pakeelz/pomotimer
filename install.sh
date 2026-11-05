#!/bin/bash
set -e

REPO="pakeelz/pomotimer"
INSTALL_DIR="/usr/local/bin"

# Make sure user is linux
OS="$(uname -s)"
if [ "$OS" != "Linux" ]; then
  echo "Error: Just for linux operating system."
  exit 1
fi

# Take URL newest releases
echo "==> Find latest releases..."
DOWNLOAD_URL=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" |
  grep "browser_download_url.*pomotimer-linux-x86_64.tar.gz" |
  cut -d '"' -f 4)

if [ -z "$DOWNLOAD_URL" ]; then
  echo "Error: Failed to find release files."
  exit 1
fi

# Download and tar to temp dir
TMP_DIR=$(mktemp -d)
echo "==> Mengunduh binary shlt..."
curl -sL "$DOWNLOAD_URL" | tar -xz -C "$TMP_DIR"

# Move binary to /usr/local/bin/
echo "==> Installing pomotimer to $INSTALL_DIR..."
sudo mv "$TMP_DIR/pomotimer" "$INSTALL_DIR/pomotimer"
sudo chmod +x "$INSTALL_DIR/pomotimer"

# Cleanup
rm -rf "$TMP_DIR"

# Create share dir and tmp dir
TMP_DIR=$(mktemp -d)
SHARE_DIR="$HOME/.local/share/pomotimer/icon/"

mkdir -p $SHARE_DIR
curl -o "$TMP_DIR/peach.svg" https://raw.githubusercontent.com/pakeelz/shlt/main/assets/peach.svg

mv "$TMP_DIR/peach/svg" "$SHARE_DIR/peach/svg"

# Cleanup
rm -rf "$TMP_DIR"

# Create .desktop file
applicationsDir="$HOME/.local/share/applications/"
mkdir -p "$applicationsDir"

pomotimerApp="$applicationsDir/pomotimer.desktop"

cat <<EOF >$pomotimerApp
[Desktop Entry]
Type=Application
Name=Pomotimer
Comment=Pomodoro Timer On Terminal
Exec=/usr/local/bin/pomotimer
Icon=$HOME/.local/share/pomotimer/icon/peach.svg
Terminal=true
Categories=Utility;
EOF

update-desktop-database "$HOME/.local/share/applications/"

echo "==> Installation Done. Give starts on https://github.com/pakeelz/pomotimer if you like this apps"
