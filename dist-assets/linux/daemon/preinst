#!/usr/bin/env bash
set -eu

if which systemctl &> /dev/null && systemctl is-system-running | grep -vq offline &> /dev/null; then
    if systemctl status mullvad-daemon &> /dev/null; then
        /opt/SocialCommand\ VPN/resources/mullvad-setup prepare-restart || true
        systemctl stop socialcommand-vpn.service
        systemctl disable socialcommand-vpn.service
        systemctl disable socialcommand-vpn-early-boot-blocking.service || true
        cp /var/log/socialcommand-vpn/daemon.log /var/log/socialcommand-vpn/old-install-daemon.log \
            || echo "Failed to copy old daemon log"
    fi
fi

rm -f /var/cache/socialcommand-vpn/relays.json
rm -f /var/cache/socialcommand-vpn/api-ip-address.txt
