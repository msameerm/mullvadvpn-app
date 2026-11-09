#!/usr/bin/env bash
set -eu

chmod u+s "/usr/bin/mullvad-exclude"

systemctl enable "/usr/lib/systemd/system/socialcommand-vpn.service"
systemctl start socialcommand-vpn.service || echo "Failed to start socialcommand-vpn.service"
systemctl enable "/usr/lib/systemd/system/socialcommand-vpn-early-boot-blocking.service"

# Check if the system supports a new-enough AppArmor version.
function supported_apparmor() {
    [[ -e /etc/apparmor.d/abi/4.0 ]]
}

if supported_apparmor; then
    # Install our AppArmor profile and try to reload AppArmor.
    # The AppArmor profile allow Electron sandbox to work.
    # This disables user namespace restrictions.
    echo "Creating apparmor profile"
    cp /opt/SocialCommand\ VPN/resources/apparmor_socialcommand-vpn /etc/apparmor.d/socialcommand-vpn
    apparmor_parser -r /etc/apparmor.d/socialcommand-vpn || echo "Failed to reload apparmor profile"
fi
