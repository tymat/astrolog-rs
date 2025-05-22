#!/bin/bash

# Function to install hey
install_hey() {
    if [[ "$OSTYPE" == "darwin"* ]]; then
        echo "Installing hey using Homebrew..."
        if ! command -v brew &> /dev/null; then
            echo "Homebrew is not installed. Please install Homebrew first:"
            echo "/bin/bash -c \"\$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\""
            exit 1
        fi
        brew install hey
    elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
        echo "Please install hey on Linux:"
        echo "go install github.com/rakyll/hey@latest"
        exit 1
    fi
}

# Check if hey is installed
if ! command -v hey &> /dev/null; then
    echo "hey is not installed."
    install_hey
fi

# Verify hey is now installed
if ! command -v hey &> /dev/null; then
    echo "Failed to install hey. Please install it manually:"
    echo "macOS: brew install hey"
    echo "Linux: go install github.com/rakyll/hey@latest"
    exit 1
fi

# Set file descriptor limit
ulimit -n 1048576 2>/dev/null || echo "Note: Could not set file descriptor limit. Run with sudo for full optimization."

# Function to run a load test with increasing concurrency
run_load_test() {
    local endpoint=$1
    local payload=$2
    local name=$3
    
    echo "Testing $name endpoint with increasing concurrency..."
    
    # Test with 100 concurrent connections
    echo -e "\nTesting with 100 concurrent connections:"
    hey -z 60s -c 100 -n 1000 -m POST -H "Content-Type: application/json" -D "$payload" http://localhost:4008/api/chart/$endpoint
    
    # Wait for connections to close
    sleep 10
    
    # Test with 200 concurrent connections
    echo -e "\nTesting with 200 concurrent connections:"
    hey -z 60s -c 200 -n 2000 -m POST -H "Content-Type: application/json" -D "$payload" http://localhost:4008/api/chart/$endpoint
    
    # Wait for connections to close
    sleep 10
    
    # Test with 500 concurrent connections
    echo -e "\nTesting with 500 concurrent connections:"
    hey -z 60s -c 500 -n 5000 -m POST -H "Content-Type: application/json" -D "$payload" http://localhost:4008/api/chart/$endpoint
    
    # Wait for connections to close
    sleep 10
    
    # Test with 1000 concurrent connections
    echo -e "\nTesting with 1000 concurrent connections:"
    hey -z 60s -c 1000 -n 10000 -m POST -H "Content-Type: application/json" -D "$payload" http://localhost:4008/api/chart/$endpoint
    
    # Wait for connections to close
    sleep 10
}

# Run tests for each endpoint
run_load_test "natal" "natal_payload.json" "Natal Chart"
run_load_test "transit" "transit_payload.json" "Transit Chart"
run_load_test "synastry" "synastry_payload.json" "Synastry Chart"
