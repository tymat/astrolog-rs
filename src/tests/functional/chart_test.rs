use astrolog_rs::core::{ChartInfo, ChartPositions};
use astrolog_rs::calc::{
    calculate_planet_positions,
    calculate_houses,
    calculate_julian_date,
    HouseSystem
};
use chrono::{DateTime, Utc, TimeZone};
use approx::assert_relative_eq;

/// Test data from original Astrolog output
const TEST_CHART_DATA: &str = r#"
Astrolog 7.50 chart for Mon Oct 24, 1977  4:56am (ST Zone 0W) 121.05E 14.65N
Body  Locat. Ret. Lati. Rul.      House  Rul. Veloc.    Equal Houses

Sun : 210.674   +0.0001 (-) [ 9th house] [-] +0.995  -  House cusp  1: 310.315
Moon: 358.595   +1.5177 (-) [ 2nd house] [X] +12.82  -  House cusp  2: 340.315
Merc: 214.148   +0.2340 (-) [ 9th house] [d] +1.632  -  House cusp  3:  10.315
Venu: 188.853   +1.5671 (R) [ 8th house] [d] +1.242  -  House cusp  4:  40.315
Mars: 118.878   +1.2190 (f) [ 6th house] [-] +0.440  -  House cusp  5:  70.315
Jupi:  96.142   -0.3561 (X) [ 5th house] [-] +0.000  -  House cusp  6: 100.315
Satu: 148.485   +1.1716 (d) [ 7th house] [X] +0.080  -  House cusp  7: 130.315
Uran: 221.400   +0.3886 (X) [10th house] [-] +0.061  -  House cusp  8: 160.315
Nept: 254.296   +1.4347 (-) [11th house] [-] +0.029  -  House cusp  9: 190.315
Plut: 194.736   +16.546 (-) [ 9th house] [-] +0.038  -  House cusp 10: 220.315
Chir:   0.000   +0.0000 (-) [ 2nd house] [-] +0.000  -  House cusp 11: 250.315
Cere:   0.000   +0.0000 (-) [ 2nd house] [R] +0.000  -  House cusp 12: 280.315
Pall:   0.000   +0.0000 (-) [ 2nd house] [-] +0.000
Juno:   0.000   +0.0000 (d) [ 2nd house] [-] +0.000     Car Fix Mut TOT   +:12
Vest:   0.000   +0.0000 (-) [ 2nd house] [d] +0.000  Fir  5   1   1   7   -: 8
Nort: 194.198 R +0.0000 (-) [ 9th house] [f] -0.052  Ear  0   0   1   1   M: 9
Lili:  80.421   -4.7107 (-) [ 5th house] [-] +0.111  Air  3   1   1   5   N:11
Fort:  98.235   +0.0000 (-) [ 5th house] [-] +398.9  Wat  3   3   1   7   A: 9
Vert: 162.930   +0.0000 (f) [ 8th house] [-] +139.2  TOT 11   5   4  20   D:11
East: 315.072   +0.0000 (-) [ 1st house] [R] +362.4                       <:11
"#;

#[test]
fn test_chart_generation() {
    // Create chart info for the test case
    let chart_info = ChartInfo {
        date: Utc.with_ymd_and_hms(1977, 10, 24, 4, 56, 0).unwrap(),
        latitude: 14.65,
        longitude: 121.05,
        timezone: 0.0,
        house_system: HouseSystem::Placidus,
    };

    // Calculate planetary positions
    let jd = calculate_julian_date(
        chart_info.date.year(),
        chart_info.date.month(),
        chart_info.date.day(),
        chart_info.date.hour() as f64,
        chart_info.date.minute() as f64,
        chart_info.date.second() as f64,
        chart_info.timezone,
    );

    let positions = calculate_planet_positions(jd).unwrap();

    // Verify Sun position
    assert_relative_eq!(positions[0].longitude, 210.674, epsilon = 0.001);
    assert_relative_eq!(positions[0].latitude, 0.0001, epsilon = 0.001);
    assert_relative_eq!(positions[0].speed_longitude, 0.995, epsilon = 0.001);

    // Verify Moon position
    assert_relative_eq!(positions[1].longitude, 358.595, epsilon = 0.001);
    assert_relative_eq!(positions[1].latitude, 1.5177, epsilon = 0.001);
    assert_relative_eq!(positions[1].speed_longitude, 12.82, epsilon = 0.001);

    // Verify Mercury position
    assert_relative_eq!(positions[2].longitude, 214.148, epsilon = 0.001);
    assert_relative_eq!(positions[2].latitude, 0.2340, epsilon = 0.001);
    assert_relative_eq!(positions[2].speed_longitude, 1.632, epsilon = 0.001);

    // Calculate house cusps
    let mut chart_positions = ChartPositions {
        zodiac_positions: positions.iter().map(|p| p.longitude).collect(),
        house_placements: vec![0; positions.len()],
        house_cusps: vec![0.0; 12],
    };

    calculate_houses(&chart_info, &mut chart_positions, HouseSystem::Placidus).unwrap();

    // Verify house cusps
    assert_relative_eq!(chart_positions.house_cusps[0], 310.315, epsilon = 0.001);
    assert_relative_eq!(chart_positions.house_cusps[1], 340.315, epsilon = 0.001);
    assert_relative_eq!(chart_positions.house_cusps[2], 10.315, epsilon = 0.001);
    assert_relative_eq!(chart_positions.house_cusps[3], 40.315, epsilon = 0.001);
    assert_relative_eq!(chart_positions.house_cusps[4], 70.315, epsilon = 0.001);
    assert_relative_eq!(chart_positions.house_cusps[5], 100.315, epsilon = 0.001);
    assert_relative_eq!(chart_positions.house_cusps[6], 130.315, epsilon = 0.001);
    assert_relative_eq!(chart_positions.house_cusps[7], 160.315, epsilon = 0.001);
    assert_relative_eq!(chart_positions.house_cusps[8], 190.315, epsilon = 0.001);
    assert_relative_eq!(chart_positions.house_cusps[9], 220.315, epsilon = 0.001);
    assert_relative_eq!(chart_positions.house_cusps[10], 250.315, epsilon = 0.001);
    assert_relative_eq!(chart_positions.house_cusps[11], 280.315, epsilon = 0.001);
}

#[test]
fn test_house_placements() {
    // Create chart info for the test case
    let chart_info = ChartInfo {
        date: Utc.with_ymd_and_hms(1977, 10, 24, 4, 56, 0).unwrap(),
        latitude: 14.65,
        longitude: 121.05,
        timezone: 0.0,
        house_system: HouseSystem::Placidus,
    };

    // Calculate planetary positions
    let jd = calculate_julian_date(
        chart_info.date.year(),
        chart_info.date.month(),
        chart_info.date.day(),
        chart_info.date.hour() as f64,
        chart_info.date.minute() as f64,
        chart_info.date.second() as f64,
        chart_info.timezone,
    );

    let positions = calculate_planet_positions(jd).unwrap();

    let mut chart_positions = ChartPositions {
        zodiac_positions: positions.iter().map(|p| p.longitude).collect(),
        house_placements: vec![0; positions.len()],
        house_cusps: vec![0.0; 12],
    };

    calculate_houses(&chart_info, &mut chart_positions, HouseSystem::Placidus).unwrap();

    // Verify house placements
    assert_eq!(chart_positions.house_placements[0], 9); // Sun in 9th house
    assert_eq!(chart_positions.house_placements[1], 2); // Moon in 2nd house
    assert_eq!(chart_positions.house_placements[2], 9); // Mercury in 9th house
    assert_eq!(chart_positions.house_placements[3], 8); // Venus in 8th house
    assert_eq!(chart_positions.house_placements[4], 6); // Mars in 6th house
    assert_eq!(chart_positions.house_placements[5], 5); // Jupiter in 5th house
    assert_eq!(chart_positions.house_placements[6], 7); // Saturn in 7th house
    assert_eq!(chart_positions.house_placements[7], 10); // Uranus in 10th house
    assert_eq!(chart_positions.house_placements[8], 11); // Neptune in 11th house
    assert_eq!(chart_positions.house_placements[9], 9); // Pluto in 9th house
} 