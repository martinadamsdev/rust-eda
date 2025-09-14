use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::utils::{error::AppError, validators};
use crate::utils::error::Result;

const MAX_COMPONENTS: usize = 10000;
const MAX_WIRES: usize = 50000;
const MAX_NETS: usize = 10000;
const MAX_LABELS: usize = 5000;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schematic {
    pub id: String,
    pub name: String,
    pub components: Vec<crate::models::Component>,
    pub wires: Vec<crate::models::Wire>,
    pub nets: Vec<Net>,
    pub labels: Vec<Label>,
    pub metadata: SchematicMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchematicMetadata {
    pub title: Option<String>,
    pub revision: Option<String>,
    pub date: Option<String>,
    pub sheet_size: String,
    pub grid_visible: bool,
    pub grid_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Net {
    pub id: String,
    pub name: String,
    pub pins: Vec<PinConnection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PinConnection {
    pub component_id: String,
    pub pin_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    pub id: String,
    pub text: String,
    pub x: f64,
    pub y: f64,
    pub font_size: u32,
    pub color: String,
}

impl Default for SchematicMetadata {
    fn default() -> Self {
        Self {
            title: None,
            revision: None,
            date: None,
            sheet_size: "A4".to_string(),
            grid_visible: true,
            grid_size: 10,
        }
    }
}

impl Schematic {
    pub fn new(name: String) -> Result<Self> {
        if !validators::validate_string_length(&name, 100) {
            return Err(AppError::InvalidInput("Invalid schematic name".to_string()));
        }
        
        Ok(Self {
            id: Uuid::new_v4().to_string(),
            name,
            components: Vec::with_capacity(100), // Pre-allocate reasonable capacity
            wires: Vec::with_capacity(200),
            nets: Vec::with_capacity(50),
            labels: Vec::with_capacity(20),
            metadata: SchematicMetadata::default(),
        })
    }
    
    
    pub fn component_count(&self) -> usize {
        self.components.len()
    }
    
    pub fn wire_count(&self) -> usize {
        self.wires.len()
    }
    
    pub fn can_add_component(&self) -> bool {
        self.components.len() < MAX_COMPONENTS
    }
    
    pub fn can_add_wire(&self) -> bool {
        self.wires.len() < MAX_WIRES
    }

    pub fn add_component(&mut self, component: crate::models::Component) -> Result<()> {
        if self.components.len() >= MAX_COMPONENTS {
            return Err(AppError::InvalidOperation(
                format!("Maximum component limit ({}) exceeded", MAX_COMPONENTS)
            ));
        }
        
        // Validate component data
        if !validators::validate_coordinate(component.x) || !validators::validate_coordinate(component.y) {
            return Err(AppError::InvalidInput("Invalid component coordinates".to_string()));
        }
        
        self.components.push(component);
        Ok(())
    }

    pub fn add_wire(&mut self, wire: crate::models::Wire) -> Result<()> {
        if self.wires.len() >= MAX_WIRES {
            return Err(AppError::InvalidOperation(
                format!("Maximum wire limit ({}) exceeded", MAX_WIRES)
            ));
        }
        
        // Validate wire points
        for point in &wire.points {
            if !validators::validate_coordinate(point.x) || !validators::validate_coordinate(point.y) {
                return Err(AppError::InvalidInput("Invalid wire coordinates".to_string()));
            }
        }
        
        self.wires.push(wire);
        Ok(())
    }
    
    pub fn add_net(&mut self, net: Net) -> Result<()> {
        if self.nets.len() >= MAX_NETS {
            return Err(AppError::InvalidOperation(
                format!("Maximum net limit ({}) exceeded", MAX_NETS)
            ));
        }
        self.nets.push(net);
        Ok(())
    }
    
    pub fn add_label(&mut self, label: Label) -> Result<()> {
        if self.labels.len() >= MAX_LABELS {
            return Err(AppError::InvalidOperation(
                format!("Maximum label limit ({}) exceeded", MAX_LABELS)
            ));
        }
        self.labels.push(label);
        Ok(())
    }

    pub fn remove_component(&mut self, id: &str) -> Option<crate::models::Component> {
        if let Some(index) = self.components.iter().position(|c| c.id == id) {
            Some(self.components.remove(index))
        } else {
            None
        }
    }

    pub fn remove_wire(&mut self, id: &str) -> Option<crate::models::Wire> {
        if let Some(index) = self.wires.iter().position(|w| w.id == id) {
            Some(self.wires.remove(index))
        } else {
            None
        }
    }

    pub fn get_component(&self, id: &str) -> Option<&crate::models::Component> {
        self.components.iter().find(|c| c.id == id)
    }

    pub fn get_component_mut(&mut self, id: &str) -> Option<&mut crate::models::Component> {
        self.components.iter_mut().find(|c| c.id == id)
    }

    pub fn generate_netlist(&self) -> Vec<Net> {
        // TODO: Implement netlist generation algorithm
        self.nets.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schematic_creation() {
        let schematic = Schematic::new("Main Sheet".to_string()).unwrap();
        assert_eq!(schematic.name, "Main Sheet");
        assert!(schematic.components.is_empty());
        assert!(schematic.wires.is_empty());
    }

    #[test]
    fn test_component_management() {
        use crate::models::Component;
        
        let mut schematic = Schematic::new("Test".to_string()).unwrap();
        let component = Component::new("resistor".to_string(), 100.0, 100.0).unwrap();
        let component_id = component.id.clone();
        
        schematic.add_component(component).unwrap();
        assert_eq!(schematic.components.len(), 1);
        
        let removed = schematic.remove_component(&component_id);
        assert!(removed.is_some());
        assert!(schematic.components.is_empty());
    }
}