# SVG Chart Generation Feature

## Overview

The astrolog-rs project now includes SVG chart generation for all chart endpoints. Each API response includes a complete SVG chart visualization as the `svg_chart` field.

## Features

- **Automatic SVG Generation**: All chart endpoints automatically generate SVG charts
- **Customizable Styling**: Chart appearance controlled via `chart_styles.json` configuration file
- **Multiple Chart Types**: Support for natal, synastry, and transit charts
- **Visual Requirements Compliance**: 
  - Planet degree numbers under symbols with borders
  - Long dotted lines for synastry aspects  
  - Short dotted lines for transit aspects
  - Different border styles for different chart types

## Configuration

### Chart Styles (`chart_styles.json`)

Create a `chart_styles.json` file in the project root to customize chart appearance:

```json
{
  "planet_colors": {
    "Sun": "#FF6B35",
    "Moon": "#4ECDC4",
    "Mercury": "#45B7D1",
    "Venus": "#96CEB4",
    "Mars": "#FFEAA7",
    "Jupiter": "#DDA0DD",
    "Saturn": "#98D8C8",
    "Uranus": "#6C5CE7",
    "Neptune": "#74B9FF",
    "Pluto": "#A29BFE"
  },
  "chart_colors": {
    "background": "#FFFFFF",
    "wheel_background": "#10002B", 
    "chart_wheel_line": "#9dade0",
    "chart1_planet_border": "#252c42",
    "chart2_planet_border": "#854077",
    "transit_planet_border": "#8dad8c",
    "chart_text_color": "#a1a4b3",
    "chart_aspect_color": "#cbcfb4"
  },
  "aspect_line_colors": {
    "Conjunction": "#FF6B6B",
    "Opposition": "#4ECDC4", 
    "Trine": "#45B7D1",
    "Square": "#FFA07A",
    "Sextile": "#98D8E8"
  }
}
```

If the file is not found, default styles are used automatically.

## API Response Format

All chart endpoints now include an `svg_chart` field containing the complete SVG markup:

```json
{
  "chart_type": "natal",
  "date": "1977-10-24T04:56:00Z",
  "latitude": 14.6486,
  "longitude": 121.0508,
  "house_system": "placidus",
  "ayanamsa": "tropical",
  "planets": [...],
  "houses": [...],
  "aspects": [...],
  "svg_chart": "<svg width=\"800\" height=\"800\" viewBox=\"0 0 800 800\" xmlns=\"http://www.w3.org/2000/svg\">...</svg>"
}
```

## Supported Endpoints

### 1. Natal Chart (`/api/chart/natal`)
- Generates natal chart with planets, houses, and aspects
- Square borders for natal planets

### 2. Chart with Transits (`/api/chart`)
- Generates natal chart with optional transit overlay
- Square borders for natal planets
- Dashed square borders for transit planets
- Short dotted lines for transit aspects

### 3. Synastry Chart (`/api/chart/synastry`)
- Generates combined chart for two people
- Square borders for chart1 planets
- Circle borders for chart2 planets  
- Long dotted lines for synastry aspects

### 4. Transit Chart (`/api/chart/transit`)
- Generates natal chart with transit planets
- Square borders for natal planets
- Dashed square borders for transit planets
- Short dotted lines for transit aspects

## Visual Elements

### Planet Symbols
- Unicode astronomical symbols (☉ ☽ ☿ ♀ ♂ ♃ ♄ ♅ ♆ ♇)
- Degree information displayed below each symbol
- Color-coded based on planet type
- Enclosed in styled borders
- **Radial positioning**: Planets arranged from center to outside based on traditional order
- **Overlap prevention**: Close planets (within 8°) arranged at different radii

### Chart Wheel
- 12 zodiac sign divisions with symbols (♈ ♉ ♊ ♋ ♌ ♍ ♎ ♏ ♐ ♑ ♒ ♓)
- House cusp lines radiating from center (50% opacity, rendered behind other elements)
- House numbers positioned within each house
- Zodiac division lines (50% opacity, rendered behind other elements)

### Aspect Lines
- Color-coded by aspect type
- Different line styles for different chart types:
  - Solid lines for natal aspects
  - Short dotted lines for transit aspects  
  - Long dotted lines for synastry aspects
- Semi-transparent for visual clarity

## Technical Implementation

### Architecture
- **Styles Module** (`src/charts/styles.rs`): Configuration loading and management
- **SVG Generator** (`src/charts/svg_generator.rs`): Core SVG generation logic
- **Chart Module** (`src/charts/mod.rs`): Public API for chart generation

### Key Components
- **Chart Background**: Configurable colors and gradients
- **Zodiac Wheel**: 12 divisions with astrological symbols
- **Layered Rendering**: Division/house lines (50% opacity) behind planets/aspects
- **Radial Planet Positioning**: Traditional order (Sun→Pluto) with overlap prevention
- **Proximity Grouping**: Planets within 8° arranged at stepped radii (15px intervals)
- **Aspect Calculation**: Precise geometric line drawing
- **House System**: Support for multiple house systems

### Performance
- SVG generation adds ~1-2ms to response time
- Charts are generated on-demand (not cached)
- Memory efficient using streaming SVG construction

## Usage Examples

### Basic Natal Chart
```bash
curl -X POST http://localhost:4008/api/chart/natal \
  -H "Content-Type: application/json" \
  -d '{
    "date": "1977-10-24T04:56:00Z",
    "latitude": 14.6486,
    "longitude": 121.0508,
    "house_system": "placidus",
    "ayanamsa": "tropical"
  }'
```

### Chart with Transits
```bash
curl -X POST http://localhost:4008/api/chart \
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
  }'
```

### Extract SVG to File
```bash
curl -X POST http://localhost:4008/api/chart/natal \
  -H "Content-Type: application/json" \
  -d '{"date":"1977-10-24T04:56:00Z","latitude":14.6486,"longitude":121.0508,"house_system":"placidus","ayanamsa":"tropical"}' \
  | jq -r '.svg_chart' > chart.svg
```

## Testing

Run the test suite to verify SVG generation:

```bash
# Run all API tests
cargo test --test api_tests

# Run SVG-specific tests
cargo test --test api_tests test_natal_chart_endpoint
cargo test --test api_tests test_chart_endpoint_with_transits  
cargo test --test api_tests test_synastry_chart_endpoint

# Test with demo script
./test_svg_charts.sh

# Test opacity changes
./test_opacity_chart.sh

# Test radial positioning
./test_radial_positioning.sh

# Test synastry radial positioning
./test_synastry_radial.sh
```

## Dependencies

- **svg**: Rust crate for SVG generation
- **serde**: JSON serialization/deserialization
- **lazy_static**: Static initialization of default styles

## Troubleshooting

### Styles Not Loading
- Ensure `chart_styles.json` exists in project root
- Check file permissions and JSON syntax
- Server logs will show fallback to default styles

### SVG Not Generated
- Verify all required dependencies are installed
- Check that chart styles are properly initialized
- Review server logs for initialization errors

### Visual Issues
- Verify chart_styles.json color values are valid hex codes
- Check that all required style sections are present
- Test with default styles by removing/renaming config file

## Future Enhancements

- House system visualization differences
- Configurable chart size and dimensions
- Additional astronomical symbols and glyphs
- Interactive SVG elements
- Custom aspect orb visualization
- Multi-language zodiac symbols 