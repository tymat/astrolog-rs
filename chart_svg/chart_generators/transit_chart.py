import math
from .base_chart import BaseChartGenerator

class TransitChartGenerator(BaseChartGenerator):
    def generate(self):
        """Generate transit chart SVG"""
        natal_planets = self.data['natal_planets']
        transit_planets = self.data['transit_planets']
        
        elements = [
            self.create_svg_header(),
            self.draw_outer_circle(),
            self.draw_zodiac_wheel(),
            self.draw_houses(),
            self.draw_transit_planets(natal_planets, transit_planets),
            self.draw_transit_aspects(),
            self.draw_transit_legend(),
            '</svg>'
        ]
        
        return '\n'.join(filter(None, elements))
    
    def create_css_styles(self):
        """Create CSS styles for the transit chart"""
        base_styles = super().create_css_styles()
        transit_styles = f'''
        .natal-indicator {{ fill: none; stroke: {self.style.get('natal_border_color', '#8B4513')}; stroke-width: 2; }}
        .transit-indicator {{ fill: none; stroke: {self.style.get('transit_border_color', '#4169E1')}; stroke-width: 2; stroke-dasharray: 4,2; }}
        .legend-text {{ font-family: sans-serif; font-size: 12px; text-anchor: start; dominant-baseline: central; }}
        .legend-background {{ fill: rgba(255, 255, 255, 0.9); stroke: #333; stroke-width: 1; }}
        '''
        return base_styles + transit_styles

    def draw_transit_planets(self, natal_planets, transit_planets):
        """Draw both natal and transit planets with proper ordering"""
        elements = []
        
        # Calculate positions for both sets (store for later use in aspects)
        self.natal_positions = self.calculate_planet_positions(natal_planets, base_radius_factor=0.2)
        self.transit_positions = self.calculate_planet_positions(transit_planets, base_radius_factor=0.5)
        
        # Draw Natal planets (inner) with solid circles
        for planet in natal_planets:
            if planet['name'] not in self.natal_positions:
                continue
                
            x, y, radius = self.natal_positions[planet['name']]
            
            # Draw solid circle indicator for Natal
            circle_radius = 12
            elements.append(f'<circle cx="{x}" cy="{y}" r="{circle_radius}" class="natal-indicator"/>')
            
            # Planet symbol with individual planet color
            symbol = self.planet_symbols.get(planet['name'], planet['name'][:2])
            planet_color = self.style.get_planet_color(planet['name'])
            elements.append(f'<text x="{x}" y="{y}" class="planet-symbol" fill="{planet_color}">{symbol}</text>')
            
            # Degree information
            degree = int(planet['longitude'] % 30)
            minute = int((planet['longitude'] % 1) * 60)
            
            degree_x = x + 20
            degree_y = y + 5
            elements.append(f'<text x="{degree_x}" y="{degree_y}" class="degree-text" fill="{planet_color}">{degree}°{minute:02d}</text>')
        
        # Draw Transit planets (outer) with dashed circles
        for planet in transit_planets:
            if planet['name'] not in self.transit_positions:
                continue
                
            x, y, radius = self.transit_positions[planet['name']]
            
            # Draw dashed circle indicator for Transit
            circle_radius = 12
            elements.append(f'<circle cx="{x}" cy="{y}" r="{circle_radius}" class="transit-indicator"/>')
            
            # Planet symbol with individual planet color
            symbol = self.planet_symbols.get(planet['name'], planet['name'][:2])
            planet_color = self.style.get_planet_color(planet['name'])
            elements.append(f'<text x="{x}" y="{y}" class="planet-symbol" fill="{planet_color}">{symbol}</text>')
            
            # Degree information
            degree = int(planet['longitude'] % 30)
            minute = int((planet['longitude'] % 1) * 60)
            
            degree_x = x + 20
            degree_y = y + 5
            elements.append(f'<text x="{degree_x}" y="{degree_y}" class="degree-text" fill="{planet_color}">{degree}°{minute:02d}</text>')
        
        return '\n'.join(elements)
    
    def draw_transit_legend(self):
        """Draw a legend explaining the transit chart indicators"""
        legend_x = 20
        legend_y = 20
        legend_width = 200
        legend_height = 80
        
        elements = []
        
        # Legend background
        elements.append(f'<rect x="{legend_x}" y="{legend_y}" width="{legend_width}" height="{legend_height}" class="legend-background"/>')
        
        # Natal indicator and label
        circle_x = legend_x + 10
        circle_y = legend_y + 25
        circle_radius = 8
        elements.append(f'<circle cx="{circle_x + circle_radius}" cy="{circle_y}" r="{circle_radius}" class="natal-indicator"/>')
        
        text_x = circle_x + circle_radius * 2 + 10
        text_y = circle_y
        natal_label = self.style.get('natal_label', 'Natal (Inner)')
        elements.append(f'<text x="{text_x}" y="{text_y}" class="legend-text">{natal_label}</text>')
        
        # Transit indicator and label
        circle_x = legend_x + 10
        circle_y = legend_y + 55
        circle_radius = 8
        elements.append(f'<circle cx="{circle_x + circle_radius}" cy="{circle_y}" r="{circle_radius}" class="transit-indicator"/>')
        
        text_x = circle_x + circle_radius * 2 + 10
        text_y = circle_y
        transit_label = self.style.get('transit_label', 'Transit (Outer)')
        elements.append(f'<text x="{text_x}" y="{text_y}" class="legend-text">{transit_label}</text>')
        
        return '\n'.join(elements)
    
    def draw_transit_aspects(self):
        """Draw transit to natal aspects with proper positioning"""
        elements = []
        
        # Use the positions calculated in draw_transit_planets
        if not hasattr(self, 'natal_positions') or not hasattr(self, 'transit_positions'):
            # Fallback: recalculate if not available
            self.natal_positions = self.calculate_planet_positions(self.data['natal_planets'], base_radius_factor=0.2)
            self.transit_positions = self.calculate_planet_positions(self.data['transit_planets'], base_radius_factor=0.5)
        
        # Create position lookups
        natal_pos_lookup = {name: (x, y) for name, (x, y, r) in self.natal_positions.items()}
        transit_pos_lookup = {name: (x, y) for name, (x, y, r) in self.transit_positions.items()}
        
        # Draw transit to natal aspects if available in the transit object
        if 'transit' in self.data and 'transit_to_natal_aspects' in self.data['transit']:
            for aspect in self.data['transit']['transit_to_natal_aspects']:
                natal_planet = aspect['planet1'].replace('Natal ', '')
                transit_planet = aspect['planet2'].replace('Transit ', '')
                
                if natal_planet in natal_pos_lookup and transit_planet in transit_pos_lookup:
                    x1, y1 = natal_pos_lookup[natal_planet]
                    x2, y2 = transit_pos_lookup[transit_planet]
                    
                    color = self.style.get_aspect_color(aspect['aspect'])
                    elements.append(f'<line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" class="aspect-line" stroke="{color}" opacity="0.6"/>')
        
        # Also check for aspects at the top level (fallback)
        for aspect_key in ['transit_aspects', 'aspects']:
            if aspect_key in self.data:
                for aspect in self.data[aspect_key]:
                    # Try to identify if this is a transit-to-natal aspect
                    planet1_name = aspect['planet1'].replace('Natal ', '').replace('Transit ', '')
                    planet2_name = aspect['planet2'].replace('Natal ', '').replace('Transit ', '')
                    
                    # Check if one is natal and one is transit
                    if (planet1_name in natal_pos_lookup and planet2_name in transit_pos_lookup):
                        x1, y1 = natal_pos_lookup[planet1_name]
                        x2, y2 = transit_pos_lookup[planet2_name]
                        
                        color = self.style.get_aspect_color(aspect['aspect'])
                        elements.append(f'<line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" class="aspect-line" stroke="{color}" opacity="0.6"/>')
                    elif (planet1_name in transit_pos_lookup and planet2_name in natal_pos_lookup):
                        x1, y1 = transit_pos_lookup[planet1_name]
                        x2, y2 = natal_pos_lookup[planet2_name]
                        
                        color = self.style.get_aspect_color(aspect['aspect'])
                        elements.append(f'<line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" class="aspect-line" stroke="{color}" opacity="0.6"/>')
        
        return '\n'.join(elements)
