# Astrolog-rs

A modern implementation of astrological calculations in Rust, focusing on accuracy and performance.

## Prerequisites

### Swiss Ephemeris Setup

The project uses Swiss Ephemeris for high-precision astronomical calculations. Before building, you need to set up the Swiss Ephemeris files and library:

1. **Download the Swiss Ephemeris source and ephemeris files:**
   - Go to [https://www.astro.com/swisseph/](https://www.astro.com/swisseph/) and download:
     - The Swiss Ephemeris source code (for the C library)
     - The Ephemeris files package (contains files like `seas_18.se1`, `semo_18.se1`, `sepl_18.se1`, etc.)

2. **Compile the Swiss Ephemeris C library:**
   - Extract the source code (if not already in `external/swisseph`).
   - In the `external/swisseph` directory, run:
     ```bash
     make
     ```
   - This will generate the static library files (e.g., `libswe.a`).

3. **Create the required directories:**
   ```bash
   mkdir -p $HOME/.swisseph/lib
   mkdir -p $HOME/.swisseph/include
   mkdir -p $HOME/.swisseph/ephe
   ```

4. **Copy the compiled library and header files:**
   ```bash
   # Copy library and header files
   cp external/swisseph/*.h $HOME/.swisseph/include/
   cp external/swisseph/*.a $HOME/.swisseph/lib/
   ```

5. **Copy the ephemeris files:**
   - Copy all `.se1` and other ephemeris files from the downloaded ephemeris package to:
   ```bash
   cp /path/to/downloaded/ephemeris/files/* $HOME/.swisseph/ephe/
   ```
   - Ensure that files like `seas_18.se1`, `semo_18.se1`, and `sepl_18.se1` are present in `$HOME/.swisseph/ephe`.

6. **Set the environment variable (if needed):**
   - By default, the application looks for ephemeris files in `$HOME/.swisseph/ephe`. If you want to override this or run from a different location, set the environment variable before running:
   ```bash
   export SE_EPHE_PATH=$HOME/.swisseph/ephe
   ```
   - You can add this to your shell profile (e.g., `~/.bashrc`, `~/.zshrc`) to make it permanent.

## Features

### Planetary Calculations
- Accurate VSOP87-based planetary position calculations
- Support for all major planets (Sun through Pluto)
- Proper handling of retrograde motion detection
- Stationary point calculations
- Geocentric and heliocentric coordinate systems

### Coordinate Systems
- Ecliptic to equatorial coordinate conversion
- Proper handling of coordinate system transformations
- Support for different ayanamsas (sidereal offsets)

### House Systems
- Multiple house system calculations
- Accurate house cusp determination
- Support for different house system methods

## Technical Details

### Planetary Motion
- Uses VSOP87 theory for high-precision planetary positions
- Implements proper retrograde motion detection with optimized time deltas:
  - Mars: 0.1 Julian centuries (3650 days) for accurate retrograde detection
  - Mercury: 0.01 Julian centuries (365 days) for precise motion tracking
- Handles 0°/360° boundary crossing in speed calculations
- Provides detailed debug output for motion analysis

### Coordinate Calculations
- Time-dependent obliquity of ecliptic
- Proper handling of coordinate transformations
- Support for different coordinate systems and reference frames

### Performance
- Written in Rust for high performance and safety
- Efficient algorithms for astronomical calculations
- Optimized coordinate transformations

## Usage

### Library Usage
```rust
use astrolog_rs::calc::planets;

// Calculate planetary positions for a given Julian date
let jd = 2460314.5; // January 14, 2024
let positions = planets::calculate_planet_positions(jd)?;

// Access individual planet positions
for position in positions {
    println!("Longitude: {:.2}°, Latitude: {:.2}°, Speed: {:.2}°/day, Retrograde: {}",
        position.longitude,
        position.latitude,
        position.speed,
        position.is_retrograde
    );
}
```

### API Usage

The server provides a REST API for generating astrological charts. Here are some example requests:

#### Generate a Natal Chart
```bash
curl -X POST http://localhost:4008/api/chart/natal \
  -H "Content-Type: application/json" \
  -d '{
    "date": "2024-01-14T12:00:00Z",
    "latitude": 40.7128,
    "longitude": -74.0060,
    "house_system": "placidus",
    "ayanamsa": "tropical"
  }'
```

#### Generate a Transit Chart
```bash
curl -X POST http://localhost:4008/api/chart/transit \
  -H "Content-Type: application/json" \
  -d '{
    "natal_date": "1990-01-01T12:00:00Z",
    "transit_date": "2024-01-14T12:00:00Z",
    "latitude": 40.7128,
    "longitude": -74.0060,
    "house_system": "placidus",
    "ayanamsa": "tropical"
  }'
```

#### Generate a Synastry Chart
```bash
curl -X POST http://localhost:4008/api/chart/synastry \
  -H "Content-Type: application/json" \
  -d '{
    "chart1": {
      "date": "1977-10-24T12:00:00Z",
      "latitude": 14.6488,
      "longitude": 121.0509,
      "house_system": "placidus",
      "ayanamsa": "tropical"
    },
    "chart2": {
      "date": "1996-01-06T12:00:00Z",
      "latitude": 36.66833,
      "longitude": 116.99722,
      "house_system": "placidus",
      "ayanamsa": "tropical"
    }
  }'
```

Response:
```json
{
  "chart_type": "synastry",
  "chart1": {
    "chart_type": "natal",
    "date": "1977-10-24T12:00:00Z",
    "latitude": 14.6488,
    "longitude": 121.0509,
    "house_system": "placidus",
    "ayanamsa": "tropical",
    "planets": [...],
    "houses": [...]
  },
  "chart2": {
    "chart_type": "natal",
    "date": "1996-01-06T12:00:00Z",
    "latitude": 36.66833,
    "longitude": 116.99722,
    "house_system": "placidus",
    "ayanamsa": "tropical",
    "planets": [...],
    "houses": [...]
  },
  "aspects": [...]
}
```

## API Documentation

### Endpoints

#### 1. Natal Chart
Generate a natal chart for a given location and birth date/time.

```http
POST /api/chart/natal
```

Request body:
```json
{
    "date": "2000-01-01T12:00:00Z",  // Birth date/time in ISO 8601 format
    "latitude": 40.7128,             // Birth location latitude
    "longitude": -74.0060,           // Birth location longitude
    "house_system": "placidus",      // House system (placidus, koch, equal, wholesign, etc.)
    "ayanamsa": "tropical"           // Ayanamsa system (tropical, lahiri, etc.)
}
```

Response:
```json
{
    "planets": [
        {
            "name": "Sun",
            "longitude": 280.5,
            "latitude": 0.0,
            "speed": 1.0,
            "is_retrograde": false,
            "house": 10
        },
        // ... other planets
    ],
    "houses": [
        {
            "number": 1,
            "longitude": 0.0,
            "latitude": 0.0
        },
        // ... other houses
    ],
    "aspects": [
        {
            "planet1": "Sun",
            "planet2": "Moon",
            "aspect": "Conjunction",
            "orb": 2.5
        },
        // ... other aspects
    ]
}
```

#### 2. Transit Chart
Generate a transit chart comparing current or future planetary positions to a natal chart.

```http
POST /api/chart/transit
```

Request body:
```json
{
    "natal_date": "2000-01-01T12:00:00Z",    // Birth date/time
    "transit_date": "2024-01-01T12:00:00Z",  // Current/future date to check
    "latitude": 40.7128,                     // Location latitude
    "longitude": -74.0060,                   // Location longitude
    "house_system": "placidus",              // House system
    "ayanamsa": "tropical"                   // Ayanamsa system
}
```

Response:
```json
{
    "natal_planets": [
        // Same format as natal chart planets
    ],
    "transit_planets": [
        // Same format as natal chart planets
    ],
    "aspects": [
        {
            "planet1": "Sun",
            "planet2": "Moon",
            "aspect": "Conjunction",
            "orb": 2.5
        },
        // ... other aspects
    ]
}
```

#### 3. Synastry Chart
Generate a synastry chart comparing two natal charts.

```http
POST /api/chart/synastry
```

Request body:
```json
{
    "chart1": {
        "date": "2000-01-01T12:00:00Z",    // Person A's birth date/time
        "latitude": 40.7128,                // Person A's birth location latitude
        "longitude": -74.0060,              // Person A's birth location longitude
        "house_system": "placidus",         // House system
        "ayanamsa": "tropical"              // Ayanamsa system
    },
    "chart2": {
        "date": "1995-01-01T12:00:00Z",    // Person B's birth date/time
        "latitude": 34.0522,                // Person B's birth location latitude
        "longitude": -118.2437,             // Person B's birth location longitude
        "house_system": "placidus",         // House system
        "ayanamsa": "tropical"              // Ayanamsa system
    }
}
```

Response:
```json
{
    "chart1_planets": [
        // Person A's planets
    ],
    "chart2_planets": [
        // Person B's planets
    ],
    "aspects": [
        {
            "planet1": "Sun",
            "planet2": "Moon",
            "aspect": "Conjunction",
            "orb": 2.5
        },
        // ... other aspects
    ]
}
```

### Error Responses

All endpoints return standard HTTP status codes:
- 200: Success
- 400: Bad Request (invalid input)
- 500: Internal Server Error

Error response format:
```json
{
    "error": "Error message description"
}
```

### Notes
- All dates should be in ISO 8601 format
- Latitude and longitude should be in decimal degrees
- Supported house systems: placidus, koch, equal, wholesign, campanus, regiomontanus
- Supported ayanamsa systems: tropical, lahiri, raman, krishnamurti, etc.

## Development

### Building
```bash
cargo build
```

### Testing
```bash
cargo test
```

### Running Tests with Debug Output
```bash
RUST_LOG=debug cargo test
```

### Running the Server
```bash
cargo run --bin server
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Based on the original Astrolog software
- Uses VSOP87 theory for planetary positions
- Implements algorithms from Meeus' Astronomical Algorithms

Changes in the last version Astrolog 5.41G

   * Fixed original Astrolog bug, where Local Horizon data (and also Prime
     Vertical) were off by up to several arc-minutes.

   * Fixed original Astrolog bug (Windows version), where "Time / Space
     Midpoint" chart is correct, but it was not possible to return back to
     "No relationship chart" - there was still previous "midpoint" chart.

   * In "Rising and Setting" chart fixed bug, where "Print Nearest Second"
     worked unproperly - seconds weren't shown for risings and settings.

     In this context more correct termin instead of "zenith" and "nadir" is
     astronomical termin "culmination" that means transit over meridian.
     There are two culminations: "upper culmination" when planet has
     highest position and "lower culmination" when planet has lowest
     position. So in the chart termins "zeniths" and "nadirs" are replaced
     by "culm.(up)" and "culm.(lo)" respectively.

   * Uncorrect object name "Nadir" has been replaced by corret "IC". Nadir
     has different meaning than IC, Nadir is the point on the sky opposite
     to Zenith.

   * Slightly improved Astrolog's behaviour in case of progressed and
     relationship charts. Earlyer Windows versions allowed to do
     progression and then do some relationship chart like Natal-Progressed.
     Or vice versa, when looking say Natal-Progressed chart switch also
     Progression "on". That all confused Astrolog and results were often
     unpredictable.

     Now any use of relationship charts (in menu "Info") switches off "Do
     Progression" (in "Chart" => "Progressions"). And vice versa, switching
     progression on (in "Chart" => "Progression") switches off any
     comparison. As result Astrolog is displaying just what it has to
     dasplay, no confusion.

   * Fixed orignal astrolog bug (appeared only in Windows version). When
     natal chart was opened from file with saved planets positions (not
     birth data), there only first output was correct, all next charts (as
     aspect list, transit searches, all comparison charts etc) had wrong
     planets positions.

     NB! Use such input files carefully! They are treated by astrolog as
     files "with no time and space", so a lot of chart types can't be used
     with them. An examples are all progression charts, because they needs
     birth-chart time to calculate progressions.

   * Fixed bug of changed versions, where in case of use of data with saved
     planet positions (just as above), data-border of graphics charts
     dilsplayed info "no time or space" twice.

   * Time and location have new format - with seconds, both for chart's
     data inputs and outputs. Only outputs with old data format are events
     searching outputs, because with reasonably low division value they
     anyway can be off by a few minutes, so seconds seems unnecessary
     there.

          "Rising and setting" chart is only exception among searching
          charts that shows time with seconds. Even with default setting d:
          48 times are off by only a few seconds, and it is quick enough to
          show precise (by seconds) times with d: 96 in reasonably short
          time. Note, that values of azimuth angles in rising/setting
          moment are extremely sensitive of time changes (very small change
          of time causes big change in azimuth) and can be off by a few
          seconds. To improve azimuth precision one has to increase
          division value up to hundreds.

     All input files with old time/location format can be used, but all
     outputs are saved only in new time/location format.

   * Calculations precision improved - time-dependent obliquity of ecliptic
     is used instead of fixed value.

   * Added new switsh to change local horizon text output from default ENWS
     to NESW. One has just to add line

          =YZ

     to astrolog.dat file and it will be deafult setting.

   * Slightly changed Prime Vertical text chart output: Altitude and
     Azimuth in header have been replaced by Amplitude and Prm-Vrt. One can
     see this chart by choosing "Local Horizon" and setting "Horizon Chart
     with Polar Center / Prime Vert. (text)" in "Chart settings". Note,
     that this switch has different meanings for text and graphics charts:
     in text mode it switches between Local Horizon <=> Prime Vertical
     outputs, in graphics it switches between "normal wiew" and "view vith
     Polar Center". However, one can switch also between Polar Center and
     Pime Vert. also in graphics by hitting 'i' on the keyboard (switch to
     bonus mode).

   * Changed calculations of Solar Arc. Original Astrolog calculates there
     actually directions, where all planets and house cusps positions are
     moved forward to an amount equal in degrees to the number of years
     that have passed between the specified date and the chart in question.
     Because real (even mean or average) motion of Sun isn't 1 degree per
     day, resulting Solar Arc Sun position doesn't match with secondary
     progressed Sun position (but they must match).

     To correct this situation, there has been added another calculation of
     Solar Arc - first secondary progressed Sun's position is determined
     and then all chart components are moved accordlingly.

     To avoid misunderstanding, existing Solar Acr calculations remains,
     but have now corrected name "Degree Per Day/Month", as earlyer one can
     change amount of degrees per year (more strictly, amount of days for 1
     degree direction. See description of -p0 and -pd switches in
     helpfile.540). Added calculation is named "Solar Arc Directions" and
     there Sun's position follows secondary progressed Sun positions, all
     other point in chart are moved accordlingly.

     Existing switches -p0, -p0n remain as previously. For new correct
     "Solar Arc Directions" calculations new switches -p1 and -p1n are
     used. In Windows version all can be done through menus: "Chart" =>
     "Progressions..."

   * Slightly changed chraphics charts' infoborder and text charts' headers
     - secondary progressed charts are still named as "progressed", Solar
     Arc and "Degree per Year/Mont" are marked as "directed" with name of
     direction.

   * Graphics chart's infoborder has additional information: Obliquity of
     ecliptic, Sidereal time, Delta T (in seconds) and in case of sidereal
     chart Ayanamsha (sidereal offest).

     Note, that Ayanamsha has negative value and dafaults to Fagan Bradley.
     Ayanamsha control in "Calculation Settings" has a dropdown to allow
     quick selection of some common systems of sidereal astrology. The
     values are additions to default value and they are 0.0 for Fagan
     Bradley, 0.883333 (or 0 degrees 53') for N.C. Lahiri, 0.983333 (or 0
     degrees 59') for Krishnamurti, and 2.333333 (or 2 degrees 20') for
     B.V. Raman. On the screen has shown resulting value.

   * Date/time and Julian Day in the graphics charts' infoboredr has been
     colored:

          1. First (or single) data/time is always bright white.

          2. Second date/time is always yellow.

          3. JD (which is actually also time/date) has color of
          corresponding date/time above (as yellow in case of transit
          comparison or direction charts). If JD doesn't correspond to
          neither date/time above (as in case of progressions where JD
          corresponds to date/time of planets' positions on the screen),
          color is green. Note, that in case of Synastry and Comparison
          charts JD will be yellow, becsuse it corresponds to second
          date/time above (data of second chart).

   * Windows version controls of progressed charts have been polished. They
     looks more clear now. As default 365.2422 (tropical) year has used.

     In "Degree per Days" dropdown are also available:

          365.25636 (sidereal year)
          27.321582 (tropical month)
          27.321661 (sidereal month)
          29.530588 (synodic month)

     Other more exotic (like Draconic) years/months aren't included.

   * Astrolog computes position of Lilith (Dark Moon) using external
     ephemeris. When ephemeris are set off, Astrolog will display the
     position of the South Node instead (see helpfile.540, description of
     -HO switch). For users, who wants to use South Node always, there has
     been added new switch =YN which forces Astrolog to do it. This switch
     can be entered through "Edit" => "Enter Command Line" (Windows
     version) or simply added to astrolog.dat file to make this behaviour
     default.

   * By default Astrolog calculates mean Lilith. For users who wants to use
     osculting position of Lilith, new switch -YL has been added. As above,
     this switch can be added to astrolog.dat file, which makes such
     behaviour default.

   * Dispositors glyphs on the graphics wheels can be switched on/off
     using switch -YD. It can be added to astrolg.dat file: =YD (on) or
     _YD (off), default "on".

---------------------------------------------------------------------------

18th. May. 2002

## Running as a Systemd Service

The server can be run as a systemd service, allowing it to start automatically on boot and be managed by systemd. The service runs as a regular user and supports configurable ports.

### Installation

1. Navigate to the systemd directory:
```bash
cd systemd
```

2. Run the installation script:
```bash
./install-service.sh
```

The service will be installed and started automatically. By default, it runs on port 8808.

### Configuration

The service can be configured by editing the service file at `/etc/systemd/system/astrolog-rs.service`. Key configuration options:

- `PORT`: The port number the server listens on (default: 8808)
- `RUST_LOG`: Logging level (default: info)
- `EPHE_PATH`: Path to the Swiss Ephemeris files

To apply configuration changes:
```bash
sudo systemctl daemon-reload
sudo systemctl restart astrolog-rs@username
```

### Service Management

Check service status:
```bash
systemctl status astrolog-rs@username
```

View service logs:
```bash
journalctl -u astrolog-rs@username
```

Stop the service:
```bash
sudo systemctl stop astrolog-rs@username
```

Start the service:
```bash
sudo systemctl start astrolog-rs@username
```

### Uninstallation

To remove the service:
```bash
cd systemd
./uninstall-service.sh
```

## Running as a Systemd Service

The server can be run as a systemd service, allowing it to start automatically on boot and be managed by systemd. The service runs as a regular user and supports configurable ports.

### Installation

1. Navigate to the systemd directory:
```bash
cd systemd
```

2. Run the installation script:
```bash
./install-service.sh
```

The service will be installed and started automatically. By default, it runs on port 8808.

### Configuration

The service can be configured by editing the service file at `/etc/systemd/system/astrolog-rs.service`. Key configuration options:

- `PORT`: The port number the server listens on (default: 8808)
- `RUST_LOG`: Logging level (default: info)
- `EPHE_PATH`: Path to the Swiss Ephemeris files

To apply configuration changes:
```bash
sudo systemctl daemon-reload
sudo systemctl restart astrolog-rs@username
```

### Service Management

Check service status:
```bash
systemctl status astrolog-rs@username
```

View service logs:
```bash
journalctl -u astrolog-rs@username
```

Stop the service:
```bash
sudo systemctl stop astrolog-rs@username
```

Start the service:
```bash
sudo systemctl start astrolog-rs@username
```

### Uninstallation

To remove the service:
```bash
cd systemd
./uninstall-service.sh
```

## Running as a Systemd Service

The server can be run as a systemd service, allowing it to start automatically on boot and be managed by systemd. The service runs as a regular user and supports configurable ports.

### Installation

1. Navigate to the systemd directory:
```bash
cd systemd
```

2. Run the installation script:
```bash
./install-service.sh
```

The service will be installed and started automatically. By default, it runs on port 8808.

### Configuration

The service can be configured by editing the service file at `/etc/systemd/system/astrolog-rs.service`. Key configuration options:

- `PORT`: The port number the server listens on (default: 8808)
- `RUST_LOG`: Logging level (default: info)
- `EPHE_PATH`: Path to the Swiss Ephemeris files

To apply configuration changes:
```bash
sudo systemctl daemon-reload
sudo systemctl restart astrolog-rs@username
```

### Service Management

Check service status:
```bash
systemctl status astrolog-rs@username
```

View service logs:
```bash
journalctl -u astrolog-rs@username
```

Stop the service:
```bash
sudo systemctl stop astrolog-rs@username
```

Start the service:
```bash
sudo systemctl start astrolog-rs@username
```

### Uninstallation

To remove the service:
```bash
cd systemd
./uninstall-service.sh
```

# Astrolog-rs

A high-performance astrological calculation server written in Rust.

## Features

- Natal chart calculations
- Transit chart calculations
- Synastry chart calculations
- High-performance concurrent request handling
- Swiss Ephemeris integration

## Prerequisites

- Rust (latest stable version)
- Swiss Ephemeris library and files
- For load testing: `hey` tool (install via `brew install hey` on macOS)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/astrolog-rs.git
cd astrolog-rs
```

2. Install Swiss Ephemeris:
   - Download Swiss Ephemeris files
   - Place them in `~/.swisseph/ephe/`
   - Install Swiss Ephemeris library in `~/.swisseph/lib/`

3. Build the project:
```bash
cargo build --release
```

## Running the Server

### Basic Start
To start the server with default settings:
```bash
./start-server.sh
```

### Optimized Start
For best performance, run with sudo to apply system optimizations:
```bash
sudo ./start-server.sh
```

The server will start on port 4008 by default. You can configure the following environment variables:
- `PORT`: Server port (default: 4008)
- `WORKERS`: Number of worker threads (default: 2x CPU cores)
- `MAX_CONCURRENT`: Maximum concurrent calculations (default: 1000)
- `RUST_LOG`: Log level (default: info)

## Load Testing

The project includes load testing scripts to verify performance under high concurrency.

### Prerequisites
1. Install the `hey` load testing tool:
   - macOS: `brew install hey`
   - Linux: `go install github.com/rakyll/hey@latest`

2. Make the test script executable:
```bash
chmod +x load_tester/loadtest.sh
```

### Running Load Tests
Run the load tests with:
```bash
cd load_tester
./loadtest.sh
```

The load test script will:
1. Test each endpoint (natal, transit, synastry)
2. Progressively increase concurrency (100, 200, 500, 1000)
3. Run each test for 60 seconds
4. Wait 10 seconds between tests for connection cleanup

### Test Payloads
The load tests use the following JSON payloads:
- `natal_payload.json`: Natal chart calculation parameters
- `transit_payload.json`: Transit chart calculation parameters
- `synastry_payload.json`: Synastry chart calculation parameters

## API Endpoints

### Natal Chart
```bash
curl -X POST http://localhost:4008/api/chart/natal \
  -H "Content-Type: application/json" \
  -d @natal_payload.json
```

### Transit Chart
```bash
curl -X POST http://localhost:4008/api/chart/transit \
  -H "Content-Type: application/json" \
  -d @transit_payload.json
```

### Synastry Chart
```bash
curl -X POST http://localhost:4008/api/chart/synastry \
  -H "Content-Type: application/json" \
  -d @synastry_payload.json
```

## Performance Optimization

The server is optimized for high concurrency with:
- Multiple worker threads
- Connection pooling
- Response compression
- Optimized TCP settings
- System resource limits

For best performance:
1. Run with sudo to apply system optimizations
2. Adjust `WORKERS` and `MAX_CONCURRENT` based on your system
3. Monitor system resources during load tests

## License

[Your License]



