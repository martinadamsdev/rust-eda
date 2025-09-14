use crate::models::{Component, Wire, Schematic, Point};
use crate::utils::error::Result;
use std::collections::HashMap;

#[tauri::command]
pub async fn add_component(
    _schematic_id: String,
    component_type: String,
    x: f64,
    y: f64,
) -> Result<Component> {
    crate::utils::validators::validate_coordinate_strict(x)?;
    crate::utils::validators::validate_coordinate_strict(y)?;
    
    let component = Component::new(component_type, x, y)?;
    Ok(component)
}

#[tauri::command]
pub async fn update_component(
    _schematic_id: String,
    _component_id: String,
    _properties: HashMap<String, serde_json::Value>,
) -> Result<()> {
    // TODO: Implement component update logic with state management
    Ok(())
}

#[tauri::command]
pub async fn delete_component(
    _schematic_id: String,
    _component_id: String,
) -> Result<()> {
    // TODO: Implement component deletion logic with state management
    Ok(())
}

#[tauri::command]
pub async fn add_wire(
    _schematic_id: String,
    start_x: f64,
    start_y: f64,
    end_x: f64,
    end_y: f64,
) -> Result<Wire> {
    if !crate::utils::validators::validate_coordinate(start_x) ||
       !crate::utils::validators::validate_coordinate(start_y) ||
       !crate::utils::validators::validate_coordinate(end_x) ||
       !crate::utils::validators::validate_coordinate(end_y) {
        return Err(crate::utils::error::AppError::InvalidInput("Invalid coordinates".to_string()));
    }
    
    let start = Point::new(start_x, start_y);
    let end = Point::new(end_x, end_y);
    let wire = Wire::new(start, end);
    
    Ok(wire)
}

#[tauri::command]
pub async fn delete_wire(
    _schematic_id: String,
    _wire_id: String,
) -> Result<()> {
    // TODO: Implement wire deletion logic with state management
    Ok(())
}

#[tauri::command]
pub async fn move_component(
    _schematic_id: String,
    _component_id: String,
    new_x: f64,
    new_y: f64,
) -> Result<()> {
    if !crate::utils::validators::validate_coordinate(new_x) ||
       !crate::utils::validators::validate_coordinate(new_y) {
        return Err(crate::utils::error::AppError::InvalidInput("Invalid coordinates".to_string()));
    }
    
    // TODO: Implement component movement logic with state management
    Ok(())
}

#[tauri::command]
pub async fn rotate_component(
    _schematic_id: String,
    _component_id: String,
    _angle: f64,
) -> Result<()> {
    // TODO: Implement component rotation logic with state management
    Ok(())
}

#[tauri::command]
pub async fn generate_netlist(schematic: Schematic) -> Result<String> {
    // Generate netlist from schematic
    let _nets = schematic.generate_netlist();
    
    // Convert to SPICE format or other netlist format
    let mut netlist = String::new();
    netlist.push_str("* Generated Netlist\n");
    netlist.push_str(&format!("* Project: {}\n", schematic.name));
    netlist.push('\n');
    
    // Add components
    for component in &schematic.components {
        netlist.push_str(&format!("{} ", component.reference));
        // Add pins/nets connections
        for pin in &component.pins {
            netlist.push_str(&format!("N{} ", pin.id));
        }
        netlist.push_str(&format!("{}\n", component.value));
    }
    
    netlist.push_str("\n.end\n");
    
    Ok(netlist)
}

#[tauri::command]
pub async fn validate_schematic(schematic: Schematic) -> Result<ValidationResult> {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    
    // Check for unconnected pins
    for component in &schematic.components {
        for pin in &component.pins {
            if !pin.connected {
                warnings.push(format!(
                    "Unconnected pin: {} on component {}",
                    pin.name, component.reference
                ));
            }
        }
    }
    
    // Check for duplicate references
    let mut references = std::collections::HashSet::new();
    for component in &schematic.components {
        if !references.insert(&component.reference) {
            errors.push(format!("Duplicate reference: {}", component.reference));
        }
    }
    
    // Check for floating wires
    for wire in &schematic.wires {
        if wire.points.len() < 2 {
            errors.push(format!("Invalid wire: {} has less than 2 points", wire.id));
        }
    }
    
    Ok(ValidationResult {
        is_valid: errors.is_empty(),
        errors,
        warnings,
    })
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}