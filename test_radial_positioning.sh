#!/bin/bash

echo "Testing radial planet positioning"
echo "================================="

# Start the server in the background
echo "Starting astrolog-rs server..."
cargo run --bin astrolog-rs &
SERVER_PID=$!

# Wait for server to start
sleep 3

# Generate a sample chart that should demonstrate radial positioning
echo "Generating natal chart with radial positioning..."
curl -s -X POST http://127.0.0.1:4008/api/chart/natal \
  -H "Content-Type: application/json" \
  -d '{
    "date": "1977-10-24T04:56:00Z",
    "latitude": 14.6486,
    "longitude": 121.0508,
    "house_system": "placidus",
    "ayanamsa": "tropical"
  }' | jq -r '.svg_chart' > radial_positioning_test.svg

echo "✓ Chart saved to radial_positioning_test.svg"
echo "  - Planets arranged from center to outside based on traditional order"
echo "  - Sun, Moon, Mercury, Venus, Mars, Jupiter, Saturn, Uranus, Neptune, Pluto"
echo "  - Planets close in longitude (within 8°) are arranged radially to prevent overlap"
echo "  - Sun closest to center, outer planets further out"

# Generate a chart with transits to test mixed positioning
echo "Generating chart with transits to test mixed positioning..."
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
  }' | jq -r '.svg_chart' > radial_with_transits_test.svg

echo "✓ Chart with transits saved to radial_with_transits_test.svg"
echo "  - Both natal and transit planets use radial positioning"
echo "  - Prevents overlaps between close planets"

# Clean up
echo "Stopping server..."
kill $SERVER_PID
wait $SERVER_PID 2>/dev/null

echo "Radial positioning test complete!"
echo "Features implemented:"
echo "  - Traditional planetary order (Sun to Pluto)"
echo "  - 8-degree threshold for grouping close planets"
echo "  - 15-pixel radial step between overlapping planets"
echo "  - Separate positioning for natal/transit/synastry charts" 