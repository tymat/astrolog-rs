use crate::core::types::{AstrologError, Chart};

/// Save a chart to a file
pub fn save_chart(_chart: &Chart, _filename: &str) -> Result<(), AstrologError> {
    Err(AstrologError::NotImplemented { 
        message: "Chart saving not yet implemented".into() 
    })
}

/// Load a chart from a file
pub fn load_chart(_filename: &str) -> Result<Chart, AstrologError> {
    Err(AstrologError::NotImplemented { 
        message: "Chart loading not yet implemented".into() 
    })
} 