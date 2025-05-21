# Astrolog-rs

A modern implementation of astrological calculations in Rust, focusing on accuracy and performance.

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
curl -X POST http://localhost:8080/api/chart/natal \
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
curl -X POST http://localhost:8080/api/chart/transit \
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
curl -X POST http://localhost:8080/api/chart/synastry \
  -H "Content-Type: application/json" \
  -d '{
    "chart1": {
      "date": "1990-01-01T12:00:00Z",
      "latitude": 40.7128,
      "longitude": -74.0060,
      "house_system": "placidus",
      "ayanamsa": "tropical"
    },
    "chart2": {
      "date": "1992-06-15T15:30:00Z",
      "latitude": 34.0522,
      "longitude": -118.2437,
      "house_system": "placidus",
      "ayanamsa": "tropical"
    }
  }'
```

#### Response Format
```json
{
  "chart_type": "natal",
  "date": "2024-01-14T12:00:00Z",
  "latitude": 40.7128,
  "longitude": -74.0060,
  "house_system": "placidus",
  "ayanamsa": "tropical",
  "planets": [
    {
      "name": "Sun",
      "longitude": 294.5,
      "latitude": 0.0,
      "speed": 0.9856,
      "is_retrograde": false,
      "house": 10
    },
    // ... other planets
  ],
  "houses": [
    {
      "number": 1,
      "longitude": 120.5
    },
    // ... other houses
  ],
  "aspects": [
    {
      "planet1": "Sun",
      "planet2": "Moon",
      "aspect": "conjunction",
      "orb": 5.2
    },
    // ... other aspects
  ]
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



