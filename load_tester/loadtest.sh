#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check for required tools
check_dependency() {
    if ! command -v $1 &> /dev/null; then
        echo -e "${RED}Error: $1 is not installed${NC}"
        echo -e "${YELLOW}Installation instructions:${NC}"
        case $1 in
            wrk)
                echo "On macOS:"
                echo "  brew install wrk"
                echo "On Ubuntu/Debian:"
                echo "  sudo apt-get install wrk"
                echo "On CentOS/RHEL:"
                echo "  sudo yum install wrk"
                ;;
        esac
        exit 1
    fi
}

# Check for required tools
check_dependency "wrk"

# Configuration
HOST="http://localhost:4008"
DURATION="30s"
THREADS=8  # Increased threads for better performance
CONNECTIONS=1000  # Start with 1000 concurrent connections
LOG_FILE="../request_errors.log"  # Path to the log file in the project root

echo "Starting load tests with $CONNECTIONS concurrent connections..."

# Test 1: Natal Chart
echo -e "\n${GREEN}Test 1: Natal Chart${NC}"
wrk -t$THREADS -c$CONNECTIONS -d$DURATION -s natal_payload.lua $HOST/api/chart/natal

# Test 2: Transit Chart
echo -e "\n${GREEN}Test 2: Transit Chart${NC}"
wrk -t$THREADS -c$CONNECTIONS -d$DURATION -s transit_payload.lua $HOST/api/chart/transit

# Test 3: Synastry Chart
echo -e "\n${GREEN}Test 3: Synastry Chart${NC}"
wrk -t$THREADS -c$CONNECTIONS -d$DURATION -s synastry_payload.lua $HOST/api/chart/synastry

# Test 4: Error Case (Invalid Latitude)
echo -e "\n${GREEN}Test 4: Error Case (Invalid Latitude)${NC}"
wrk -t$THREADS -c$CONNECTIONS -d$DURATION -s error_payload.lua $HOST/api/chart/transit

echo -e "\n${GREEN}Load tests completed!${NC}"

# Check error log file size
echo -e "\n${GREEN}Checking error log file:${NC}"
if [ -f "$LOG_FILE" ]; then
    echo "Error log file size: $(du -h "$LOG_FILE" | cut -f1)"
    echo "Last few error entries:"
    tail -n 5 "$LOG_FILE"
else
    echo -e "${RED}Error log file not found at $LOG_FILE${NC}"
fi 