#!/bin/bash

# Exit on error
set -e

# Get the current user
CURRENT_USER=$(whoami)
SERVICE_NAME="astrolog-rs@${CURRENT_USER}"

# Stop and disable the service
systemctl stop "$SERVICE_NAME"
systemctl disable "$SERVICE_NAME"

# Remove the service file
rm -f /etc/systemd/system/astrolog-rs.service

# Reload systemd
systemctl daemon-reload

# Remove the binary
rm -f /usr/local/bin/astrolog-rs

# Remove the installation directory
rm -rf /opt/astrolog-rs

# Remove the user and group if they exist
if id "astrolog" &>/dev/null; then
    userdel astrolog
fi

echo "Astrolog-rs service has been uninstalled for user $CURRENT_USER" 