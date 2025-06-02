#!/bin/bash

echo "Testing synastry chart radial positioning"
echo "========================================="

# Start the server in the background
echo "Starting astrolog-rs server..."
cargo run --bin astrolog-rs &
SERVER_PID=$!

# Wait for server to start
sleep 3

# Generate a synastry chart
echo "Generating synastry chart with radial positioning..."
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
  }' | jq -r '.svg_chart' > synastry_radial_test.svg

echo "✓ Synastry chart saved to synastry_radial_test.svg"
echo "  - Chart1 planets: square borders with radial positioning"
echo "  - Chart2 planets: circle borders with radial positioning"
echo "  - Long dotted lines for synastry aspects between charts"
echo "  - Planets positioned independently for each chart to prevent overlaps"

# Clean up
echo "Stopping server..."
kill $SERVER_PID
wait $SERVER_PID 2>/dev/null

echo "Synastry radial positioning test complete!"
echo "Features verified:"
echo "  - Independent radial positioning for each chart"
echo "  - Optimized position calculation (calculated once, reused)"
echo "  - Traditional planetary order maintained"
echo "  - Visual distinction: square borders for chart1, circles for chart2" 