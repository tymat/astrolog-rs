import math

class MathUtils:
    @staticmethod
    def degrees_to_radians(degrees):
        """Convert degrees to radians"""
        return math.radians(degrees)
    
    @staticmethod
    def normalize_longitude(longitude):
        """Normalize longitude to 0-360 range"""
        while longitude < 0:
            longitude += 360
        while longitude >= 360:
            longitude -= 360
        return longitude
    
    @staticmethod
    def calculate_aspect_orb(planet1_long, planet2_long):
        """Calculate the orb between two planetary positions"""
        diff = abs(planet1_long - planet2_long)
        if diff > 180:
            diff = 360 - diff
        return diff
