class ChartStyles:
    def __init__(self, style_options=None):
        self.options = style_options or {}
        
        # Default color schemes
        self.default_colors = {
            'planet_colors': {
                'Sun': '#FF6B35',
                'Moon': '#4ECDC4',
                'Mercury': '#45B7D1',
                'Venus': '#96CEB4',
                'Mars': '#FFEAA7',
                'Jupiter': '#DDA0DD',
                'Saturn': '#98D8C8',
                'Uranus': '#6C5CE7',
                'Neptune': '#74B9FF',
                'Pluto': '#A29BFE'
            },
            'aspect_colors': {
                'Conjunction': '#FF6B6B',
                'Opposition': '#4ECDC4',
                'Trine': '#45B7D1',
                'Square': '#FFA07A',
                'Sextile': '#98D8E8'
            }
        }
    
    def get(self, key, default=None):
        """Get style option with fallback to default"""
        return self.options.get(key, default)
    
    def get_planet_color(self, planet_name):
        """Get color for a specific planet"""
        custom_colors = self.options.get('planet_colors', {})
        return custom_colors.get(planet_name, self.default_colors['planet_colors'].get(planet_name, '#333'))
    
    def get_aspect_color(self, aspect_name):
        """Get color for a specific aspect"""
        custom_colors = self.options.get('aspect_colors', {})
        return custom_colors.get(aspect_name, self.default_colors['aspect_colors'].get(aspect_name, '#666'))
