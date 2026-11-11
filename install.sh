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
DOWNLOAD_URL="https://github.com/$REPO/releases/latest/download/pomotimer-linux-x86_64.tar.gz"

# Download and tar to temp dir
TMP_DIR=$(mktemp -d)
echo "==> Downloading binary..."
curl -sL "$DOWNLOAD_URL" | tar -xz -C "$TMP_DIR"

# Move binary to /usr/local/bin/
echo "==> Installing pomotimer to $INSTALL_DIR..."
sudo mv "$TMP_DIR/pomotimer" "$INSTALL_DIR/pomotimer"
sudo chmod +x "$INSTALL_DIR/pomotimer"

# Cleanup
rm -rf "$TMP_DIR"

# Create share dir and tmp dir
SHARE_DIR="$HOME/.local/share/pomotimer/icon"
SHARE_DIR_SOUNDS="$HOME/.local/share/pomotimer/sounds"
mkdir -p "$SHARE_DIR"
mkdir -p "$SHARE_DIR_SOUNDS"

TMP_DIR=$(mktemp -d)
curl -sLo "$TMP_DIR/timer.png" https://raw.githubusercontent.com/pakeelz/pomotimer/main/assets/timer.png
mv "$TMP_DIR/timer.png" "$SHARE_DIR/timer.png"

curl -sLo "$TMP_DIR/breaks.png" https://raw.githubusercontent.com/pakeelz/pomotimer/main/assets/breaks.png
mv "TMP_DIR/breaks.png" "$SHARE_DIR/breaks.png"

curl -sLo "$TMP_DIR/bell.mp3" https://raw.githubusercontent.com/pakeelz/pomotimer/main/assets/bell.mp3
mv "$TMP_DIR/bell.mp3" "$SHARE_DIR_SOUNDS/bell.mp3"

curl -sLo "$TMP_DIR/clink.mp3" https://raw.githubusercontent.com/pakeelz/pomotimer/main/assets/clink.mp3
mv "$TMP_DIR/clink.mp3" "$SHARE_DIR_SOUNDS/clink.mp3"

# Cleanup
rm -rf "$TMP_DIR"

# Create .desktop file
APPLICATIONS_DIR="$HOME/.local/share/applications"
mkdir -p "$APPLICATIONS_DIR"
POMOTIMER_DESKTOP="$APPLICATIONS_DIR/pomotimer.desktop"

cat <<EOF >"$POMOTIMER_DESKTOP"
[Desktop Entry]
Type=Application
Name=Pomotimer
Comment=Pomodoro Timer On Terminal
Exec=/usr/local/bin/pomotimer
Icon=$HOME/.local/share/pomotimer/icon/timer.png
Terminal=true
Categories=Utility;
EOF

update-desktop-database "$APPLICATIONS_DIR"

echo "==> Installation Done. Give a star on https://github.com/$REPO if you like this app."
