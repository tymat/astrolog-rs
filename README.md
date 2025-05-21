# Astrolog-rs

A high-precision astrological calculation library written in Rust, providing accurate planetary positions, house systems, and aspect calculations.

## Features

- Accurate planetary position calculations using VSOP87 theory
- Support for all major house systems (Placidus, Koch, Equal, Whole Sign)
- Comprehensive aspect calculations with configurable orbs
- Retrograde motion detection
- Latitude calculations for all planets
- Julian date conversions and time calculations
- Error handling with detailed error types
- Serialization support for all data structures

## Project Structure

```
astrolog-rs/
├── src/
│   ├── calc/           # Core calculation modules
│   │   ├── planets.rs  # Planetary position calculations
│   │   ├── houses.rs   # House system calculations
│   │   ├── aspects.rs  # Aspect calculations
│   │   ├── vsop87.rs   # VSOP87 theory implementation
│   │   └── utils.rs    # Utility functions
│   ├── core/           # Core types and constants
│   │   └── types.rs    # Type definitions and error handling
│   └── lib.rs          # Library entry point
├── tests/              # Integration tests
└── Cargo.toml          # Project configuration
```

## Usage

```rust
use astrolog_rs::calc::planets::{Planet, calculate_planet_position};
use astrolog_rs::calc::houses::{HouseSystem, calculate_houses};
use astrolog_rs::calc::aspects::calculate_aspects;

// Calculate planetary positions
let jd = 2451545.0; // January 1, 2000
let positions = calculate_planet_positions(jd)?;

// Calculate houses
let houses = calculate_houses(
    jd,
    40.7128, // New York latitude
    -74.0060, // New York longitude
    HouseSystem::Placidus
)?;

// Calculate aspects
let aspects = calculate_aspects(&positions, &[8.0, 6.0, 8.0, 8.0, 10.0]);
```

## Error Handling

The library provides detailed error handling through the `AstrologError` enum:

```rust
pub enum AstrologError {
    CalculationError { message: String },
    HouseSystemError { message: String, system: String },
    CoordinateError { message: String, from: String, to: String },
    AspectError { message: String, planets: (String, String) },
    DateTimeError { message: String, date: Option<DateTime<Utc>> },
    LocationError { message: String, latitude: Option<f64>, longitude: Option<f64> },
    NotImplemented { message: String },
    InvalidInput { message: String, parameter: String },
}
```

## Testing

Run the test suite with:

```bash
cargo test
```

The test suite includes:
- Planetary position accuracy tests
- House system calculation tests
- Aspect calculation tests
- Coordinate transformation tests
- Error handling tests

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details. 