use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use crate::utils::{error::AppError, validators};
use crate::utils::error::Result;

// Component library definitions for M2.1
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentLibrary {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub categories: Vec<ComponentCategory>,
    pub components: HashMap<String, ComponentTemplate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentCategory {
    pub id: String,
    pub name: String,
    pub parent_id: Option<String>,
    pub description: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentTemplate {
    pub id: String,
    pub name: String,
    pub category_id: String,
    pub description: Option<String>,
    pub symbol: ComponentSymbol,
    pub footprint: Option<String>,
    pub parameters: HashMap<String, ParameterTemplate>,
    pub default_properties: HashMap<String, ComponentProperty>,
    pub pins: Vec<PinTemplate>,
    pub keywords: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentSymbol {
    pub width: f64,
    pub height: f64,
    pub draw_commands: Vec<DrawCommand>,
    pub graphics: Option<SymbolGraphics>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolGraphics {
    pub bounds: GraphicsBounds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphicsBounds {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawCommand {
    pub command_type: DrawCommandType,
    pub parameters: Vec<f64>,
    pub style: Option<DrawStyle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DrawCommandType {
    Rectangle,
    Circle,
    Line,
    Arc,
    Text,
    Polygon,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawStyle {
    pub stroke_width: f64,
    pub stroke_color: String,
    pub fill_color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterTemplate {
    pub name: String,
    pub parameter_type: ParameterType,
    pub default_value: serde_json::Value,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub unit: Option<String>,
    pub description: Option<String>,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ParameterType {
    String,
    Number,
    Boolean,
    Choice(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PinTemplate {
    pub id: String,
    pub name: String,
    pub number: String,
    pub x: f64,
    pub y: f64,
    pub pin_type: PinType,
    pub electrical: ElectricalType,
}

// Instance component for schematic
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Component {
    pub id: String,
    pub type_id: String,
    pub library_id: Option<String>,
    pub reference: String,
    pub value: String,
    pub x: f64,
    pub y: f64,
    pub rotation: f64,
    pub mirrored: bool,
    pub properties: HashMap<String, ComponentProperty>,
    pub pins: Vec<Pin>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentProperty {
    pub value: serde_json::Value,
    pub visible: bool,
    pub editable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pin {
    pub id: String,
    pub name: String,
    pub number: String,
    pub x: f64,
    pub y: f64,
    pub pin_type: PinType,
    pub electrical: ElectricalType,
    pub connected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PinType {
    Input,
    Output,
    Bidirectional,
    Power,
    Ground,
    Passive,
    NotConnected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElectricalType {
    pub voltage: Option<f64>,
    pub current: Option<f64>,
    pub impedance: Option<f64>,
}

impl Component {
    pub fn new(type_id: String, x: f64, y: f64) -> Result<Self> {
        if !validators::validate_coordinate(x) || !validators::validate_coordinate(y) {
            return Err(crate::utils::error::AppError::InvalidInput("Invalid coordinates".to_string()));
        }
        
        Ok(Self {
            id: Uuid::new_v4().to_string(),
            type_id: type_id.clone(),
            library_id: None,
            reference: format!("U{}", rand::random::<u16>()),
            value: type_id,
            x,
            y,
            rotation: 0.0,
            mirrored: false,
            properties: HashMap::new(),
            pins: Vec::new(),
        })
    }

    pub fn from_library(library_id: String, component_id: String, x: f64, y: f64) -> Result<Self> {
        let mut component = Self::new(component_id.clone(), x, y)?;
        component.library_id = Some(library_id);
        Ok(component)
    }

    pub fn from_template(template: &ComponentTemplate, library_id: String, x: f64, y: f64) -> Result<Self> {
        if !validators::validate_coordinate(x) || !validators::validate_coordinate(y) {
            return Err(crate::utils::error::AppError::InvalidInput("Invalid coordinates".to_string()));
        }
        
        let component = Self {
            id: Uuid::new_v4().to_string(),
            type_id: template.id.clone(),
            library_id: Some(library_id),
            reference: format!("U{}", rand::random::<u16>()),
            value: template.name.clone(),
            x,
            y,
            rotation: 0.0,
            mirrored: false,
            properties: template.default_properties.clone(),
            pins: template.pins.iter().map(|pt| Pin {
                id: pt.id.clone(),
                name: pt.name.clone(),
                number: pt.number.clone(),
                x: pt.x,
                y: pt.y,
                pin_type: pt.pin_type.clone(),
                electrical: pt.electrical.clone(),
                connected: false,
            }).collect(),
        };
        
        Ok(component)
    }

    pub fn set_position(&mut self, x: f64, y: f64) -> Result<()> {
        if !validators::validate_coordinate(x) || !validators::validate_coordinate(y) {
            return Err(crate::utils::error::AppError::InvalidInput("Invalid coordinates".to_string()));
        }
        self.x = x;
        self.y = y;
        Ok(())
    }

    pub fn rotate(&mut self, angle: f64) {
        self.rotation = (self.rotation + angle) % 360.0;
        // Update pin positions based on rotation
        self.update_pin_positions();
    }

    pub fn mirror(&mut self) {
        self.mirrored = !self.mirrored;
        self.update_pin_positions();
    }

    pub fn set_property(&mut self, key: String, value: serde_json::Value, visible: bool) {
        self.properties.insert(
            key,
            ComponentProperty {
                value,
                visible,
                editable: true,
            },
        );
    }

    pub fn get_property(&self, key: &str) -> Option<&ComponentProperty> {
        self.properties.get(key)
    }

    fn update_pin_positions(&mut self) {
        // Apply rotation and mirroring transformations to pin positions
        let cos = self.rotation.to_radians().cos();
        let sin = self.rotation.to_radians().sin();
        
        for pin in &mut self.pins {
            let mut x = pin.x;
            let y = pin.y;
            
            if self.mirrored {
                x = -x;
            }
            
            // Apply rotation
            let new_x = x * cos - y * sin;
            let new_y = x * sin + y * cos;
            
            pin.x = new_x;
            pin.y = new_y;
        }
    }

    pub fn get_bounding_box(&self) -> (f64, f64, f64, f64) {
        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;
        let mut max_x = f64::MIN;
        let mut max_y = f64::MIN;

        for pin in &self.pins {
            min_x = min_x.min(self.x + pin.x);
            min_y = min_y.min(self.y + pin.y);
            max_x = max_x.max(self.x + pin.x);
            max_y = max_y.max(self.y + pin.y);
        }

        // Add some padding for the component body
        let padding = 20.0;
        (
            min_x - padding,
            min_y - padding,
            max_x + padding,
            max_y + padding,
        )
    }
}

// Component Library implementation
impl ComponentLibrary {
    pub fn new(name: String) -> Result<Self> {
        validators::validate_string_length_range(&name, 1, 100)?;
        
        Ok(Self {
            id: Uuid::new_v4().to_string(),
            name,
            version: "1.0.0".to_string(),
            description: None,
            categories: Vec::new(),
            components: HashMap::new(),
        })
    }

    pub fn add_category(&mut self, category: ComponentCategory) -> Result<()> {
        validators::validate_string_length_range(&category.name, 1, 50)?;
        
        if self.categories.iter().any(|c| c.id == category.id) {
            return Err(AppError::InvalidInput("Category already exists".to_string()));
        }
        
        self.categories.push(category);
        Ok(())
    }

    pub fn add_component_template(&mut self, template: ComponentTemplate) -> Result<()> {
        validators::validate_string_length_range(&template.name, 1, 100)?;
        
        if !self.categories.iter().any(|c| c.id == template.category_id) {
            return Err(AppError::InvalidInput("Category does not exist".to_string()));
        }
        
        self.components.insert(template.id.clone(), template);
        Ok(())
    }

    pub fn get_component_template(&self, id: &str) -> Option<&ComponentTemplate> {
        self.components.get(id)
    }

    pub fn get_components_by_category(&self, category_id: &str) -> Vec<&ComponentTemplate> {
        self.components.values()
            .filter(|c| c.category_id == category_id)
            .collect()
    }

    pub fn search_components(&self, query: &str) -> Vec<&ComponentTemplate> {
        let query_lower = query.to_lowercase();
        self.components.values()
            .filter(|c| {
                c.name.to_lowercase().contains(&query_lower) ||
                c.keywords.iter().any(|k| k.to_lowercase().contains(&query_lower)) ||
                c.description.as_ref().map_or(false, |d| d.to_lowercase().contains(&query_lower))
            })
            .collect()
    }
}

impl ComponentTemplate {
    pub fn new(name: String, category_id: String, symbol: ComponentSymbol) -> Result<Self> {
        validators::validate_string_length_range(&name, 1, 100)?;
        
        Ok(Self {
            id: Uuid::new_v4().to_string(),
            name,
            category_id,
            description: None,
            symbol,
            footprint: None,
            parameters: HashMap::new(),
            default_properties: HashMap::new(),
            pins: Vec::new(),
            keywords: Vec::new(),
        })
    }

    pub fn add_parameter(&mut self, name: String, param: ParameterTemplate) -> Result<()> {
        validators::validate_string_length_range(&name, 1, 50)?;
        self.parameters.insert(name, param);
        Ok(())
    }

    pub fn add_pin(&mut self, pin: PinTemplate) -> Result<()> {
        if self.pins.iter().any(|p| p.id == pin.id || p.number == pin.number) {
            return Err(AppError::InvalidInput("Pin already exists".to_string()));
        }
        self.pins.push(pin);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_creation() {
        let component = Component::new("resistor".to_string(), 100.0, 200.0).unwrap();
        assert_eq!(component.type_id, "resistor");
        assert_eq!(component.x, 100.0);
        assert_eq!(component.y, 200.0);
        assert_eq!(component.rotation, 0.0);
    }

    #[test]
    fn test_component_rotation() {
        let mut component = Component::new("resistor".to_string(), 0.0, 0.0).unwrap();
        component.rotate(90.0);
        assert_eq!(component.rotation, 90.0);
        component.rotate(270.0);
        assert_eq!(component.rotation, 0.0);
    }

    #[test]
    fn test_component_properties() {
        let mut component = Component::new("resistor".to_string(), 0.0, 0.0).unwrap();
        component.set_property(
            "resistance".to_string(),
            serde_json::json!("10k"),
            true,
        );
        
        let prop = component.get_property("resistance");
        assert!(prop.is_some());
        assert_eq!(prop.unwrap().value, serde_json::json!("10k"));
    }
}