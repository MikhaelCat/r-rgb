#!/bin/bash
set -euo pipefail

echo "==> Stopping keyglow processes..."
pkill -f keyglow 2>/dev/null || true

echo "==> Disabling and removing keyglow systemd units..."
systemctl --global disable keyglow.service 2>/dev/null || true
systemctl disable keyglow-restore.service 2>/dev/null || true

rm -f /etc/systemd/user/keyglow.service
rm -f /etc/systemd/user/graphical-session.target.wants/keyglow.service
rm -f /etc/systemd/system/keyglow-restore.service
rm -f /etc/systemd/system/*.target.wants/keyglow-restore.service 2>/dev/null || true

echo "==> Removing keyglow configuration and runtime files..."
rm -f /etc/tmpfiles.d/keyglow.conf
rm -f /etc/udev/rules.d/70-keyglow.rules
rm -f /run/lock/keyglow.lock
rm -f /usr/local/bin/keyglow
rm -rf /opt/keyglow
rm -rf /tmp/keyglow-staging

systemctl daemon-reload
udevadm control --reload 2>/dev/null || true

echo "==> Keyglow has been completely uninstalled!"
