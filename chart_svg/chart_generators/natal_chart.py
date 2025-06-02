import math
from .base_chart import BaseChartGenerator

class NatalChartGenerator(BaseChartGenerator):
    def generate(self):
        """Generate natal chart SVG"""
        elements = [
            self.create_svg_header(),
            self.draw_outer_circle(),
            self.draw_zodiac_wheel(),
            self.draw_houses(),
            self.draw_planets(self.data['planets']),
            self.draw_aspects(self.data.get('aspects', []), self.data['planets']),
            '</svg>'
        ]
        
        return '\n'.join(elements)
