use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use crate::calc::planets::{Planet, PlanetPosition};
use crate::calc::houses::HousePosition;
use crate::calc::coordinates::calculate_julian_date;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chart {
    pub date: DateTime<Utc>,
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: f64,
    pub house_system: String,
    pub planets: Vec<(Planet, PlanetPosition)>,
    pub houses: HousePosition,
}

impl Chart {
    pub fn new(
        date: DateTime<Utc>,
        latitude: f64,
        longitude: f64,
        timezone: f64,
        house_system: String,
    ) -> Self {
        Self {
            date,
            latitude,
            longitude,
            timezone,
            house_system,
            planets: Vec::new(),
            houses: HousePosition::new([0.0; 12], crate::core::HouseSystem::Placidus),
        }
    }

    pub fn get_planet_position(&self, planet: Planet) -> Option<&PlanetPosition> {
        self.planets
            .iter()
            .find(|(p, _)| *p == planet)
            .map(|(_, pos)| pos)
    }

    pub fn get_house_cusp(&self, house: u8) -> Option<f64> {
        self.houses.get_cusp(house)
    }
}

/// Generate a complete astrological chart for the given parameters
pub fn generate_chart(
    date: DateTime<Utc>,
    latitude: f64,
    longitude: f64,
    timezone: f64,
    house_system: String,
) -> Result<Chart, String> {
    // Calculate Julian date
    let year = date.year();
    let month = date.month();
    let day = date.day();
    let hour = date.hour() as f64;
    let minute = date.minute() as f64;
    let second = date.second() as f64;

    let julian_date = calculate_julian_date(
        year,
        month,
        day,
        hour,
        minute,
        second,
        timezone,
    );

    // Create chart structure
    let mut chart = Chart::new(date, latitude, longitude, timezone, house_system);

    // Calculate planet positions
    for planet in [
        Planet::Sun,
        Planet::Moon,
        Planet::Mercury,
        Planet::Venus,
        Planet::Mars,
        Planet::Jupiter,
        Planet::Saturn,
        Planet::Uranus,
        Planet::Neptune,
        Planet::Pluto,
    ].iter() {
        let position = crate::calc::planets::calculate_planet_position(*planet, julian_date)?;
        chart.planets.push((*planet, position));
    }

    // Calculate house cusps
    let house_system = match house_system.as_str() {
        "P" | "p" | "Placidus" => crate::core::HouseSystem::Placidus,
        "K" | "k" | "Koch" => crate::core::HouseSystem::Koch,
        "O" | "o" | "Porphyrius" => crate::core::HouseSystem::Porphyrius,
        "R" | "r" | "Regiomontanus" => crate::core::HouseSystem::Regiomontanus,
        "C" | "c" | "Campanus" => crate::core::HouseSystem::Campanus,
        "A" | "a" | "Equal" => crate::core::HouseSystem::Equal,
        "V" | "v" | "Vehlow" => crate::core::HouseSystem::Vehlow,
        "W" | "w" | "Whole" => crate::core::HouseSystem::Whole,
        "M" | "m" | "Meridian" => crate::core::HouseSystem::Meridian,
        "B" | "b" | "Alcabitius" => crate::core::HouseSystem::Alcabitius,
        "X" | "x" | "Morinus" => crate::core::HouseSystem::Morinus,
        "Y" | "y" | "Vedic" => crate::core::HouseSystem::Vedic,
        _ => return Err("Invalid house system".into()),
    };

    chart.houses = crate::calc::houses::calculate_houses(
        julian_date,
        latitude,
        longitude,
        house_system,
    )?;

    Ok(chart)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_chart_creation() {
        let date = Utc.with_ymd_and_hms(1977, 10, 24, 4, 56, 0).unwrap();
        let chart = generate_chart(
            date,
            14.65,  // 14:38:55N
            121.05, // 121:03:03E
            0.0,    // ST Zone 0W
            "Placidus".to_string(),
        ).unwrap();

        assert_eq!(chart.latitude, 14.65);
        assert_eq!(chart.longitude, 121.05);
        assert_eq!(chart.timezone, 0.0);
        assert_eq!(chart.house_system, "Placidus");
    }
} 