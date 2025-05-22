#!/bin/bash

# Set file descriptor limit for this script and child processes
ulimit -n 1048576

# Common ab settings for high concurrency
AB_SETTINGS="-k -r -l -t 60"

# Function to run a load test with increasing concurrency
run_load_test() {
    local endpoint=$1
    local payload=$2
    local name=$3
    
    echo "Testing $name endpoint with increasing concurrency..."
    
    # Test with 200 concurrent connections
    echo -e "\nTesting with 200 concurrent connections:"
    ab $AB_SETTINGS -c 200 -n 2000 -p $payload -T application/json http://localhost:4008/api/chart/$endpoint
    
    # Test with 500 concurrent connections
    echo -e "\nTesting with 500 concurrent connections:"
    ab $AB_SETTINGS -c 500 -n 5000 -p $payload -T application/json http://localhost:4008/api/chart/$endpoint
    
    # Test with 1000 concurrent connections
    echo -e "\nTesting with 1000 concurrent connections:"
    ab $AB_SETTINGS -c 1000 -n 10000 -p $payload -T application/json http://localhost:4008/api/chart/$endpoint
    
    # Test with 1500 concurrent connections
    echo -e "\nTesting with 1500 concurrent connections:"
    ab $AB_SETTINGS -c 1500 -n 15000 -p $payload -T application/json http://localhost:4008/api/chart/$endpoint
}

# Run tests for each endpoint
run_load_test "natal" "natal_payload.json" "Natal Chart"
run_load_test "transit" "transit_payload.json" "Transit Chart"
run_load_test "synastry" "synastry_payload.json" "Synastry Chart"
