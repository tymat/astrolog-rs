class SVGUtils:
    @staticmethod
    def create_circle(cx, cy, r, css_class='', fill='none', stroke='black'):
        """Create SVG circle element"""
        return f'<circle cx="{cx}" cy="{cy}" r="{r}" class="{css_class}" fill="{fill}" stroke="{stroke}"/>'
    
    @staticmethod
    def create_line(x1, y1, x2, y2, css_class='', stroke='black'):
        """Create SVG line element"""
        return f'<line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" class="{css_class}" stroke="{stroke}"/>'
    
    @staticmethod
    def create_text(x, y, text, css_class='', fill='black'):
        """Create SVG text element"""
        return f'<text x="{x}" y="{y}" class="{css_class}" fill="{fill}">{text}</text>'
