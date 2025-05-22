#!/bin/bash

# Exit on error
set -e

# Get the current user
CURRENT_USER=$(whoami)
LAUNCH_AGENTS_DIR="$HOME/Library/LaunchAgents"
LOG_DIR="$HOME/Library/Logs/astrolog-rs"

# Create required directories
mkdir -p "$LAUNCH_AGENTS_DIR"
mkdir -p "$LOG_DIR"

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

# Stop the service if it's running
if launchctl list | grep -q "com.astrolog-rs"; then
    echo "Stopping existing service..."
    launchctl unload "$LAUNCH_AGENTS_DIR/com.astrolog-rs.plist"
fi

# Copy the service file
cp com.astrolog-rs.plist "$LAUNCH_AGENTS_DIR/"

# Load the service
launchctl load "$LAUNCH_AGENTS_DIR/com.astrolog-rs.plist"

echo "Astrolog-rs service has been installed and started for user $CURRENT_USER"
echo "The service will run on port 4008 by default"
echo "You can check the status with: launchctl list | grep astrolog-rs"
echo "View logs at: $LOG_DIR/output.log and $LOG_DIR/error.log" 