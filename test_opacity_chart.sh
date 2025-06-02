#!/bin/bash

echo "Testing opacity changes in chart generation"
echo "==========================================="

# Start the server in the background
echo "Starting astrolog-rs server..."
cargo run --bin astrolog-rs &
SERVER_PID=$!

# Wait for server to start
sleep 3

# Generate a sample chart with the new opacity settings
echo "Generating sample chart with 50% opacity house/divider lines..."
curl -s -X POST http://127.0.0.1:4008/api/chart/natal \
  -H "Content-Type: application/json" \
  -d '{
    "date": "1977-10-24T04:56:00Z",
    "latitude": 14.6486,
    "longitude": 121.0508,
    "house_system": "placidus",
    "ayanamsa": "tropical"
  }' | jq -r '.svg_chart' > opacity_test_chart.svg

echo "✓ Chart saved to opacity_test_chart.svg"
echo "  - House cusp lines are now 50% opacity"
echo "  - Zodiac division lines are now 50% opacity"
echo "  - Lines render behind planets and aspects"

# Clean up
echo "Stopping server..."
kill $SERVER_PID
wait $SERVER_PID 2>/dev/null

echo "Test complete! Open opacity_test_chart.svg to see the visual changes." 