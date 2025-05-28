#!/bin/bash

# Example requests for the new /chart endpoint with transit functionality

echo "=== Testing /chart endpoint with custom transit data ==="
curl -X POST http://localhost:4008/api/chart \
  -H "Content-Type: application/json" \
  -d '{
    "date": "1977-10-24T04:56:00Z",
    "latitude": 14.6486,
    "longitude": 121.0508,
    "house_system": "placidus",
    "ayanamsa": "tropical",
    "transit": {
      "date": "2025-05-27T12:00:00Z",
      "latitude": 19.49,
      "longitude": -155.99
    }
  }' | jq '.'

echo ""
echo "=== Testing /chart endpoint without transit data (uses defaults) ==="
curl -X POST http://localhost:4008/api/chart \
  -H "Content-Type: application/json" \
  -d '{
    "date": "1977-10-24T04:56:00Z",
    "latitude": 14.6486,
    "longitude": 121.0508,
    "house_system": "placidus",
    "ayanamsa": "tropical"
  }' | jq '.'

echo ""
echo "=== Testing original /chart/natal endpoint (still works) ==="
curl -X POST http://localhost:4008/api/chart/natal \
  -H "Content-Type: application/json" \
  -d '{
    "date": "1977-10-24T04:56:00Z",
    "latitude": 14.6486,
    "longitude": 121.0508,
    "house_system": "placidus",
    "ayanamsa": "tropical"
  }' | jq '.' 