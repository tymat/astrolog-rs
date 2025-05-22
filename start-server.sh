#!/bin/bash

# Set system limits
ulimit -n 1048576
ulimit -u 65535
ulimit -s 65535

# Set TCP settings
sysctl -w net.inet.tcp.keepidle=60000
sysctl -w net.inet.tcp.keepintvl=10000
sysctl -w net.inet.tcp.keepcnt=6
sysctl -w net.inet.tcp.max_syn_backlog=4096
sysctl -w kern.ipc.somaxconn=4096

# Set environment variables
export PORT=4008
export RUST_LOG=info
export WORKERS=4
export RUST_BACKTRACE=1
export MAX_CONCURRENT=250
export SE_EPHE_PATH="$HOME/.swisseph/ephe"
export LD_LIBRARY_PATH="$HOME/.swisseph/lib"
export CPATH="$HOME/.swisseph/include"

# Start the server
cargo run --release 