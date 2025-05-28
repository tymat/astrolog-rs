#!/bin/bash

# Detect OS
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS specific settings
    echo "Detected macOS, applying macOS-specific settings..."
    
    # Set file descriptor limit (this one usually works without sudo)
    ulimit -n 1048576 2>/dev/null || echo "Note: Could not set file descriptor limit. Run with sudo for full optimization."

    # Check if running with sudo
    if [ "$EUID" -ne 0 ]; then
        echo "Note: Some optimizations require sudo. For best performance, run: sudo $0"
    else
        # Set system limits (only with sudo)
        ulimit -u 65535 2>/dev/null
        ulimit -s 65535 2>/dev/null

        # Set TCP settings for macOS (only with sudo)
        sysctl -w kern.ipc.somaxconn=8192 2>/dev/null
        sysctl -w kern.maxfiles=1048576 2>/dev/null
        sysctl -w kern.maxfilesperproc=1048576 2>/dev/null
    fi

elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    # Linux specific settings
    echo "Detected Linux, applying Linux-specific settings..."
    
    # Check if running with sudo
    if [ "$EUID" -ne 0 ]; then
        echo "Note: Some optimizations require sudo. For best performance, run: sudo $0"
    else
        # Set system limits
        ulimit -n 1048576
        ulimit -u 65535
        ulimit -s 65535

        # Set TCP settings for Linux
        sysctl -w net.ipv4.tcp_keepalive_time=60
        sysctl -w net.ipv4.tcp_keepalive_intvl=10
        sysctl -w net.ipv4.tcp_keepalive_probes=6
        sysctl -w net.ipv4.tcp_max_syn_backlog=8192
        sysctl -w net.core.somaxconn=8192
        sysctl -w net.ipv4.tcp_fastopen=3
        sysctl -w net.ipv4.tcp_slow_start_after_idle=0
        sysctl -w net.ipv4.tcp_no_metrics_save=1
        sysctl -w net.ipv4.tcp_tw_reuse=1
        sysctl -w net.ipv4.tcp_fin_timeout=30
        sysctl -w net.core.netdev_max_backlog=8192
        sysctl -w net.ipv4.tcp_max_tw_buckets=2000000
        sysctl -w net.ipv4.tcp_tw_recycle=0
        sysctl -w net.ipv4.tcp_rmem='4096 87380 16777216'
        sysctl -w net.ipv4.tcp_wmem='4096 87380 16777216'
        sysctl -w net.core.rmem_max=16777216
        sysctl -w net.core.wmem_max=16777216
    fi
else
    echo "Unsupported OS: $OSTYPE"
    exit 1
fi

# Common environment variables
export PORT=4008
export RUST_LOG=info
export WORKERS=8
export RUST_BACKTRACE=1
export MAX_CONCURRENT=1000
export SE_EPHE_PATH="$HOME/.swisseph/ephe"
export LD_LIBRARY_PATH="$HOME/.swisseph/lib"
export CPATH="$HOME/.swisseph/include"
export RUSTFLAGS="-C target-cpu=native"

# Start the server
target/release/astrolog-rs
