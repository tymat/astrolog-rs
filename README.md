# Astrolog-rs

A modern astrology calculation engine and API service written in Rust, porting the functionality of the original Astrolog program.

## Project Goals

- Port Astrolog's core calculation functionality to Rust
- Provide a REST/JSON-RPC API for generating astrological charts
- Maintain calculation accuracy and compatibility with the original program
- Create a modern, maintainable codebase with comprehensive testing

## Features

- Planetary position calculations (VSOP87)
- House system calculations (Placidus, Koch, Equal, etc.)
- Aspect calculations with orbs
- Chart generation with multiple output formats
- REST API for chart generation and calculations
- Comprehensive test suite

## Project Structure

```
astrolog-rs/
├── src/
│   ├── api/          # API endpoints and routing
│   │   ├── aspects.rs    # Aspect calculations
│   │   ├── coordinates.rs # Coordinate conversions
│   │   ├── houses.rs     # House system calculations
│   │   ├── planets.rs    # Planetary calculations
│   │   └── utils.rs      # Calculation utilities
│   ├── charts/       # Chart generation and rendering
│   ├── core/         # Core types and data structures
│   ├── data/         # Constants and static data
│   ├── io/           # File I/O operations
│   └── utils/        # General utilities
└── tests/            # Test suite
```

## Key Components

### Core Types
- `Chart`: Main chart data structure
- `ChartInfo`: Chart metadata (date, time, location)
- `ChartPositions`: Planetary positions
- `Position`: Individual position data
- `Aspect`: Aspect data between planets
- `HouseSystem`: Supported house systems
- `AspectType`: Supported aspect types

### Calculation Modules
- Coordinate conversions (ecliptic, equatorial, horizontal)
- House system calculations
- Aspect calculations with orbs
- Planetary position calculations
- Retrograde and station calculations

### API Endpoints
- `/health`: Health check endpoint
- `/api/v1/chart`: Chart generation endpoint
- `/api/v1/transit`: Transit calculation endpoint

## Testing Approach

The project uses a comprehensive testing strategy:

1. Unit Tests
   - Individual function testing
   - Edge case handling
   - Mathematical accuracy verification

2. Functional Tests
   - End-to-end chart generation
   - API endpoint testing
   - Data validation

3. Integration Tests
   - Cross-module functionality
   - API integration
   - File I/O operations

## Development Status

- [x] Project structure setup
- [x] Core types and data structures
- [x] Basic API framework
- [x] Coordinate conversion module
- [x] Aspect calculation module
- [x] House system module
- [ ] Planetary calculations (VSOP87)
- [ ] Chart generation
- [ ] API endpoints
- [ ] File I/O operations
- [ ] Documentation
- [ ] Test suite

## Building and Running

See [BUILD.md](BUILD.md) for detailed build instructions.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run the test suite
5. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Original Astrolog program by Walter D. Pullen
- VSOP87 theory by P. Bretagnon and G. Francou
- Swiss Ephemeris for reference calculations 