import math
from .base_chart import BaseChartGenerator

class SynastryChartGenerator(BaseChartGenerator):
    def generate(self):
        """Generate synastry chart SVG"""
        chart1_planets = self.data['chart1']['planets']
        chart2_planets = self.data['chart2']['planets']
        
        elements = [
            self.create_svg_header(),
            self.draw_outer_circle(),
            self.draw_zodiac_wheel(),
            self.draw_houses() if 'houses' in self.data['chart1'] else '',
            self.draw_synastry_planets(chart1_planets, chart2_planets),
            self.draw_synastry_aspects(),
            self.draw_chart_legend(),
            '</svg>'
        ]
        
        return '\n'.join(filter(None, elements))
    
    def create_css_styles(self):
        """Create CSS styles for the synastry chart"""
        base_styles = super().create_css_styles()
        synastry_styles = f'''
        .chart1-indicator {{ fill: none; stroke: {self.style.get('chart1_border_color', '#FF6B35')}; stroke-width: 2; }}
        .chart2-indicator {{ fill: none; stroke: {self.style.get('chart2_border_color', '#4ECDC4')}; stroke-width: 2; }}
        .legend-text {{ font-family: sans-serif; font-size: 12px; text-anchor: start; dominant-baseline: central; }}
        .legend-background {{ fill: rgba(255, 255, 255, 0.9); stroke: #333; stroke-width: 1; }}
        '''
        return base_styles + synastry_styles

    def draw_synastry_planets(self, chart1_planets, chart2_planets):
        """Draw planets for both charts with proper ordering and indicators"""
        elements = []
        
        # Calculate positions for both charts
        chart1_positions = self.calculate_planet_positions(chart1_planets, base_radius_factor=0.2)
        chart2_positions = self.calculate_planet_positions(chart2_planets, base_radius_factor=0.5)
        
        # Draw Chart 1 planets (inner) with boxes
        for planet in chart1_planets:
            if planet['name'] not in chart1_positions:
                continue
                
            x, y, radius = chart1_positions[planet['name']]
            
            # Draw box indicator for Chart 1
            box_size = 22
            box_x = x - box_size/2
            box_y = y - box_size/2
            
            elements.append(f'<rect x="{box_x}" y="{box_y}" width="{box_size}" height="{box_size}" class="chart1-indicator"/>')
            
            # Planet symbol with individual planet color
            symbol = self.planet_symbols.get(planet['name'], planet['name'][:2])
            planet_color = self.style.get_planet_color(planet['name'])
            elements.append(f'<text x="{x}" y="{y}" class="planet-symbol" fill="{planet_color}">{symbol}</text>')
            
            # Degree information
            degree = int(planet['longitude'] % 30)
            minute = int((planet['longitude'] % 1) * 60)
            
            degree_x = x + 25
            degree_y = y + 5
            elements.append(f'<text x="{degree_x}" y="{degree_y}" class="degree-text" fill="{planet_color}">{degree}°{minute:02d}</text>')
        
        # Draw Chart 2 planets (outer) with circles
        for planet in chart2_planets:
            if planet['name'] not in chart2_positions:
                continue
                
            x, y, radius = chart2_positions[planet['name']]
            
            # Draw circle indicator for Chart 2
            circle_radius = 13
            elements.append(f'<circle cx="{x}" cy="{y}" r="{circle_radius}" class="chart2-indicator"/>')
            
            # Planet symbol with individual planet color
            symbol = self.planet_symbols.get(planet['name'], planet['name'][:2])
            planet_color = self.style.get_planet_color(planet['name'])
            elements.append(f'<text x="{x}" y="{y}" class="planet-symbol" fill="{planet_color}">{symbol}</text>')
            
            # Degree information
            degree = int(planet['longitude'] % 30)
            minute = int((planet['longitude'] % 1) * 60)
            
            degree_x = x + 25
            degree_y = y + 5
            elements.append(f'<text x="{degree_x}" y="{degree_y}" class="degree-text" fill="{planet_color}">{degree}°{minute:02d}</text>')
        
        return '\n'.join(elements)
    
    def draw_chart_legend(self):
        """Draw a legend explaining the chart indicators"""
        legend_x = 20
        legend_y = 20
        legend_width = 200
        legend_height = 80
        
        elements = []
        
        # Legend background
        elements.append(f'<rect x="{legend_x}" y="{legend_y}" width="{legend_width}" height="{legend_height}" class="legend-background"/>')
        
        # Chart 1 indicator and label
        box_x = legend_x + 10
        box_y = legend_y + 20
        box_size = 16
        elements.append(f'<rect x="{box_x}" y="{box_y}" width="{box_size}" height="{box_size}" class="chart1-indicator"/>')
        
        text_x = box_x + box_size + 10
        text_y = box_y + box_size/2
        chart1_label = self.style.get('chart1_label', 'Chart 1 (Inner)')
        elements.append(f'<text x="{text_x}" y="{text_y}" class="legend-text">{chart1_label}</text>')
        
        # Chart 2 indicator and label
        circle_x = legend_x + 10
        circle_y = legend_y + 50
        circle_radius = 8
        elements.append(f'<circle cx="{circle_x + circle_radius}" cy="{circle_y}" r="{circle_radius}" class="chart2-indicator"/>')
        
        text_x = circle_x + circle_radius * 2 + 10
        text_y = circle_y
        chart2_label = self.style.get('chart2_label', 'Chart 2 (Outer)')
        elements.append(f'<text x="{text_x}" y="{text_y}" class="legend-text">{chart2_label}</text>')
        
        return '\n'.join(elements)
    
    def draw_synastry_aspects(self):
        """Draw synastry aspects between the two charts"""
        elements = []
        
        # Get calculated positions
        chart1_positions = self.calculate_planet_positions(self.data['chart1']['planets'], base_radius_factor=0.2)
        chart2_positions = self.calculate_planet_positions(self.data['chart2']['planets'], base_radius_factor=0.5)
        
        # Create position lookups
        chart1_pos_lookup = {name: (x, y) for name, (x, y, r) in chart1_positions.items()}
        chart2_pos_lookup = {name: (x, y) for name, (x, y, r) in chart2_positions.items()}
        
        for synastry in self.data.get('synastries', []):
            person1_planet = synastry['person1']
            person2_planet = synastry['person2']
            
            if person1_planet in chart1_pos_lookup and person2_planet in chart2_pos_lookup:
                x1, y1 = chart1_pos_lookup[person1_planet]
                x2, y2 = chart2_pos_lookup[person2_planet]
                
                color = self.style.get_aspect_color(synastry['aspect'])
                elements.append(f'<line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" class="aspect-line" stroke="{color}" opacity="0.5"/>')
        
        return '\n'.join(elements)
