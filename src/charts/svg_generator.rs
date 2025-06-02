use crate::api::types::{ChartResponse, PlanetInfo, AspectInfo, HouseInfo, TransitResponse, SynastryResponse};
use crate::charts::styles::get_styles;
use svg::Document;
use svg::node::element::{Circle, Line, Text, Rectangle};
use svg::node::Text as TextNode;
use std::f64::consts::PI;

const CHART_SIZE: f64 = 800.0;
const CENTER: f64 = CHART_SIZE / 2.0;
const OUTER_RADIUS: f64 = 350.0;
const INNER_RADIUS: f64 = 280.0;
const BASE_PLANET_RADIUS: f64 = 240.0;
const PLANET_RADIUS_STEP: f64 = 15.0;

pub struct SVGChartGenerator {
    pub width: f64,
    pub height: f64,
    pub center_x: f64,
    pub center_y: f64,
    pub outer_radius: f64,
}

impl Default for SVGChartGenerator {
    fn default() -> Self {
        Self {
            width: CHART_SIZE,
            height: CHART_SIZE,
            center_x: CENTER,
            center_y: CENTER,
            outer_radius: OUTER_RADIUS,
        }
    }
}

impl SVGChartGenerator {
    pub fn new() -> Self {
        Self::default()
    }

    // Traditional planetary order from center to edge
    fn get_planetary_order(&self) -> Vec<&str> {
        vec!["Sun", "Moon", "Mercury", "Venus", "Mars", "Jupiter", "Saturn", "Uranus", "Neptune", "Pluto"]
    }

    // Get planetary order index (lower = closer to center)
    fn get_planet_order_index(&self, planet_name: &str) -> usize {
        self.get_planetary_order()
            .iter()
            .position(|&p| p == planet_name)
            .unwrap_or(10) // Unknown planets go to outer edge
    }

    // Group planets by proximity in longitude
    fn group_planets_by_proximity(&self, planets: &[PlanetInfo], threshold_degrees: f64) -> Vec<Vec<PlanetInfo>> {
        let mut sorted_planets = planets.to_vec();
        sorted_planets.sort_by(|a, b| a.longitude.partial_cmp(&b.longitude).unwrap());
        
        let mut groups = Vec::new();
        let mut current_group = Vec::new();
        
        for planet in sorted_planets {
            if current_group.is_empty() {
                current_group.push(planet);
            } else {
                let last_planet = current_group.last().unwrap();
                let mut longitude_diff = (planet.longitude - last_planet.longitude).abs();
                
                // Handle wrap-around at 0/360 degrees
                if longitude_diff > 180.0 {
                    longitude_diff = 360.0 - longitude_diff;
                }
                
                if longitude_diff <= threshold_degrees {
                    current_group.push(planet);
                } else {
                    groups.push(current_group);
                    current_group = vec![planet];
                }
            }
        }
        
        if !current_group.is_empty() {
            groups.push(current_group);
        }
        
        groups
    }

    // Calculate planet positions with radial ordering
    fn calculate_planet_positions(&self, planets: &[PlanetInfo]) -> std::collections::HashMap<String, (f64, f64)> {
        let planet_groups = self.group_planets_by_proximity(planets, 8.0); // 8 degree threshold
        let mut positions = std::collections::HashMap::new();
        
        for group in planet_groups {
            if group.len() == 1 {
                // Single planet - use base radius
                let planet = &group[0];
                let angle = self.longitude_to_angle(planet.longitude);
                let (x, y) = self.calculate_position(angle, BASE_PLANET_RADIUS);
                positions.insert(planet.name.clone(), (x, y));
            } else {
                // Multiple planets close together - arrange by planetary order with angular and radial offsets
                let mut sorted_group = group;
                sorted_group.sort_by(|a, b| {
                    self.get_planet_order_index(&a.name)
                        .cmp(&self.get_planet_order_index(&b.name))
                });
                
                // Calculate the center longitude for the group
                let center_longitude = sorted_group.iter()
                    .map(|p| p.longitude)
                    .sum::<f64>() / sorted_group.len() as f64;
                
                for (i, planet) in sorted_group.iter().enumerate() {
                    // Use different radius for each planet (closer to center = higher priority)
                    let radius = BASE_PLANET_RADIUS - (i as f64 * PLANET_RADIUS_STEP);
                    
                    // Add angular offset to prevent overlap on same radial line
                    let angular_offset = (i as f64 - (sorted_group.len() - 1) as f64 / 2.0) * 2.0; // degrees
                    let adjusted_longitude = center_longitude + angular_offset;
                    let angle = self.longitude_to_angle(adjusted_longitude);
                    
                    let (x, y) = self.calculate_position(angle, radius);
                    positions.insert(planet.name.clone(), (x, y));
                }
            }
        }
        
        positions
    }

    // Planet symbols using Unicode
    fn get_planet_symbol(&self, planet_name: &str) -> &str {
        match planet_name {
            "Sun" => "☉",
            "Moon" => "☽",
            "Mercury" => "☿",
            "Venus" => "♀",
            "Mars" => "♂",
            "Jupiter" => "♃",
            "Saturn" => "♄",
            "Uranus" => "♅",
            "Neptune" => "♆",
            "Pluto" => "♇",
            _ => "?"
        }
    }

    // Zodiac signs
    fn get_zodiac_signs(&self) -> [&str; 12] {
        ["♈︎", "♉︎", "♊︎", "♋︎", "♌︎", "♍︎", "♎︎", "♏︎", "♐︎", "♑︎", "♒︎", "♓︎"]
    }

    // Convert longitude to angle (0° Aries = top of chart)
    fn longitude_to_angle(&self, longitude: f64) -> f64 {
        // Subtract 90 degrees to make 0° Aries at top
        (longitude - 90.0) * PI / 180.0
    }

    // Calculate position on circle
    fn calculate_position(&self, angle: f64, radius: f64) -> (f64, f64) {
        let x = self.center_x + radius * angle.cos();
        let y = self.center_y + radius * angle.sin();
        (x, y)
    }

    // Create SVG document with background
    pub fn create_svg_document(&self) -> Result<Document, String> {
        let styles = get_styles().ok_or("Chart styles not initialized. chart_styles.json is required.")?;
        let background_color = styles.get_chart_color("background");
        
        Ok(Document::new()
            .set("viewBox", (0, 0, self.width as i32, self.height as i32))
            .set("width", self.width)
            .set("height", self.height)
            .set("style", format!("background-color: {}", background_color))
            .add(
                Rectangle::new()
                    .set("width", "100%")
                    .set("height", "100%")
                    .set("fill", background_color)
            ))
    }

    // Draw outer circle and zodiac wheel background
    pub fn draw_chart_wheel_background(&self, doc: Document) -> Result<Document, String> {
        let styles = get_styles().ok_or("Chart styles not initialized. chart_styles.json is required.")?;
        
        // Outer circle
        let outer_circle = Circle::new()
            .set("cx", self.center_x)
            .set("cy", self.center_y)
            .set("r", self.outer_radius)
            .set("fill", styles.get_chart_color("wheel_background"))
            .set("stroke", styles.get_chart_color("chart_wheel_line"))
            .set("stroke-width", 2);

        // Inner circle
        let inner_circle = Circle::new()
            .set("cx", self.center_x)
            .set("cy", self.center_y)
            .set("r", INNER_RADIUS)
            .set("fill", "none")
            .set("stroke", styles.get_chart_color("chart_wheel_line"))
            .set("stroke-width", 1);

        Ok(doc.add(outer_circle).add(inner_circle))
    }

    // Draw zodiac division lines with opacity
    pub fn draw_zodiac_divisions(&self, doc: Document) -> Result<Document, String> {
        let styles = get_styles().ok_or("Chart styles not initialized. chart_styles.json is required.")?;
        let mut doc = doc;

        // Draw zodiac divisions with 50% opacity
        for i in 0..12 {
            let angle = (i as f64 * 30.0) * PI / 180.0 - PI / 2.0;
            
            // Division lines with opacity
            let (x1, y1) = self.calculate_position(angle, INNER_RADIUS);
            let (x2, y2) = self.calculate_position(angle, self.outer_radius);
            
            let line = Line::new()
                .set("x1", x1)
                .set("y1", y1)
                .set("x2", x2)
                .set("y2", y2)
                .set("stroke", styles.get_chart_color("chart_wheel_line"))
                .set("stroke-width", 1)
                .set("opacity", 0.5);
            
            doc = doc.add(line);
        }

        Ok(doc)
    }

    // Draw zodiac signs text
    pub fn draw_zodiac_signs(&self, doc: Document) -> Result<Document, String> {
        let styles = get_styles().ok_or("Chart styles not initialized. chart_styles.json is required.")?;
        let mut doc = doc;
        let signs = self.get_zodiac_signs();

        for i in 0..12 {
            let angle = (i as f64 * 30.0) * PI / 180.0 - PI / 2.0;
            
            // Zodiac signs
            let sign_angle = angle + (15.0 * PI / 180.0);
            let sign_radius = (INNER_RADIUS + self.outer_radius) / 2.0;
            let (sign_x, sign_y) = self.calculate_position(sign_angle, sign_radius);
            
            let sign_text = Text::new()
                .set("x", sign_x)
                .set("y", sign_y)
                .set("text-anchor", "middle")
                .set("dominant-baseline", "central")
                .set("fill", styles.get_chart_color("chart_text_color"))
                .set("font-family", "serif")
                .set("font-size", 18)
                .add(TextNode::new(signs[i]));
            
            doc = doc.add(sign_text);
        }

        Ok(doc)
    }

    // Draw houses
    pub fn draw_houses(&self, doc: Document, houses: &[HouseInfo]) -> Result<Document, String> {
        let styles = get_styles().ok_or("Chart styles not initialized. chart_styles.json is required.")?;
        let mut doc = doc;

        for house in houses {
            let angle = self.longitude_to_angle(house.longitude);
            
            // House cusp lines with opacity
            let (x1, y1) = (self.center_x, self.center_y);
            let (x2, y2) = self.calculate_position(angle, INNER_RADIUS);
            
            let line = Line::new()
                .set("x1", x1)
                .set("y1", y1)
                .set("x2", x2)
                .set("y2", y2)
                .set("stroke", styles.get_chart_color("chart_wheel_line"))
                .set("stroke-width", 1)
                .set("opacity", 0.5);
            
            doc = doc.add(line);

            // House numbers
            let number_radius = INNER_RADIUS * 0.8;
            let next_house_angle = angle + (15.0 * PI / 180.0);
            let (num_x, num_y) = self.calculate_position(next_house_angle, number_radius);
            
            let house_text = Text::new()
                .set("x", num_x)
                .set("y", num_y)
                .set("text-anchor", "middle")
                .set("dominant-baseline", "central")
                .set("fill", styles.get_chart_color("chart_text_color"))
                .set("font-family", "sans-serif")
                .set("font-size", 12)
                .add(TextNode::new(house.number.to_string()));
            
            doc = doc.add(house_text);
        }

        Ok(doc)
    }

    // Draw planets with borders and degrees using radial positioning
    pub fn draw_planets(&self, doc: Document, planets: &[PlanetInfo], border_type: &str) -> Result<Document, String> {
        let styles = get_styles().ok_or("Chart styles not initialized. chart_styles.json is required.")?;
        let mut doc = doc;
        let positions = self.calculate_planet_positions(planets);

        for planet in planets {
            let (x, y) = positions.get(&planet.name).cloned().unwrap_or((self.center_x, self.center_y));
            
            // Planet border
            let border_color = match border_type {
                "chart1" => styles.get_chart_color("chart1_planet_border"),
                "chart2" => styles.get_chart_color("chart2_planet_border"),
                "transit" => styles.get_chart_color("transit_planet_border"),
                _ => styles.get_chart_color("chart1_planet_border")
            };

            let border_style = match border_type {
                "transit" => "stroke-dasharray: 3,3",
                _ => ""
            };

            let planet_border = Rectangle::new()
                .set("x", x - 15.0)
                .set("y", y - 15.0)
                .set("width", 30)
                .set("height", 30)
                .set("fill", "none")
                .set("stroke", border_color)
                .set("stroke-width", 1)
                .set("style", border_style);

            if border_type == "chart2" {
                // Circle border for chart2
                let circle_border = Circle::new()
                    .set("cx", x)
                    .set("cy", y)
                    .set("r", 15)
                    .set("fill", "none")
                    .set("stroke", border_color)
                    .set("stroke-width", 1);
                doc = doc.add(circle_border);
            } else {
                doc = doc.add(planet_border);
            }

            // Planet symbol
            let planet_color = styles.get_planet_color(&planet.name);
            let symbol = self.get_planet_symbol(&planet.name);
            
            let planet_text = Text::new()
                .set("x", x)
                .set("y", y - 3.0)
                .set("text-anchor", "middle")
                .set("dominant-baseline", "central")
                .set("fill", planet_color)
                .set("font-family", "serif")
                .set("font-size", 16)
                .add(TextNode::new(symbol));
            
            doc = doc.add(planet_text);

            // Degree information
            let degree = (planet.longitude % 30.0) as i32;
            let minute = ((planet.longitude % 1.0) * 60.0) as i32;
            let degree_text = format!("{}°{:02}'", degree, minute);
            
            let degree_label = Text::new()
                .set("x", x)
                .set("y", y + 8.0)
                .set("text-anchor", "middle")
                .set("dominant-baseline", "central")
                .set("fill", planet_color)
                .set("font-family", "sans-serif")
                .set("font-size", 8)
                .add(TextNode::new(degree_text));
            
            doc = doc.add(degree_label);
        }

        Ok(doc)
    }

    // Draw planets with custom positioning (for synastry charts)
    pub fn draw_planets_with_positions(&self, doc: Document, planets: &[PlanetInfo], positions: &std::collections::HashMap<String, (f64, f64)>, border_type: &str) -> Result<Document, String> {
        let styles = get_styles().ok_or("Chart styles not initialized. chart_styles.json is required.")?;
        let mut doc = doc;

        for planet in planets {
            let (x, y) = positions.get(&planet.name).cloned().unwrap_or((self.center_x, self.center_y));
            
            // Planet border
            let border_color = match border_type {
                "chart1" => styles.get_chart_color("chart1_planet_border"),
                "chart2" => styles.get_chart_color("chart2_planet_border"),
                "transit" => styles.get_chart_color("transit_planet_border"),
                _ => styles.get_chart_color("chart1_planet_border")
            };

            let border_style = match border_type {
                "transit" => "stroke-dasharray: 3,3",
                _ => ""
            };

            let planet_border = Rectangle::new()
                .set("x", x - 15.0)
                .set("y", y - 15.0)
                .set("width", 30)
                .set("height", 30)
                .set("fill", "none")
                .set("stroke", border_color)
                .set("stroke-width", 1)
                .set("style", border_style);

            if border_type == "chart2" {
                // Circle border for chart2
                let circle_border = Circle::new()
                    .set("cx", x)
                    .set("cy", y)
                    .set("r", 15)
                    .set("fill", "none")
                    .set("stroke", border_color)
                    .set("stroke-width", 1);
                doc = doc.add(circle_border);
            } else {
                doc = doc.add(planet_border);
            }

            // Planet symbol
            let planet_color = styles.get_planet_color(&planet.name);
            let symbol = self.get_planet_symbol(&planet.name);
            
            let planet_text = Text::new()
                .set("x", x)
                .set("y", y - 3.0)
                .set("text-anchor", "middle")
                .set("dominant-baseline", "central")
                .set("fill", planet_color)
                .set("font-family", "serif")
                .set("font-size", 16)
                .add(TextNode::new(symbol));
            
            doc = doc.add(planet_text);

            // Degree information
            let degree = (planet.longitude % 30.0) as i32;
            let minute = ((planet.longitude % 1.0) * 60.0) as i32;
            let degree_text = format!("{}°{:02}'", degree, minute);
            
            let degree_label = Text::new()
                .set("x", x)
                .set("y", y + 8.0)
                .set("text-anchor", "middle")
                .set("dominant-baseline", "central")
                .set("fill", planet_color)
                .set("font-family", "sans-serif")
                .set("font-size", 8)
                .add(TextNode::new(degree_text));
            
            doc = doc.add(degree_label);
        }

        Ok(doc)
    }

    // Draw aspects using radial positioning
    pub fn draw_aspects(&self, doc: Document, aspects: &[AspectInfo], planets: &[PlanetInfo], line_style: &str) -> Result<Document, String> {
        let styles = get_styles().ok_or("Chart styles not initialized. chart_styles.json is required.")?;
        let mut doc = doc;

        // Get planet positions using radial positioning
        let planet_positions = self.calculate_planet_positions(planets);

        for aspect in aspects {
            // Strip prefixes from planet names for lookup
            let planet1_name = aspect.planet1.replace("Natal ", "").replace("Transit ", "");
            let planet2_name = aspect.planet2.replace("Natal ", "").replace("Transit ", "");
            
            if let (Some((x1, y1)), Some((x2, y2))) = (
                planet_positions.get(&planet1_name).cloned(),
                planet_positions.get(&planet2_name).cloned()
            ) {
                let color = styles.get_aspect_color(&aspect.aspect);
                
                let stroke_style = match line_style {
                    "dotted" => "stroke-dasharray: 2,2",
                    "long_dotted" => "stroke-dasharray: 5,5",
                    _ => ""
                };

                let line = Line::new()
                    .set("x1", x1)
                    .set("y1", y1)
                    .set("x2", x2)
                    .set("y2", y2)
                    .set("stroke", color)
                    .set("stroke-width", 1)
                    .set("opacity", 0.7)
                    .set("style", stroke_style);
                
                doc = doc.add(line);
            }
        }

        Ok(doc)
    }

    // Draw aspects using custom positioning
    pub fn draw_aspects_with_positions(&self, doc: Document, aspects: &[AspectInfo], _planets: &[PlanetInfo], positions: &std::collections::HashMap<String, (f64, f64)>, line_style: &str) -> Result<Document, String> {
        let styles = get_styles().ok_or("Chart styles not initialized. chart_styles.json is required.")?;
        let mut doc = doc;

        for aspect in aspects {
            // Strip prefixes from planet names for lookup
            let planet1_name = aspect.planet1.replace("Natal ", "").replace("Transit ", "");
            let planet2_name = aspect.planet2.replace("Natal ", "").replace("Transit ", "");
            
            if let (Some((x1, y1)), Some((x2, y2))) = (
                positions.get(&planet1_name).cloned(),
                positions.get(&planet2_name).cloned()
            ) {
                let color = styles.get_aspect_color(&aspect.aspect);
                
                let stroke_style = match line_style {
                    "dotted" => "stroke-dasharray: 2,2",
                    "long_dotted" => "stroke-dasharray: 5,5",
                    _ => ""
                };

                let line = Line::new()
                    .set("x1", x1)
                    .set("y1", y1)
                    .set("x2", x2)
                    .set("y2", y2)
                    .set("stroke", color)
                    .set("stroke-width", 1)
                    .set("opacity", 0.7)
                    .set("style", stroke_style);
                
                doc = doc.add(line);
            }
        }

        Ok(doc)
    }

    // Generate natal chart SVG
    pub fn generate_natal_chart(&self, chart_data: &ChartResponse) -> Result<String, String> {
        let mut doc = self.create_svg_document()?;
        doc = self.draw_chart_wheel_background(doc)?;
        doc = self.draw_zodiac_divisions(doc)?;
        doc = self.draw_zodiac_signs(doc)?;
        doc = self.draw_houses(doc, &chart_data.houses)?;
        
        // Add transit data if present
        if let Some(transit_data) = &chart_data.transit {
            // Calculate positions separately for each chart type
            let natal_positions = self.calculate_planet_positions(&chart_data.planets);
            let mut transit_positions = self.calculate_planet_positions(&transit_data.planets);
            
            // Check for overlaps between natal and transit planets and adjust transit positions if needed
            let mut adjustments_made = std::collections::HashSet::new();
            
            for (transit_planet, transit_pos) in &transit_positions.clone() {
                for (_natal_planet, natal_pos) in &natal_positions {
                    // Calculate distance between positions
                    let dx = transit_pos.0 - natal_pos.0;
                    let dy = transit_pos.1 - natal_pos.1;
                    let distance = (dx * dx + dy * dy).sqrt();
                    
                    // Only adjust if positions are very close (within 25 pixels) to avoid unnecessary moves
                    if distance < 25.0 && !adjustments_made.contains(transit_planet) {
                        // Find the planet's longitude for angle calculation
                        if let Some(planet_info) = transit_data.planets.iter().find(|p| &p.name == transit_planet) {
                            // Add a smaller angular offset (3 degrees) and move outward
                            let adjusted_longitude = planet_info.longitude + 3.0;
                            let adjusted_angle = self.longitude_to_angle(adjusted_longitude);
                            let adjusted_radius = BASE_PLANET_RADIUS + 20.0; // Slightly more for transits
                            let adjusted_pos = self.calculate_position(adjusted_angle, adjusted_radius);
                            
                            transit_positions.insert(transit_planet.clone(), adjusted_pos);
                            adjustments_made.insert(transit_planet.clone());
                        }
                        break;
                    }
                }
            }
            
            // Draw planets using calculated positions
            doc = self.draw_planets_with_positions(doc, &chart_data.planets, &natal_positions, "chart1")?;
            doc = self.draw_planets_with_positions(doc, &transit_data.planets, &transit_positions, "transit")?;
            
            // Draw aspects using calculated positions
            doc = self.draw_aspects_with_positions(doc, &chart_data.aspects, &chart_data.planets, &natal_positions, "solid")?;
            doc = self.draw_aspects_with_positions(doc, &transit_data.aspects, &transit_data.planets, &transit_positions, "dotted")?;
            
            // Draw transit-to-natal aspects
            let styles = get_styles().ok_or("Chart styles not initialized. chart_styles.json is required.")?;
            for aspect in &transit_data.transit_to_natal_aspects {
                // Strip prefixes from planet names for lookup
                let planet1_name = aspect.planet1.replace("Natal ", "").replace("Transit ", "");
                let planet2_name = aspect.planet2.replace("Natal ", "").replace("Transit ", "");
                
                // Determine which positions to use based on aspect planet prefixes
                let pos1 = if aspect.planet1.contains("Natal") {
                    natal_positions.get(&planet1_name).cloned()
                } else {
                    transit_positions.get(&planet1_name).cloned()
                };
                
                let pos2 = if aspect.planet2.contains("Transit") {
                    transit_positions.get(&planet2_name).cloned()
                } else {
                    natal_positions.get(&planet2_name).cloned()
                };
                
                if let (Some((x1, y1)), Some((x2, y2))) = (pos1, pos2) {
                    let color = styles.get_aspect_color(&aspect.aspect);
                    
                    let line = Line::new()
                        .set("x1", x1)
                        .set("y1", y1)
                        .set("x2", x2)
                        .set("y2", y2)
                        .set("stroke", color)
                        .set("stroke-width", 1)
                        .set("opacity", 0.7)
                        .set("style", "stroke-dasharray: 2,2");
                    
                    doc = doc.add(line);
                }
            }
        } else {
            // No transits - use regular positioning
            doc = self.draw_planets(doc, &chart_data.planets, "chart1")?;
            doc = self.draw_aspects(doc, &chart_data.aspects, &chart_data.planets, "solid")?;
        }

        Ok(doc.to_string())
    }

    // Generate synastry chart SVG
    pub fn generate_synastry_chart(&self, synastry_data: &SynastryResponse) -> Result<String, String> {
        let mut doc = self.create_svg_document()?;
        doc = self.draw_chart_wheel_background(doc)?;
        doc = self.draw_zodiac_divisions(doc)?;
        doc = self.draw_zodiac_signs(doc)?;
        doc = self.draw_houses(doc, &synastry_data.chart1.houses)?;
        
        // Calculate positions separately for each chart type
        let chart1_positions = self.calculate_planet_positions(&synastry_data.chart1.planets);
        let mut chart2_positions = self.calculate_planet_positions(&synastry_data.chart2.planets);
        
        // Check for overlaps between the two charts and adjust chart2 positions if needed (more conservative)
        let mut adjustments_made = std::collections::HashSet::new();
        
        for (chart2_planet, chart2_pos) in &chart2_positions.clone() {
            for (_chart1_planet, chart1_pos) in &chart1_positions {
                // Calculate distance between positions
                let dx = chart2_pos.0 - chart1_pos.0;
                let dy = chart2_pos.1 - chart1_pos.1;
                let distance = (dx * dx + dy * dy).sqrt();
                
                // Only adjust if positions are very close (within 25 pixels) to avoid unnecessary moves
                if distance < 25.0 && !adjustments_made.contains(chart2_planet) {
                    // Find the planet's longitude for angle calculation
                    if let Some(planet_info) = synastry_data.chart2.planets.iter().find(|p| &p.name == chart2_planet) {
                        // Add a smaller angular offset (3 degrees) and move slightly outward
                        let adjusted_longitude = planet_info.longitude + 3.0;
                        let adjusted_angle = self.longitude_to_angle(adjusted_longitude);
                        let adjusted_radius = BASE_PLANET_RADIUS + 15.0; // Smaller adjustment
                        let adjusted_pos = self.calculate_position(adjusted_angle, adjusted_radius);
                        
                        chart2_positions.insert(chart2_planet.clone(), adjusted_pos);
                        adjustments_made.insert(chart2_planet.clone());
                    }
                    break;
                }
            }
        }
        
        // Draw planets using the calculated positions
        doc = self.draw_planets_with_positions(doc, &synastry_data.chart1.planets, &chart1_positions, "chart1")?;
        doc = self.draw_planets_with_positions(doc, &synastry_data.chart2.planets, &chart2_positions, "chart2")?;
        
        // Draw aspects for each chart separately
        doc = self.draw_aspects_with_positions(doc, &synastry_data.chart1.aspects, &synastry_data.chart1.planets, &chart1_positions, "solid")?;
        doc = self.draw_aspects_with_positions(doc, &synastry_data.chart2.aspects, &synastry_data.chart2.planets, &chart2_positions, "solid")?;
        
        // Draw synastry aspects between charts
        let styles = get_styles().ok_or("Chart styles not initialized. chart_styles.json is required.")?;
        for aspect in &synastry_data.synastries {
            if let (Some((x1, y1)), Some((x2, y2))) = (
                chart1_positions.get(&aspect.person1).cloned(),
                chart2_positions.get(&aspect.person2).cloned()
            ) {
                let color = styles.get_aspect_color(&aspect.aspect);
                
                let line = Line::new()
                    .set("x1", x1)
                    .set("y1", y1)
                    .set("x2", x2)
                    .set("y2", y2)
                    .set("stroke", color)
                    .set("stroke-width", 1)
                    .set("opacity", 0.7)
                    .set("style", "stroke-dasharray: 5,5");
                
                doc = doc.add(line);
            }
        }

        Ok(doc.to_string())
    }

    // Generate transit chart SVG
    pub fn generate_transit_chart(&self, transit_data: &TransitResponse) -> Result<String, String> {
        let mut doc = self.create_svg_document()?;
        doc = self.draw_chart_wheel_background(doc)?;
        doc = self.draw_zodiac_divisions(doc)?;
        doc = self.draw_zodiac_signs(doc)?;
        doc = self.draw_houses(doc, &transit_data.houses)?;
        
        // Calculate positions separately for each chart type
        let natal_positions = self.calculate_planet_positions(&transit_data.natal_planets);
        let mut transit_positions = self.calculate_planet_positions(&transit_data.transit_planets);
        
        // Check for overlaps between natal and transit planets and adjust transit positions if needed
        let mut adjustments_made = std::collections::HashSet::new();
        
        for (transit_planet, transit_pos) in &transit_positions.clone() {
            for (_natal_planet, natal_pos) in &natal_positions {
                // Calculate distance between positions
                let dx = transit_pos.0 - natal_pos.0;
                let dy = transit_pos.1 - natal_pos.1;
                let distance = (dx * dx + dy * dy).sqrt();
                
                // Only adjust if positions are very close (within 25 pixels) to avoid unnecessary moves
                if distance < 25.0 && !adjustments_made.contains(transit_planet) {
                    // Find the planet's longitude for angle calculation
                    if let Some(planet_info) = transit_data.transit_planets.iter().find(|p| &p.name == transit_planet) {
                        // Add a smaller angular offset (3 degrees) and move outward
                        let adjusted_longitude = planet_info.longitude + 3.0;
                        let adjusted_angle = self.longitude_to_angle(adjusted_longitude);
                        let adjusted_radius = BASE_PLANET_RADIUS + 20.0; // Slightly more for transits
                        let adjusted_pos = self.calculate_position(adjusted_angle, adjusted_radius);
                        
                        transit_positions.insert(transit_planet.clone(), adjusted_pos);
                        adjustments_made.insert(transit_planet.clone());
                    }
                    break;
                }
            }
        }
        
        // Draw planets using calculated positions
        doc = self.draw_planets_with_positions(doc, &transit_data.natal_planets, &natal_positions, "chart1")?;
        doc = self.draw_planets_with_positions(doc, &transit_data.transit_planets, &transit_positions, "transit")?;
        
        // Draw aspects using calculated positions
        doc = self.draw_aspects_with_positions(doc, &transit_data.natal_aspects, &transit_data.natal_planets, &natal_positions, "solid")?;
        doc = self.draw_aspects_with_positions(doc, &transit_data.transit_aspects, &transit_data.transit_planets, &transit_positions, "dotted")?;

        Ok(doc.to_string())
    }
} 
