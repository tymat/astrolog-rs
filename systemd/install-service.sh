#!/bin/bash

# Exit on error
set -e

# Get the current user
CURRENT_USER=$(whoami)
SERVICE_NAME="astrolog-rs@${CURRENT_USER}"

# Verify Swiss Ephemeris setup
SWISSEPH_DIR="$HOME/.swisseph"
if [ ! -d "$SWISSEPH_DIR" ]; then
    echo "Error: Swiss Ephemeris directory not found at $SWISSEPH_DIR"
    echo "Please follow the setup instructions in the README.md"
    exit 1
fi

# Verify required directories and files
for dir in "ephe" "lib" "include"; do
    if [ ! -d "$SWISSEPH_DIR/$dir" ]; then
        echo "Error: Required directory $SWISSEPH_DIR/$dir not found"
        echo "Please follow the setup instructions in the README.md"
        exit 1
    fi
done

# Verify ephemeris files
if [ ! -f "$SWISSEPH_DIR/ephe/seas_18.se1" ]; then
    echo "Error: Required ephemeris file not found at $SWISSEPH_DIR/ephe/seas_18.se1"
    echo "Please follow the setup instructions in the README.md"
    exit 1
fi

# Check if the service is already installed
if systemctl is-active --quiet "$SERVICE_NAME"; then
    echo "Service is already running. Stopping it first..."
    systemctl stop "$SERVICE_NAME"
fi

# Copy the service file
cp astrolog-rs.service /etc/systemd/system/

# Reload systemd
systemctl daemon-reload

# Enable and start the service
systemctl enable "$SERVICE_NAME"
systemctl start "$SERVICE_NAME"

echo "Astrolog-rs service has been installed and started for user $CURRENT_USER"
echo "The service will run on port 8808 by default"
echo "You can change the port by editing the service file and setting the PORT environment variable"
echo "You can check the status with: systemctl status $SERVICE_NAME"
echo "View logs with: journalctl -u $SERVICE_NAME" 