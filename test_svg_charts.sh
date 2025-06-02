#!/bin/bash

echo "Testing SVG Chart Generation"
echo "============================"

# Start the server in the background
echo "Starting astrolog-rs server..."
cargo run --bin astrolog-rs &
SERVER_PID=$!

# Wait for server to start
sleep 3

# Test natal chart with SVG
echo "Testing natal chart endpoint..."
curl -s -X POST http://127.0.0.1:4008/api/chart/natal \
  -H "Content-Type: application/json" \
  -d '{
    "date": "1977-10-24T04:56:00Z",
    "latitude": 14.6486,
    "longitude": 121.0508,
    "house_system": "placidus",
    "ayanamsa": "tropical"
  }' | jq '.svg_chart != null' | grep -q true && echo "✓ Natal chart SVG generated" || echo "✗ Natal chart SVG failed"

# Test chart with transits
echo "Testing chart with transits endpoint..."
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
  }' | jq '.svg_chart != null' | grep -q true && echo "✓ Chart with transits SVG generated" || echo "✗ Chart with transits SVG failed"

# Test synastry chart
echo "Testing synastry chart endpoint..."
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
      "date": "1985-05-15T10:30:00Z",
      "latitude": 34.0522,
      "longitude": -118.2437,
      "house_system": "placidus",
      "ayanamsa": "tropical"
    }
  }' | jq '.svg_chart != null' | grep -q true && echo "✓ Synastry chart SVG generated" || echo "✗ Synastry chart SVG failed"

# Test transit chart
echo "Testing transit chart endpoint..."
curl -s -X POST http://127.0.0.1:4008/api/chart/transit \
  -H "Content-Type: application/json" \
  -d '{
    "natal_date": "1977-10-24T04:56:00Z",
    "transit_date": "2025-01-20T12:00:00Z",
    "latitude": 14.6486,
    "longitude": 121.0508,
    "house_system": "placidus",
    "ayanamsa": "tropical"
  }' | jq '.svg_chart != null' | grep -q true && echo "✓ Transit chart SVG generated" || echo "✗ Transit chart SVG failed"

# Save a sample SVG chart to file
echo "Saving sample SVG chart..."
curl -s -X POST http://127.0.0.1:4008/api/chart/natal \
  -H "Content-Type: application/json" \
  -d '{
    "date": "1977-10-24T04:56:00Z",
    "latitude": 14.6486,
    "longitude": 121.0508,
    "house_system": "placidus",
    "ayanamsa": "tropical"
  }' | jq -r '.svg_chart' > sample_natal_chart.svg && echo "✓ Sample chart saved to sample_natal_chart.svg"

# Clean up
echo "Stopping server..."
kill $SERVER_PID
wait $SERVER_PID 2>/dev/null

echo "Testing complete!" 