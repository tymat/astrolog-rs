from flask import Flask, request, jsonify
from flask_cors import CORS
import json
import traceback
from chart_generators.natal_chart import NatalChartGenerator
from chart_generators.synastry_chart import SynastryChartGenerator
from chart_generators.transit_chart import TransitChartGenerator

app = Flask(__name__)
CORS(app)

@app.route('/generate-chart', methods=['POST'])
def generate_chart():
    try:
        data = request.get_json()
        
        if not data:
            return jsonify({'error': 'No data provided'}), 400
        
        chart_type = data.get('chart_type')
        style_options = data.get('style_options', {})
        
        if chart_type == 'natal':
            generator = NatalChartGenerator(data, style_options)
        elif chart_type == 'synastry':
            generator = SynastryChartGenerator(data, style_options)
        elif chart_type == 'transit':
            generator = TransitChartGenerator(data, style_options)
        else:
            return jsonify({'error': f'Unsupported chart type: {chart_type}'}), 400
        
        svg_content = generator.generate()
        
        return jsonify({
            'success': True,
            'chart_type': chart_type,
            'svg': svg_content,
            'metadata': generator.get_metadata()
        })
        
    except Exception as e:
        return jsonify({
            'success': False,
            'error': str(e),
            'traceback': traceback.format_exc()
        }), 500

@app.route('/health', methods=['GET'])
def health_check():
    return jsonify({'status': 'healthy', 'service': 'auspex-imager'})

if __name__ == '__main__':
    app.run(debug=True, host='0.0.0.0', port=4012)
