use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HouseSystem {
    Placidus = 0,
    Koch = 1,
    Equal = 2,
    WholeSign = 3,
    Campanus = 4,
    Regiomontanus = 5,
    Meridian = 6,
    Alcabitius = 7,
    Morinus = 8,
    Krusinski = 9,
}

impl FromStr for HouseSystem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "placidus" => Ok(HouseSystem::Placidus),
            "koch" => Ok(HouseSystem::Koch),
            "equal" => Ok(HouseSystem::Equal),
            "wholesign" => Ok(HouseSystem::WholeSign),
            "campanus" => Ok(HouseSystem::Campanus),
            "regiomontanus" => Ok(HouseSystem::Regiomontanus),
            "meridian" => Ok(HouseSystem::Meridian),
            "alcabitius" => Ok(HouseSystem::Alcabitius),
            "morinus" => Ok(HouseSystem::Morinus),
            "krusinski" => Ok(HouseSystem::Krusinski),
            _ => Err(format!("Invalid house system: {}", s)),
        }
    }
} 