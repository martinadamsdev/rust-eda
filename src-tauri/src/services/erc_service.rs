use crate::models::schematic::Schematic;
use crate::models::component::{Component as SchematicComponent, Pin};
use crate::models::wire::Wire as SchematicWire;
use crate::utils::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ERCReport {
    pub errors: Vec<ERCError>,
    pub warnings: Vec<ERCWarning>,
    pub passed: bool,
    pub timestamp: i64,
    pub statistics: ERCStatistics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ERCError {
    pub error_type: ERCErrorType,
    pub message: String,
    pub location: Option<ERCLocation>,
    pub severity: ERCSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ERCWarning {
    pub warning_type: ERCWarningType,
    pub message: String,
    pub location: Option<ERCLocation>,
    pub severity: ERCSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ERCLocation {
    pub x: f64,
    pub y: f64,
    pub component_id: Option<String>,
    pub wire_id: Option<String>,
    pub pin_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ERCErrorType {
    UnconnectedPin,
    ShortCircuit,
    PowerGroundShort,
    MultipleDrivers,
    NoDriver,
    InvalidConnection,
    MissingGround,
    MissingPower,
    DuplicateReference,
    FloatingNet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ERCWarningType {
    UnusedPin,
    PowerPinNotConnected,
    GroundPinNotConnected,
    NoDecouplingCapacitor,
    LongTrace,
    HighImpedanceNet,
    MissingPullResistor,
    UnlabeledNet,
    SinglePinNet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ERCSeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ERCStatistics {
    pub total_components: usize,
    pub total_wires: usize,
    pub total_pins: usize,
    pub connected_pins: usize,
    pub unconnected_pins: usize,
    pub total_nets: usize,
    pub power_nets: usize,
    pub ground_nets: usize,
}

#[derive(Debug, Clone)]
struct NetConnection {
    net_name: String,
    connected_pins: Vec<PinConnection>,
    wire_ids: Vec<String>,
}

#[derive(Debug, Clone)]
struct PinConnection {
    component_id: String,
    pin_id: String,
    pin_type: PinType,
    #[allow(dead_code)]
    x: f64,
    #[allow(dead_code)]
    y: f64,
}

#[derive(Debug, Clone, PartialEq)]
enum PinType {
    Input,
    Output,
    Bidirectional,
    Power,
    Ground,
    Passive,
    #[allow(dead_code)]
    NotConnected,
}

pub struct ERCChecker {
    schematic: Schematic,
    nets: HashMap<String, NetConnection>,
    errors: Vec<ERCError>,
    warnings: Vec<ERCWarning>,
}

impl ERCChecker {
    pub fn new(schematic: Schematic) -> Self {
        Self {
            schematic,
            nets: HashMap::new(),
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn run_check(&mut self) -> Result<ERCReport> {
        // Build net connections
        self.build_nets();
        
        // Run various checks
        self.check_unconnected_pins();
        self.check_power_ground_connections();
        self.check_short_circuits();
        self.check_multiple_drivers();
        self.check_floating_nets();
        self.check_duplicate_references();
        self.check_decoupling_capacitors();
        self.check_pull_resistors();
        self.check_net_labels();
        
        // Calculate statistics
        let statistics = self.calculate_statistics();
        
        // Create report
        let report = ERCReport {
            errors: self.errors.clone(),
            warnings: self.warnings.clone(),
            passed: self.errors.is_empty(),
            timestamp: chrono::Utc::now().timestamp(),
            statistics,
        };
        
        Ok(report)
    }

    fn build_nets(&mut self) {
        // Build connectivity graph from wires and components
        let mut net_id = 0;
        let mut visited_wires = HashSet::new();
        
        for wire in &self.schematic.wires {
            if visited_wires.contains(&wire.id) {
                continue;
            }
            
            // Wire doesn't have net_name field, generate one
            let net_name = format!("NET_{}", net_id);
            net_id += 1;
            
            let mut net = NetConnection {
                net_name: net_name.clone(),
                connected_pins: Vec::new(),
                wire_ids: vec![wire.id.clone()],
            };
            
            // Find all connected wires and components
            self.trace_net(&wire.id, &mut net, &mut visited_wires);
            
            self.nets.insert(net_name, net);
        }
    }

    fn trace_net(&self, wire_id: &str, net: &mut NetConnection, visited: &mut HashSet<String>) {
        if visited.contains(wire_id) {
            return;
        }
        visited.insert(wire_id.to_string());
        
        // Find wire
        let wire = match self.schematic.wires.iter().find(|w| w.id == wire_id) {
            Some(w) => w,
            None => return,
        };
        
        // Check connections to components
        for component in &self.schematic.components {
            for pin in &component.pins {
                if self.is_pin_connected_to_wire(pin, wire) {
                    net.connected_pins.push(PinConnection {
                        component_id: component.id.clone(),
                        pin_id: pin.id.clone(),
                        pin_type: self.get_pin_type(component, pin),
                        x: pin.x,
                        y: pin.y,
                    });
                }
            }
        }
        
        // Find connected wires
        for other_wire in &self.schematic.wires {
            if other_wire.id != wire_id && self.are_wires_connected(wire, other_wire) {
                net.wire_ids.push(other_wire.id.clone());
                self.trace_net(&other_wire.id, net, visited);
            }
        }
    }

    fn is_pin_connected_to_wire(&self, pin: &Pin, wire: &SchematicWire) -> bool {
        const THRESHOLD: f64 = 5.0; // Connection threshold in pixels
        
        // Check if pin is at any wire endpoint
        if let Some(first) = wire.points.first() {
            if (pin.x - first.x).abs() < THRESHOLD && (pin.y - first.y).abs() < THRESHOLD {
                return true;
            }
        }
        
        if let Some(last) = wire.points.last() {
            if (pin.x - last.x).abs() < THRESHOLD && (pin.y - last.y).abs() < THRESHOLD {
                return true;
            }
        }
        
        // Check if pin is on wire segment
        for window in wire.points.windows(2) {
            if self.is_point_on_line_segment(
                (pin.x, pin.y),
                (window[0].x, window[0].y),
                (window[1].x, window[1].y),
                THRESHOLD
            ) {
                return true;
            }
        }
        
        false
    }

    fn are_wires_connected(&self, wire1: &SchematicWire, wire2: &SchematicWire) -> bool {
        const THRESHOLD: f64 = 5.0;
        
        // Check endpoint connections
        for p1 in &[wire1.points.first(), wire1.points.last()] {
            for p2 in &[wire2.points.first(), wire2.points.last()] {
                if let (Some(p1), Some(p2)) = (p1, p2) {
                    if (p1.x - p2.x).abs() < THRESHOLD && (p1.y - p2.y).abs() < THRESHOLD {
                        return true;
                    }
                }
            }
        }
        
        false
    }

    fn is_point_on_line_segment(&self, point: (f64, f64), start: (f64, f64), end: (f64, f64), threshold: f64) -> bool {
        let dist_to_line = self.distance_point_to_line_segment(point, start, end);
        dist_to_line < threshold
    }

    fn distance_point_to_line_segment(&self, point: (f64, f64), start: (f64, f64), end: (f64, f64)) -> f64 {
        let dx = end.0 - start.0;
        let dy = end.1 - start.1;
        let length_squared = dx * dx + dy * dy;
        
        if length_squared == 0.0 {
            // Start and end are the same point
            return ((point.0 - start.0).powi(2) + (point.1 - start.1).powi(2)).sqrt();
        }
        
        let t = ((point.0 - start.0) * dx + (point.1 - start.1) * dy) / length_squared;
        let t = t.max(0.0).min(1.0);
        
        let projection = (start.0 + t * dx, start.1 + t * dy);
        ((point.0 - projection.0).powi(2) + (point.1 - projection.1).powi(2)).sqrt()
    }

    fn get_pin_type(&self, _component: &SchematicComponent, _pin: &Pin) -> PinType {
        // TODO: Determine pin type based on component type and pin name
        // For now, return a default
        PinType::Passive
    }

    fn check_unconnected_pins(&mut self) {
        for component in &self.schematic.components {
            for pin in &component.pins {
                if !pin.connected {
                    // Check if this pin is actually connected via nets
                    let is_connected = self.nets.values().any(|net| {
                        net.connected_pins.iter().any(|conn| {
                            conn.component_id == component.id && conn.pin_id == pin.id
                        })
                    });
                    
                    if !is_connected {
                        self.errors.push(ERCError {
                            error_type: ERCErrorType::UnconnectedPin,
                            message: format!(
                                "Pin {} of component {} is not connected",
                                pin.id, component.reference
                            ),
                            location: Some(ERCLocation {
                                x: pin.x,
                                y: pin.y,
                                component_id: Some(component.id.clone()),
                                wire_id: None,
                                pin_id: Some(pin.id.clone()),
                            }),
                            severity: ERCSeverity::High,
                        });
                    }
                }
            }
        }
    }

    fn check_power_ground_connections(&mut self) {
        // Check for power and ground shorts
        for net in self.nets.values() {
            let has_power = net.connected_pins.iter().any(|p| p.pin_type == PinType::Power);
            let has_ground = net.connected_pins.iter().any(|p| p.pin_type == PinType::Ground);
            
            if has_power && has_ground {
                self.errors.push(ERCError {
                    error_type: ERCErrorType::PowerGroundShort,
                    message: format!("Power and ground are shorted in net {}", net.net_name),
                    location: None,
                    severity: ERCSeverity::Critical,
                });
            }
        }
        
        // Check for missing power/ground connections
        let has_power_net = self.nets.values().any(|net| {
            net.net_name.to_uppercase().contains("VCC") || 
            net.net_name.to_uppercase().contains("VDD") ||
            net.net_name.contains("+")
        });
        
        let has_ground_net = self.nets.values().any(|net| {
            net.net_name.to_uppercase().contains("GND") || 
            net.net_name.to_uppercase().contains("VSS")
        });
        
        if !has_power_net && self.schematic.components.len() > 2 {
            self.warnings.push(ERCWarning {
                warning_type: ERCWarningType::PowerPinNotConnected,
                message: "No power net detected in the schematic".to_string(),
                location: None,
                severity: ERCSeverity::Medium,
            });
        }
        
        if !has_ground_net && self.schematic.components.len() > 2 {
            self.warnings.push(ERCWarning {
                warning_type: ERCWarningType::GroundPinNotConnected,
                message: "No ground net detected in the schematic".to_string(),
                location: None,
                severity: ERCSeverity::Medium,
            });
        }
    }

    fn check_short_circuits(&mut self) {
        // Check for multiple outputs driving the same net
        for net in self.nets.values() {
            let output_pins: Vec<_> = net.connected_pins.iter()
                .filter(|p| p.pin_type == PinType::Output)
                .collect();
            
            if output_pins.len() > 1 {
                self.errors.push(ERCError {
                    error_type: ERCErrorType::MultipleDrivers,
                    message: format!(
                        "Net {} has multiple output drivers",
                        net.net_name
                    ),
                    location: None,
                    severity: ERCSeverity::High,
                });
            }
        }
    }

    fn check_multiple_drivers(&mut self) {
        // Already handled in check_short_circuits
    }

    fn check_floating_nets(&mut self) {
        for net in self.nets.values() {
            if net.connected_pins.len() == 1 {
                self.warnings.push(ERCWarning {
                    warning_type: ERCWarningType::SinglePinNet,
                    message: format!(
                        "Net {} is connected to only one pin",
                        net.net_name
                    ),
                    location: None,
                    severity: ERCSeverity::Low,
                });
            }
            
            // Check for nets with no drivers
            let has_driver = net.connected_pins.iter().any(|p| {
                p.pin_type == PinType::Output || 
                p.pin_type == PinType::Bidirectional ||
                p.pin_type == PinType::Power
            });
            
            let needs_driver = net.connected_pins.iter().any(|p| {
                p.pin_type == PinType::Input
            });
            
            if needs_driver && !has_driver {
                self.errors.push(ERCError {
                    error_type: ERCErrorType::NoDriver,
                    message: format!(
                        "Net {} has input pins but no driver",
                        net.net_name
                    ),
                    location: None,
                    severity: ERCSeverity::High,
                });
            }
        }
    }

    fn check_duplicate_references(&mut self) {
        let mut references = HashSet::new();
        
        for component in &self.schematic.components {
            if !references.insert(component.reference.clone()) {
                self.errors.push(ERCError {
                    error_type: ERCErrorType::DuplicateReference,
                    message: format!(
                        "Duplicate component reference: {}",
                        component.reference
                    ),
                    location: Some(ERCLocation {
                        x: component.x,
                        y: component.y,
                        component_id: Some(component.id.clone()),
                        wire_id: None,
                        pin_id: None,
                    }),
                    severity: ERCSeverity::High,
                });
            }
        }
    }

    fn check_decoupling_capacitors(&mut self) {
        // Check if ICs have nearby decoupling capacitors
        for component in &self.schematic.components {
            if component.reference.starts_with('U') || component.reference.starts_with("IC") {
                // Look for nearby capacitors
                let has_nearby_cap = self.schematic.components.iter().any(|other| {
                    if other.reference.starts_with('C') {
                        let distance = ((component.x - other.x).powi(2) + 
                                       (component.y - other.y).powi(2)).sqrt();
                        distance < 100.0 // Within 100 pixels
                    } else {
                        false
                    }
                });
                
                if !has_nearby_cap {
                    self.warnings.push(ERCWarning {
                        warning_type: ERCWarningType::NoDecouplingCapacitor,
                        message: format!(
                            "IC {} may need a decoupling capacitor",
                            component.reference
                        ),
                        location: Some(ERCLocation {
                            x: component.x,
                            y: component.y,
                            component_id: Some(component.id.clone()),
                            wire_id: None,
                            pin_id: None,
                        }),
                        severity: ERCSeverity::Low,
                    });
                }
            }
        }
    }

    fn check_pull_resistors(&mut self) {
        // Check for high-impedance inputs that may need pull resistors
        for net in self.nets.values() {
            let input_pins: Vec<_> = net.connected_pins.iter()
                .filter(|p| p.pin_type == PinType::Input)
                .collect();
            
            let has_driver = net.connected_pins.iter().any(|p| {
                p.pin_type == PinType::Output || 
                p.pin_type == PinType::Bidirectional
            });
            
            let has_pull_resistor = self.schematic.components.iter().any(|comp| {
                comp.reference.starts_with('R') && 
                comp.pins.iter().any(|pin| {
                    net.connected_pins.iter().any(|conn| {
                        conn.component_id == comp.id && conn.pin_id == pin.id
                    })
                })
            });
            
            if !input_pins.is_empty() && !has_driver && !has_pull_resistor {
                self.warnings.push(ERCWarning {
                    warning_type: ERCWarningType::MissingPullResistor,
                    message: format!(
                        "Net {} has high-impedance inputs, consider adding a pull resistor",
                        net.net_name
                    ),
                    location: None,
                    severity: ERCSeverity::Low,
                });
            }
        }
    }

    fn check_net_labels(&mut self) {
        for net in self.nets.values() {
            if net.net_name.starts_with("NET_") {
                self.warnings.push(ERCWarning {
                    warning_type: ERCWarningType::UnlabeledNet,
                    message: format!(
                        "Net {} is not labeled, consider adding a descriptive name",
                        net.net_name
                    ),
                    location: None,
                    severity: ERCSeverity::Info,
                });
            }
        }
    }

    fn calculate_statistics(&self) -> ERCStatistics {
        let total_pins: usize = self.schematic.components.iter()
            .map(|c| c.pins.len())
            .sum();
        
        let connected_pins: usize = self.nets.values()
            .map(|net| net.connected_pins.len())
            .sum();
        
        let power_nets = self.nets.values()
            .filter(|net| {
                net.net_name.to_uppercase().contains("VCC") || 
                net.net_name.to_uppercase().contains("VDD") ||
                net.net_name.contains("+")
            })
            .count();
        
        let ground_nets = self.nets.values()
            .filter(|net| {
                net.net_name.to_uppercase().contains("GND") || 
                net.net_name.to_uppercase().contains("VSS")
            })
            .count();
        
        ERCStatistics {
            total_components: self.schematic.components.len(),
            total_wires: self.schematic.wires.len(),
            total_pins,
            connected_pins,
            unconnected_pins: total_pins.saturating_sub(connected_pins),
            total_nets: self.nets.len(),
            power_nets,
            ground_nets,
        }
    }
}

// Tauri commands
#[tauri::command]
pub async fn run_erc_check(schematic: Schematic) -> Result<ERCReport> {
    let mut checker = ERCChecker::new(schematic);
    checker.run_check()
}

#[tauri::command]
pub async fn get_erc_rules() -> Result<Vec<ERCRule>> {
    Ok(vec![
        ERCRule {
            id: "unconnected_pins".to_string(),
            name: "Unconnected Pins".to_string(),
            description: "Check for pins that are not connected to any net".to_string(),
            enabled: true,
            severity: ERCSeverity::High,
        },
        ERCRule {
            id: "power_ground_short".to_string(),
            name: "Power/Ground Short".to_string(),
            description: "Check for shorts between power and ground".to_string(),
            enabled: true,
            severity: ERCSeverity::Critical,
        },
        ERCRule {
            id: "multiple_drivers".to_string(),
            name: "Multiple Drivers".to_string(),
            description: "Check for nets with multiple output drivers".to_string(),
            enabled: true,
            severity: ERCSeverity::High,
        },
        ERCRule {
            id: "floating_nets".to_string(),
            name: "Floating Nets".to_string(),
            description: "Check for nets with no drivers".to_string(),
            enabled: true,
            severity: ERCSeverity::Medium,
        },
        ERCRule {
            id: "duplicate_references".to_string(),
            name: "Duplicate References".to_string(),
            description: "Check for duplicate component references".to_string(),
            enabled: true,
            severity: ERCSeverity::High,
        },
    ])
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ERCRule {
    pub id: String,
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub severity: ERCSeverity,
}