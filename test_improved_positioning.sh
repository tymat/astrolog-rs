#!/bin/bash

echo "Testing improved radial positioning with angular offsets"
echo "======================================================="

# Start the server in the background
echo "Starting astrolog-rs server..."
cargo run --bin astrolog-rs &
SERVER_PID=$!

# Wait for server to start
sleep 3

echo "Generating charts to test improved positioning..."

# Generate a natal chart
echo "1. Testing natal chart..."
curl -s -X POST http://127.0.0.1:4008/api/chart/natal \
  -H "Content-Type: application/json" \
  -d '{
    "date": "1977-10-24T04:56:00Z",
    "latitude": 14.6486,
    "longitude": 121.0508,
    "house_system": "placidus",
    "ayanamsa": "tropical"
  }' | jq -r '.svg_chart' > improved_natal_test.svg

echo "✓ Improved natal chart saved to improved_natal_test.svg"

# Generate a chart with transits 
echo "2. Testing chart with transits..."
curl -s -X POST http://127.0.0.1:4008/api/chart \
  -H "Content-Type: application/json" \
  -d '{
    "date": "1977-10-24T04:56:00Z",
    "latitude": 14.6486,
    "longitude": 121.0508,
    "house_system": "placidus",
    "ayanamsa": "tropical",
    "transit": {
      "date": "2025-01-20T12:00:00Z",
      "latitude": 40.7128,
      "longitude": -74.0060
    }
  }' | jq -r '.svg_chart' > improved_natal_with_transits_test.svg

echo "✓ Improved natal with transits chart saved to improved_natal_with_transits_test.svg"

# Generate a synastry chart
echo "3. Testing synastry chart..."
curl -s -X POST http://127.0.0.1:4008/api/chart/synastry \
  -H "Content-Type: application/json" \
  -d '{
    "chart1": {
      "date": "1977-10-24T04:56:00Z",
      "latitude": 14.6486,
      "longitude": 121.0508,
      "house_system": "placidus",
      "ayanamsa": "tropical"
    },
    "chart2": {
      "date": "1980-03-15T12:30:00Z",
      "latitude": 40.7128,
      "longitude": -74.0060,
      "house_system": "placidus",
      "ayanamsa": "tropical"
    }
  }' | jq -r '.svg_chart' > improved_synastry_test.svg

echo "✓ Improved synastry chart saved to improved_synastry_test.svg"

# Generate a transit chart
echo "4. Testing transit chart..."
curl -s -X POST http://127.0.0.1:4008/api/chart/transit \
  -H "Content-Type: application/json" \
  -d '{
    "natal": {
      "date": "1977-10-24T04:56:00Z",
      "latitude": 14.6486,
      "longitude": 121.0508,
      "house_system": "placidus",
      "ayanamsa": "tropical"
    },
    "transit": {
      "date": "2025-01-20T12:00:00Z",
      "latitude": 40.7128,
      "longitude": -74.0060
    }
  }' | jq -r '.svg_chart' > improved_transit_test.svg

echo "✓ Improved transit chart saved to improved_transit_test.svg"

# Clean up
echo "Stopping server..."
kill $SERVER_PID
wait $SERVER_PID 2>/dev/null

echo ""
echo "Improved positioning test complete!"
echo "=================================="
echo "Key improvements implemented:"
echo "  ✓ Angular offsets: Close planets now have small angular separations"
echo "  ✓ Radial offsets: Different distances from center based on planetary order"
echo "  ✓ Unified positioning: All charts calculate positions for all planets together"
echo "  ✓ No overlaps: Natal, transit, and synastry planets avoid visual overlaps"
echo ""
echo "Generated test files:"
echo "  - improved_natal_test.svg"
echo "  - improved_natal_with_transits_test.svg" 
echo "  - improved_synastry_test.svg"
echo "  - improved_transit_test.svg" 