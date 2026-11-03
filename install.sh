#!/bin/bash
set -euo pipefail

PREFIX="${PREFIX:-/usr/local}"
BIN_DIR="$PREFIX/bin"

echo "==> Building rrgb release binary..."
cargo build --release

echo "==> Installing binary to $BIN_DIR/rrgb..."
install -Dm755 target/release/rrgb "$BIN_DIR/rrgb"

echo "==> Installing udev rules for MSI / SteelSeries..."
install -Dm644 packaging/50-rrgb.rules /etc/udev/rules.d/50-rrgb.rules
udevadm control --reload 2>/dev/null || true
udevadm trigger 2>/dev/null || true

echo "==> Installing systemd services for boot & resume auto-restore..."
install -Dm644 packaging/rrgb.service /etc/systemd/user/rrgb.service
install -Dm644 packaging/rrgb-restore.service /etc/systemd/system/rrgb-restore.service

systemctl daemon-reload
systemctl enable rrgb-restore.service 2>/dev/null || true
systemctl --global enable rrgb.service 2>/dev/null || true

echo "==> Installation complete! Try running: rrgb status or rrgb check"
