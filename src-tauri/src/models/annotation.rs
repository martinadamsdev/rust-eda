use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotation {
    pub id: String,
    pub text: String,
    pub x: f64,
    pub y: f64,
    pub font_size: f64,
    pub color: String,
}