import math
from utils.math_utils import MathUtils
from utils.svg_utils import SVGUtils
from styles.chart_styles import ChartStyles

class BaseChartGenerator:
    def __init__(self, data, style_options=None):
        self.data = data
        self.style = ChartStyles(style_options or {})
        self.math = MathUtils()
        self.svg = SVGUtils()
        
        # Chart dimensions
        self.width = self.style.get('width', 800)
        self.height = self.style.get('height', 800)
        self.center_x = self.width / 2
        self.center_y = self.height / 2
        self.radius = min(self.width, self.height) * 0.4
        
        # Planet order from center to edge (traditional order)
        self.planet_order = [
            'Sun', 'Moon', 'Mercury', 'Venus', 'Mars', 
            'Jupiter', 'Saturn', 'Uranus', 'Neptune', 'Pluto'
        ]
        
        # Zodiac signs
        self.zodiac_signs = [
            'Aries', 'Taurus', 'Gemini', 'Cancer', 'Leo', 'Virgo',
            'Libra', 'Scorpio', 'Sagittarius', 'Capricorn', 'Aquarius', 'Pisces'
        ]
        
        # Planet symbols
        self.planet_symbols = {
            'Sun': '☉', 'Moon': '☽', 'Mercury': '☿', 'Venus': '♀', 'Mars': '♂',
            'Jupiter': '♃', 'Saturn': '♄', 'Uranus': '♅', 'Neptune': '♆', 'Pluto': '♇'
        }
        
        # Aspect symbols
        self.aspect_symbols = {
            'Conjunction': '☌', 'Opposition': '☍', 'Trine': '△', 
            'Square': '□', 'Sextile': '⚹'
        }

    def get_planet_radius(self, planet_name, base_radius_factor=0.3, spacing=0.04):
        """Calculate radius for a planet based on its order from center"""
        if planet_name not in self.planet_order:
            # For unknown planets, place them at the outer edge
            planet_index = len(self.planet_order)
        else:
            planet_index = self.planet_order.index(planet_name)
        
        radius_factor = base_radius_factor + (planet_index * spacing)
        return self.radius * radius_factor

    def sort_planets_by_order(self, planets):
        """Sort planets according to their traditional order"""
        def get_planet_order_index(planet):
            name = planet['name']
            if name in self.planet_order:
                return self.planet_order.index(name)
            else:
                return len(self.planet_order)  # Put unknown planets at the end
        
        return sorted(planets, key=get_planet_order_index)

    def group_planets_by_proximity(self, planets, threshold_degrees=5):
        """Group planets that are close together in longitude"""
        sorted_planets = sorted(planets, key=lambda p: p['longitude'])
        groups = []
        current_group = []
        
        for planet in sorted_planets:
            if not current_group:
                current_group = [planet]
            else:
                last_planet = current_group[-1]
                longitude_diff = abs(planet['longitude'] - last_planet['longitude'])
                
                # Handle wrap-around at 0/360 degrees
                if longitude_diff > 180:
                    longitude_diff = 360 - longitude_diff
                
                if longitude_diff <= threshold_degrees:
                    current_group.append(planet)
                else:
                    groups.append(current_group)
                    current_group = [planet]
        
        if current_group:
            groups.append(current_group)
        
        return groups

    def calculate_planet_positions(self, planets, base_radius_factor=0.3):
        """Calculate positions for planets, handling overlaps"""
        planet_groups = self.group_planets_by_proximity(planets)
        positions = {}
        
        for group in planet_groups:
            if len(group) == 1:
                # Single planet - use its natural radius
                planet = group[0]
                radius = self.get_planet_radius(planet['name'], base_radius_factor)
                angle = math.radians(planet['longitude'] - 90)
                
                x = self.center_x + radius * math.cos(angle)
                y = self.center_y + radius * math.sin(angle)
                positions[planet['name']] = (x, y, radius)
            else:
                # Multiple planets close together - arrange them radially
                sorted_group = self.sort_planets_by_order(group)
                
                for i, planet in enumerate(sorted_group):
                    radius = self.get_planet_radius(planet['name'], base_radius_factor)
                    angle = math.radians(planet['longitude'] - 90)
                    
                    x = self.center_x + radius * math.cos(angle)
                    y = self.center_y + radius * math.sin(angle)
                    positions[planet['name']] = (x, y, radius)
        
        return positions

    def generate(self):
        """Override in subclasses"""
        raise NotImplementedError

    def get_metadata(self):
        """Return chart metadata"""
        return {
            'chart_type': self.data.get('chart_type'),
            'generated_at': self.data.get('date'),
            'house_system': self.data.get('house_system'),
            'ayanamsa': self.data.get('ayanamsa')
        }

    def create_svg_header(self):
        """Create SVG header with styling"""
        return f'''<svg width="{self.width}" height="{self.height}" 
                   viewBox="0 0 {self.width} {self.height}" 
                   xmlns="http://www.w3.org/2000/svg">
                   <defs>{self.create_gradients()}</defs>
                   <style>{self.create_css_styles()}</style>'''

    def create_gradients(self):
        """Create gradient definitions"""
        return '''
        <radialGradient id="chartGradient" cx="50%" cy="50%" r="50%">
            <stop offset="0%" stop-color="#ffffff" stop-opacity="1"/>
            <stop offset="100%" stop-color="#f8f9fa" stop-opacity="1"/>
        </radialGradient>
        '''

    def create_css_styles(self):
        """Create CSS styles for the chart"""
        return f'''
        .chart-background {{ fill: url(#chartGradient); stroke: {self.style.get('border_color', '#333')}; stroke-width: 2; }}
        .house-line {{ stroke: {self.style.get('house_line_color', '#ccc')}; stroke-width: 1; fill: none; }}
        .zodiac-line {{ stroke: {self.style.get('zodiac_line_color', '#666')}; stroke-width: 2; fill: none; }}
        .planet-symbol {{ font-family: serif; font-size: {self.style.get('planet_font_size', '16')}px; text-anchor: middle; dominant-baseline: central; }}
        .sign-symbol {{ font-family: serif; font-size: {self.style.get('sign_font_size', '14')}px; text-anchor: middle; dominant-baseline: central; }}
        .aspect-line {{ stroke-width: 1; fill: none; opacity: 0.7; }}
        .degree-text {{ font-family: sans-serif; font-size: 10px; text-anchor: middle; dominant-baseline: central; }}
        '''

    def draw_outer_circle(self):
        """Draw the main chart circle"""
        return f'<circle cx="{self.center_x}" cy="{self.center_y}" r="{self.radius}" class="chart-background"/>'

    def draw_zodiac_wheel(self):
        """Draw the zodiac wheel with 12 signs"""
        elements = []
        
        for i in range(12):
            # Calculate positions for zodiac divisions
            angle1 = math.radians(i * 30 - 90)
            angle2 = math.radians((i + 1) * 30 - 90)
            
            # Draw division lines
            x1 = self.center_x + (self.radius * 0.8) * math.cos(angle1)
            y1 = self.center_y + (self.radius * 0.8) * math.sin(angle1)
            x2 = self.center_x + self.radius * math.cos(angle1)
            y2 = self.center_y + self.radius * math.sin(angle1)
            
            elements.append(f'<line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" class="zodiac-line"/>')
            
            # Add zodiac sign symbols
            sign_angle = math.radians(i * 30 + 15 - 90)
            sign_x = self.center_x + (self.radius * 0.9) * math.cos(sign_angle)
            sign_y = self.center_y + (self.radius * 0.9) * math.sin(sign_angle)
            
            elements.append(f'<text x="{sign_x}" y="{sign_y}" class="sign-symbol">{self.zodiac_signs[i][:3]}</text>')
        
        return '\n'.join(elements)

    def draw_houses(self):
        """Draw house divisions"""
        if 'houses' not in self.data:
            return ''
        
        elements = []
        
        for house in self.data['houses']:
            house_angle = math.radians(house['longitude'] - 90)
            
            # Draw house cusp lines
            x1 = self.center_x
            y1 = self.center_y
            x2 = self.center_x + (self.radius * 0.8) * math.cos(house_angle)
            y2 = self.center_y + (self.radius * 0.8) * math.sin(house_angle)
            
            elements.append(f'<line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" class="house-line"/>')
            
            # Add house numbers
            next_house_angle = house_angle + math.radians(15)
            number_x = self.center_x + (self.radius * 0.75) * math.cos(next_house_angle)
            number_y = self.center_y + (self.radius * 0.75) * math.sin(next_house_angle)
            
            elements.append(f'<text x="{number_x}" y="{number_y}" class="degree-text">{house["number"]}</text>')
        
        return '\n'.join(elements)

    def draw_planets(self, planets, base_radius_factor=0.3):
        """Draw planets on the chart with proper ordering"""
        elements = []
        positions = self.calculate_planet_positions(planets, base_radius_factor)
        
        for planet in planets:
            if planet['name'] not in positions:
                continue
                
            x, y, radius = positions[planet['name']]
            
            # Planet symbol
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

    def draw_aspects(self, aspects, planets, positions=None):
        """Draw aspect lines between planets"""
        elements = []
        
        if positions is None:
            positions = self.calculate_planet_positions(planets)
        
        # Create planet position lookup
        planet_positions = {}
        for planet in planets:
            if planet['name'] in positions:
                x, y, radius = positions[planet['name']]
                planet_positions[planet['name']] = (x, y)
        
        for aspect in aspects:
            planet1_name = aspect['planet1'].replace('Natal ', '').replace('Transit ', '')
            planet2_name = aspect['planet2'].replace('Natal ', '').replace('Transit ', '')
            
            if planet1_name in planet_positions and planet2_name in planet_positions:
                x1, y1 = planet_positions[planet1_name]
                x2, y2 = planet_positions[planet2_name]
                
                color = self.style.get_aspect_color(aspect['aspect'])
                elements.append(f'<line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" class="aspect-line" stroke="{color}"/>')
        
        return '\n'.join(elements)
