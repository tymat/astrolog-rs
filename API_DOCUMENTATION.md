# Astrolog-rs API Documentation

## Overview

Astrolog-rs is a high-performance astrology API server built in Rust that provides natal chart calculations, transit analysis, and synastry comparisons using the Swiss Ephemeris for astronomical accuracy.

## Base URL

```
http://127.0.0.1:4008
```

## Recent Changes

### Version Updates

#### Tight Transit Orbs Implementation
- **Transit aspects now use tight orbs (< 3 degrees)** for more precise analysis
- **Natal chart aspects maintain standard orbs** for accuracy
- **No breaking changes** to API endpoints or response formats
- **Improved performance** with fewer calculated aspects

#### Major Aspects System
- **Major aspects only by default** (conjunction, sextile, square, trine, opposition)
- **Optional minor aspects** via `include_minor_aspects` parameter
- **Eliminated duplicate aspects** with closest-aspect-only logic
- **Enhanced precision** in aspect calculations

## API Endpoints

### 1. Health Check

**Endpoint:** `GET /health`

**Description:** Check server health and service status.

**Response:**
```json
{
  "status": "healthy",
  "timestamp": "2025-05-28T00:19:19.123Z",
  "service": "astrolog-rs",
  "version": "0.2.0",
  "checks": {
    "ephemeris": "available",
    "server": "running"
  }
}
```

### 2. Natal Chart with Transits

**Endpoint:** `POST /api/chart`

**Description:** Generate a natal chart with optional transit data. This is the main endpoint that supports both natal chart calculation and transit analysis.

**Request Body:**
```json
{
  "date": "1977-10-24T04:56:00Z",
  "latitude": 14.6486,
  "longitude": 121.0508,
  "house_system": "placidus",
  "ayanamsa": "tropical",
  "include_minor_aspects": false,
  "transit": {
    "date": "2025-05-27T12:00:00Z",
    "latitude": 19.49,
    "longitude": -155.99
  }
}
```

**Request Parameters:**
- `date` (string, required): Birth date/time in ISO 8601 format
- `latitude` (number, required): Birth latitude in decimal degrees
- `longitude` (number, required): Birth longitude in decimal degrees
- `house_system` (string, required): House system ("placidus", "koch", "equal", "wholesign", "campanus", "regiomontanus")
- `ayanamsa` (string, required): Ayanamsa system ("tropical", "lahiri", etc.)
- `include_minor_aspects` (boolean, optional): Include minor aspects (default: false)
- `transit` (object, optional): Transit calculation data
  - `date` (string, required): Transit date/time in ISO 8601 format
  - `latitude` (number, optional): Transit location latitude (default: 51.45)
  - `longitude` (number, optional): Transit location longitude (default: 0.05)

**Response:**
```json
{
  "chart_type": "natal",
  "date": "1977-10-24T04:56:00Z",
  "latitude": 14.6486,
  "longitude": 121.0508,
  "house_system": "placidus",
  "ayanamsa": "tropical",
  "planets": [
    {
      "name": "Sun",
      "longitude": 0.5234,
      "latitude": 0.0012,
      "speed": 0.9856,
      "is_retrograde": false,
      "house": 1
    }
  ],
  "houses": [
    {
      "number": 1,
      "longitude": 15.2345,
      "latitude": 0.0
    }
  ],
  "aspects": [
    {
      "aspect": "Conjunction",
      "orb": 2.34,
      "planet1": "Sun",
      "planet2": "Moon"
    }
  ],
  "transit": {
    "date": "2025-05-27T12:00:00Z",
    "latitude": 19.49,
    "longitude": -155.99,
    "planets": [
      {
        "name": "Sun",
        "longitude": 66.1234,
        "latitude": 0.0023,
        "speed": 0.9845,
        "is_retrograde": false,
        "house": 3
      }
    ],
    "aspects": [
      {
        "aspect": "Square",
        "orb": 1.23,
        "planet1": "Sun",
        "planet2": "Mars"
      }
    ],
    "transit_to_natal_aspects": [
      {
        "aspect": "Trine",
        "orb": 0.87,
        "planet1": "Natal Sun",
        "planet2": "Transit Jupiter"
      }
    ]
  }
}
```

### 3. Natal Chart Only

**Endpoint:** `POST /api/chart/natal`

**Description:** Generate a natal chart without transit calculations.

**Request Body:**
```json
{
  "date": "1977-10-24T04:56:00Z",
  "latitude": 14.6486,
  "longitude": 121.0508,
  "house_system": "placidus",
  "ayanamsa": "tropical",
  "include_minor_aspects": false
}
```

**Response:**
```json
{
  "chart_type": "natal",
  "date": "1977-10-24T04:56:00Z",
  "latitude": 14.6486,
  "longitude": 121.0508,
  "house_system": "placidus",
  "ayanamsa": "tropical",
  "planets": [
    {
      "name": "Sun",
      "longitude": 0.5234,
      "latitude": 0.0012,
      "speed": 0.9856,
      "is_retrograde": false,
      "house": 1
    }
  ],
  "houses": [
    {
      "number": 1,
      "longitude": 15.2345,
      "latitude": 0.0
    }
  ],
  "aspects": [
    {
      "aspect": "Conjunction",
      "orb": 2.34,
      "planet1": "Sun",
      "planet2": "Moon"
    }
  ],
  "transit": null
}
```

### 4. Transit Chart

**Endpoint:** `POST /api/chart/transit`

**Description:** Generate a transit chart comparing natal positions to transit positions.

**Request Body:**
```json
{
  "natal_date": "1977-10-24T04:56:00Z",
  "transit_date": "2025-05-27T12:00:00Z",
  "latitude": 14.6486,
  "longitude": 121.0508,
  "house_system": "placidus",
  "ayanamsa": "tropical",
  "include_minor_aspects": false
}
```

**Request Parameters:**
- `natal_date` (string, required): Birth date/time in ISO 8601 format
- `transit_date` (string, required): Transit date/time in ISO 8601 format
- `latitude` (number, required): Location latitude in decimal degrees
- `longitude` (number, required): Location longitude in decimal degrees
- `house_system` (string, required): House system
- `ayanamsa` (string, required): Ayanamsa system
- `include_minor_aspects` (boolean, optional): Include minor aspects (default: false)

**Response:**
```json
{
  "chart_type": "transit",
  "natal_date": "1977-10-24T04:56:00Z",
  "transit_date": "2025-05-27T12:00:00Z",
  "latitude": 14.6486,
  "longitude": 121.0508,
  "house_system": "placidus",
  "ayanamsa": "tropical",
  "natal_planets": [
    {
      "name": "Sun",
      "longitude": 0.5234,
      "latitude": 0.0012,
      "speed": 0.9856,
      "is_retrograde": false,
      "house": 1
    }
  ],
  "transit_planets": [
    {
      "name": "Sun",
      "longitude": 66.1234,
      "latitude": 0.0023,
      "speed": 0.9845,
      "is_retrograde": false,
      "house": 3
    }
  ],
  "natal_aspects": [
    {
      "aspect": "Conjunction",
      "orb": 2.34,
      "planet1": "Sun",
      "planet2": "Moon"
    }
  ],
  "transit_aspects": [
    {
      "aspect": "Square",
      "orb": 1.23,
      "planet1": "Sun",
      "planet2": "Mars"
    }
  ]
}
```

### 5. Synastry Chart

**Endpoint:** `POST /api/chart/synastry`

**Description:** Generate a synastry chart comparing two natal charts.

**Request Body:**
```json
{
  "chart1": {
    "date": "1977-10-24T04:56:00Z",
    "latitude": 14.6486,
    "longitude": 121.0508,
    "house_system": "placidus",
    "ayanamsa": "tropical",
    "include_minor_aspects": false
  },
  "chart2": {
    "date": "1985-03-15T14:30:00Z",
    "latitude": 40.7128,
    "longitude": -74.0060,
    "house_system": "placidus",
    "ayanamsa": "tropical",
    "include_minor_aspects": false
  }
}
```

**Response:**
```json
{
  "chart_type": "synastry",
  "chart1": {
    "chart_type": "natal",
    "date": "1977-10-24T04:56:00Z",
    "latitude": 14.6486,
    "longitude": 121.0508,
    "house_system": "placidus",
    "ayanamsa": "tropical",
    "planets": [...],
    "houses": [...],
    "aspects": [...],
    "transit": null
  },
  "chart2": {
    "chart_type": "natal",
    "date": "1985-03-15T14:30:00Z",
    "latitude": 40.7128,
    "longitude": -74.0060,
    "house_system": "placidus",
    "ayanamsa": "tropical",
    "planets": [...],
    "houses": [...],
    "aspects": [...],
    "transit": null
  },
  "synastries": [
    {
      "aspect": "Trine",
      "orb": 1.45,
      "planet1": "Natal Sun",
      "planet2": "Natal Moon"
    }
  ]
}
```

## Data Types

### Planet Information
```json
{
  "name": "Sun",
  "longitude": 0.5234,
  "latitude": 0.0012,
  "speed": 0.9856,
  "is_retrograde": false,
  "house": 1
}
```

### House Information
```json
{
  "number": 1,
  "longitude": 15.2345,
  "latitude": 0.0
}
```

### Aspect Information
```json
{
  "aspect": "Conjunction",
  "orb": 2.34,
  "planet1": "Sun",
  "planet2": "Moon"
}
```

## Aspect Types

### Major Aspects (Default)
- **Conjunction** (0°) - Orb: 10° natal, 3° transit
- **Sextile** (60°) - Orb: 8° natal, 3° transit
- **Square** (90°) - Orb: 10° natal, 3° transit
- **Trine** (120°) - Orb: 10° natal, 3° transit
- **Opposition** (180°) - Orb: 10° natal, 3° transit

### Minor Aspects (Optional)
- **SemiSextile** (30°) - Orb: 3° natal, 2° transit
- **SemiSquare** (45°) - Orb: 3° natal, 2° transit
- **Quintile** (72°) - Orb: 3° natal, 2° transit
- **BiQuintile** (144°) - Orb: 3° natal, 2° transit
- **Sesquisquare** (135°) - Orb: 3° natal, 2° transit
- **Quincunx** (150°) - Orb: 3° natal, 2° transit

### Harmonic Aspects (Optional)
- **Septile** (51.43°) - Orb: 2° natal, 1.5° transit
- **BiSeptile** (102.86°) - Orb: 2° natal, 1.5° transit
- **TriSeptile** (154.29°) - Orb: 2° natal, 1.5° transit
- **Novile** (40°) - Orb: 2° natal, 1.5° transit
- **BiNovile** (80°) - Orb: 2° natal, 1.5° transit
- **QuadNovile** (160°) - Orb: 2° natal, 1.5° transit

## House Systems

- **placidus** - Placidus (default)
- **koch** - Koch
- **equal** - Equal House
- **wholesign** - Whole Sign
- **campanus** - Campanus
- **regiomontanus** - Regiomontanus

## Ayanamsa Systems

- **tropical** - Western Tropical (default)
- **lahiri** - Lahiri (Chitrapaksha)
- **raman** - Raman
- **krishnamurti** - Krishnamurti

## Planets Included

1. **Sun**
2. **Moon**
3. **Mercury**
4. **Venus**
5. **Mars**
6. **Jupiter**
7. **Saturn**
8. **Uranus**
9. **Neptune**
10. **Pluto**

## Error Responses

### 400 Bad Request
```json
{
  "error": "Invalid request format",
  "message": "Missing required field: date"
}
```

### 500 Internal Server Error
```json
{
  "error": "Calculation failed",
  "message": "Failed to calculate planet positions"
}
```

## Usage Examples

### Basic Natal Chart
```bash
curl -X POST http://127.0.0.1:4008/api/chart/natal \
  -H "Content-Type: application/json" \
  -d '{
    "date": "1977-10-24T04:56:00Z",
    "latitude": 14.6486,
    "longitude": 121.0508,
    "house_system": "placidus",
    "ayanamsa": "tropical"
  }'
```

### Natal Chart with Transits
```bash
curl -X POST http://127.0.0.1:4008/api/chart \
  -H "Content-Type: application/json" \
  -d '{
    "date": "1977-10-24T04:56:00Z",
    "latitude": 14.6486,
    "longitude": 121.0508,
    "house_system": "placidus",
    "ayanamsa": "tropical",
    "include_minor_aspects": true,
    "transit": {
      "date": "2025-05-27T12:00:00Z",
      "latitude": 19.49,
      "longitude": -155.99
    }
  }'
```

### Transit Chart
```bash
curl -X POST http://127.0.0.1:4008/api/chart/transit \
  -H "Content-Type: application/json" \
  -d '{
    "natal_date": "1977-10-24T04:56:00Z",
    "transit_date": "2025-05-27T12:00:00Z",
    "latitude": 14.6486,
    "longitude": 121.0508,
    "house_system": "placidus",
    "ayanamsa": "tropical",
    "include_minor_aspects": false
  }'
```

### Synastry Chart
```bash
curl -X POST http://127.0.0.1:4008/api/chart/synastry \
  -H "Content-Type: application/json" \
  -d '{
    "chart1": {
      "date": "1977-10-24T04:56:00Z",
      "latitude": 14.6486,
      "longitude": 121.0508,
      "house_system": "placidus",
      "ayanamsa": "tropical",
      "include_minor_aspects": false
    },
    "chart2": {
      "date": "1985-03-15T14:30:00Z",
      "latitude": 40.7128,
      "longitude": -74.0060,
      "house_system": "placidus",
      "ayanamsa": "tropical",
      "include_minor_aspects": false
    }
  }'
```

## Performance Notes

- **Transit aspects use tight orbs** for better performance and precision
- **Major aspects only by default** reduces calculation time
- **Concurrent request handling** with configurable limits
- **Swiss Ephemeris integration** for astronomical accuracy
- **Request logging and monitoring** for debugging

## Rate Limiting

- **Maximum concurrent calculations**: 500
- **Maximum queue size**: 10,000
- **Maximum wait time**: 30 seconds

## Server Configuration

- **Default port**: 4008
- **Workers**: 10
- **Request timeout**: 30 seconds
- **Logging**: Enabled with request/response tracking

## Dependencies

- **Swiss Ephemeris**: For astronomical calculations
- **Actix-web**: Web framework
- **Serde**: JSON serialization
- **Chrono**: Date/time handling
- **Tokio**: Async runtime 