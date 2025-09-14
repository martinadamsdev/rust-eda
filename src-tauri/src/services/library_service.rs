use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::models::component::{ComponentLibrary, ComponentTemplate, ComponentCategory, ComponentSymbol, DrawCommand, DrawCommandType, DrawStyle, ParameterTemplate, ParameterType, PinTemplate, SymbolGraphics, GraphicsBounds};
use crate::models::{PinType, ElectricalType};
use crate::utils::error::Result;

use super::extended_components;
use super::logic_gates;
use super::integrated_circuits;
use super::sensors_power;

pub struct LibraryService {
    libraries: Arc<Mutex<HashMap<String, ComponentLibrary>>>,
}

impl LibraryService {
    pub fn new() -> Self {
        let service = Self {
            libraries: Arc::new(Mutex::new(HashMap::new())),
        };
        
        // Initialize with standard library
        if let Err(e) = service.create_standard_library() {
            eprintln!("Failed to create standard library: {:?}", e);
        }
        
        service
    }

    pub fn create_standard_library(&self) -> Result<()> {
        let mut standard_lib = ComponentLibrary::new("Standard Electronics".to_string())?;
        
        // Add categories
        let categories = vec![
            ComponentCategory {
                id: "passive".to_string(),
                name: "Passive Components".to_string(),
                parent_id: None,
                description: Some("Resistors, capacitors, inductors".to_string()),
                color: Some("#4CAF50".to_string()),
            },
            ComponentCategory {
                id: "active".to_string(),
                name: "Active Components".to_string(),
                parent_id: None,
                description: Some("Transistors, ICs, semiconductors".to_string()),
                color: Some("#2196F3".to_string()),
            },
            ComponentCategory {
                id: "connector".to_string(),
                name: "Connectors".to_string(),
                parent_id: None,
                description: Some("Headers, terminals, connectors".to_string()),
                color: Some("#FF9800".to_string()),
            },
            ComponentCategory {
                id: "power".to_string(),
                name: "Power Components".to_string(),
                parent_id: None,
                description: Some("Voltage regulators, power supplies".to_string()),
                color: Some("#F44336".to_string()),
            },
            ComponentCategory {
                id: "digital".to_string(),
                name: "Digital Logic".to_string(),
                parent_id: None,
                description: Some("Logic gates, flip-flops, counters".to_string()),
                color: Some("#9C27B0".to_string()),
            },
        ];

        for category in categories {
            standard_lib.add_category(category)?;
        }

        // Add basic components
        self.add_resistor_template(&mut standard_lib)?;
        self.add_capacitor_template(&mut standard_lib)?;
        self.add_led_template(&mut standard_lib)?;
        self.add_op_amp_template(&mut standard_lib)?;
        self.add_connector_template(&mut standard_lib)?;
        
        // Add extended components (50+ total)
        extended_components::add_all_extended_components(&mut standard_lib)?;
        logic_gates::add_logic_gates(&mut standard_lib)?;
        integrated_circuits::add_integrated_circuits(&mut standard_lib)?;
        sensors_power::add_sensors(&mut standard_lib)?;
        sensors_power::add_power_components(&mut standard_lib)?;
        sensors_power::add_display_components(&mut standard_lib)?;

        // Store the library
        let mut libraries = self.libraries.lock()
            .map_err(|_| crate::utils::error::AppError::ThreadError("Mutex poisoned in initialize_standard_library".to_string()))?;
        libraries.insert(standard_lib.id.clone(), standard_lib);
        
        Ok(())
    }

    fn add_resistor_template(&self, library: &mut ComponentLibrary) -> Result<()> {
        let symbol = ComponentSymbol {
            width: 60.0,
            height: 20.0,
            draw_commands: vec![
                // IEEE Standard Zigzag Resistor Symbol
                DrawCommand {
                    command_type: DrawCommandType::Line,
                    parameters: vec![-30.0, 0.0, -15.0, 0.0],
                    style: Some(DrawStyle {
                        stroke_width: 2.0,
                        stroke_color: "#000000".to_string(),
                        fill_color: None,
                    }),
                },
                DrawCommand {
                    command_type: DrawCommandType::Line,
                    parameters: vec![-15.0, 0.0, -12.0, -8.0],
                    style: Some(DrawStyle {
                        stroke_width: 2.0,
                        stroke_color: "#000000".to_string(),
                        fill_color: None,
                    }),
                },
                DrawCommand {
                    command_type: DrawCommandType::Line,
                    parameters: vec![-12.0, -8.0, -6.0, 8.0],
                    style: Some(DrawStyle {
                        stroke_width: 2.0,
                        stroke_color: "#000000".to_string(),
                        fill_color: None,
                    }),
                },
                DrawCommand {
                    command_type: DrawCommandType::Line,
                    parameters: vec![-6.0, 8.0, 0.0, -8.0],
                    style: Some(DrawStyle {
                        stroke_width: 2.0,
                        stroke_color: "#000000".to_string(),
                        fill_color: None,
                    }),
                },
                DrawCommand {
                    command_type: DrawCommandType::Line,
                    parameters: vec![0.0, -8.0, 6.0, 8.0],
                    style: Some(DrawStyle {
                        stroke_width: 2.0,
                        stroke_color: "#000000".to_string(),
                        fill_color: None,
                    }),
                },
                DrawCommand {
                    command_type: DrawCommandType::Line,
                    parameters: vec![6.0, 8.0, 12.0, -8.0],
                    style: Some(DrawStyle {
                        stroke_width: 2.0,
                        stroke_color: "#000000".to_string(),
                        fill_color: None,
                    }),
                },
                DrawCommand {
                    command_type: DrawCommandType::Line,
                    parameters: vec![12.0, -8.0, 15.0, 0.0],
                    style: Some(DrawStyle {
                        stroke_width: 2.0,
                        stroke_color: "#000000".to_string(),
                        fill_color: None,
                    }),
                },
                DrawCommand {
                    command_type: DrawCommandType::Line,
                    parameters: vec![15.0, 0.0, 30.0, 0.0],
                    style: Some(DrawStyle {
                        stroke_width: 2.0,
                        stroke_color: "#000000".to_string(),
                        fill_color: None,
                    }),
                },
                // Value text
                DrawCommand {
                    command_type: DrawCommandType::Text,
                    parameters: vec![0.0, 15.0, 10.0],
                    style: Some(DrawStyle {
                        stroke_width: 1.0,
                        stroke_color: "#000000".to_string(),
                        fill_color: None,
                    }),
                },
            ],
            graphics: Some(SymbolGraphics {
                bounds: GraphicsBounds { width: 60.0, height: 20.0 }
            }),
        };

        let mut template = ComponentTemplate::new(
            "Resistor".to_string(),
            "passive".to_string(),
            symbol,
        )?;

        // Add parameters
        template.add_parameter("resistance".to_string(), ParameterTemplate {
            name: "Resistance".to_string(),
            parameter_type: ParameterType::Number,
            default_value: serde_json::Value::Number(serde_json::Number::from(1000)),
            min_value: Some(0.1),
            max_value: Some(1e12),
            unit: Some("Ω".to_string()),
            description: Some("Resistance value in ohms".to_string()),
            required: true,
        })?;

        template.add_parameter("tolerance".to_string(), ParameterTemplate {
            name: "Tolerance".to_string(),
            parameter_type: ParameterType::Choice(vec!["1%".to_string(), "5%".to_string(), "10%".to_string()]),
            default_value: serde_json::Value::String("5%".to_string()),
            min_value: None,
            max_value: None,
            unit: None,
            description: Some("Resistance tolerance".to_string()),
            required: false,
        })?;

        // Add pins
        template.add_pin(PinTemplate {
            id: "pin1".to_string(),
            name: "1".to_string(),
            number: "1".to_string(),
            x: -30.0,
            y: 0.0,
            pin_type: PinType::Passive,
            electrical: ElectricalType {
                voltage: None,
                current: None,
                impedance: None,
            },
        })?;

        template.add_pin(PinTemplate {
            id: "pin2".to_string(),
            name: "2".to_string(),
            number: "2".to_string(),
            x: 30.0,
            y: 0.0,
            pin_type: PinType::Passive,
            electrical: ElectricalType {
                voltage: None,
                current: None,
                impedance: None,
            },
        })?;

        template.keywords = vec!["resistor".to_string(), "R".to_string(), "ohm".to_string()];
        
        library.add_component_template(template)?;
        Ok(())
    }

    fn add_capacitor_template(&self, library: &mut ComponentLibrary) -> Result<()> {
        let symbol = ComponentSymbol {
            width: 30.0,
            height: 30.0,
            draw_commands: vec![
                DrawCommand {
                    command_type: DrawCommandType::Line,
                    parameters: vec![-5.0, -15.0, -5.0, 15.0],
                    style: Some(DrawStyle {
                        stroke_width: 3.0,
                        stroke_color: "#000000".to_string(),
                        fill_color: None,
                    }),
                },
                DrawCommand {
                    command_type: DrawCommandType::Line,
                    parameters: vec![5.0, -15.0, 5.0, 15.0],
                    style: Some(DrawStyle {
                        stroke_width: 3.0,
                        stroke_color: "#000000".to_string(),
                        fill_color: None,
                    }),
                },
                DrawCommand {
                    command_type: DrawCommandType::Line,
                    parameters: vec![-20.0, 0.0, -5.0, 0.0],
                    style: Some(DrawStyle {
                        stroke_width: 2.0,
                        stroke_color: "#000000".to_string(),
                        fill_color: None,
                    }),
                },
                DrawCommand {
                    command_type: DrawCommandType::Line,
                    parameters: vec![5.0, 0.0, 20.0, 0.0],
                    style: Some(DrawStyle {
                        stroke_width: 2.0,
                        stroke_color: "#000000".to_string(),
                        fill_color: None,
                    }),
                },
            ],
            graphics: Some(SymbolGraphics {
                bounds: GraphicsBounds { width: 40.0, height: 20.0 }
            }),
        };

        let mut template = ComponentTemplate::new(
            "Capacitor".to_string(),
            "passive".to_string(),
            symbol,
        )?;

        template.add_parameter("capacitance".to_string(), ParameterTemplate {
            name: "Capacitance".to_string(),
            parameter_type: ParameterType::Number,
            default_value: serde_json::Value::Number(
                serde_json::Number::from_f64(1e-6)
                    .ok_or_else(|| crate::utils::error::AppError::InvalidInput("Invalid capacitance value".to_string()))?
            ),
            min_value: Some(1e-15),
            max_value: Some(1.0),
            unit: Some("F".to_string()),
            description: Some("Capacitance value in farads".to_string()),
            required: true,
        })?;

        template.add_parameter("voltage_rating".to_string(), ParameterTemplate {
            name: "Voltage Rating".to_string(),
            parameter_type: ParameterType::Number,
            default_value: serde_json::Value::Number(serde_json::Number::from(16)),
            min_value: Some(1.0),
            max_value: Some(1000.0),
            unit: Some("V".to_string()),
            description: Some("Maximum voltage rating".to_string()),
            required: false,
        })?;

        // Add pins
        template.add_pin(PinTemplate {
            id: "pin1".to_string(),
            name: "1".to_string(),
            number: "1".to_string(),
            x: -20.0,
            y: 0.0,
            pin_type: PinType::Passive,
            electrical: ElectricalType {
                voltage: None,
                current: None,
                impedance: None,
            },
        })?;

        template.add_pin(PinTemplate {
            id: "pin2".to_string(),
            name: "2".to_string(),
            number: "2".to_string(),
            x: 20.0,
            y: 0.0,
            pin_type: PinType::Passive,
            electrical: ElectricalType {
                voltage: None,
                current: None,
                impedance: None,
            },
        })?;

        template.keywords = vec!["capacitor".to_string(), "C".to_string(), "cap".to_string()];
        
        library.add_component_template(template)?;
        Ok(())
    }

    fn add_led_template(&self, library: &mut ComponentLibrary) -> Result<()> {
        let symbol = ComponentSymbol {
            width: 40.0,
            height: 30.0,
            draw_commands: vec![
                // Triangle (diode)
                DrawCommand {
                    command_type: DrawCommandType::Polygon,
                    parameters: vec![-5.0, -10.0, -5.0, 10.0, 5.0, 0.0],
                    style: Some(DrawStyle {
                        stroke_width: 2.0,
                        stroke_color: "#000000".to_string(),
                        fill_color: Some("#FF0000".to_string()),
                    }),
                },
                // Cathode line
                DrawCommand {
                    command_type: DrawCommandType::Line,
                    parameters: vec![5.0, -10.0, 5.0, 10.0],
                    style: Some(DrawStyle {
                        stroke_width: 2.0,
                        stroke_color: "#000000".to_string(),
                        fill_color: None,
                    }),
                },
                // Anode line
                DrawCommand {
                    command_type: DrawCommandType::Line,
                    parameters: vec![-20.0, 0.0, -5.0, 0.0],
                    style: Some(DrawStyle {
                        stroke_width: 2.0,
                        stroke_color: "#000000".to_string(),
                        fill_color: None,
                    }),
                },
                // Cathode line
                DrawCommand {
                    command_type: DrawCommandType::Line,
                    parameters: vec![5.0, 0.0, 20.0, 0.0],
                    style: Some(DrawStyle {
                        stroke_width: 2.0,
                        stroke_color: "#000000".to_string(),
                        fill_color: None,
                    }),
                },
            ],
            graphics: Some(SymbolGraphics {
                bounds: GraphicsBounds { width: 40.0, height: 20.0 }
            }),
        };

        let mut template = ComponentTemplate::new(
            "LED".to_string(),
            "active".to_string(),
            symbol,
        )?;

        template.add_parameter("forward_voltage".to_string(), ParameterTemplate {
            name: "Forward Voltage".to_string(),
            parameter_type: ParameterType::Number,
            default_value: serde_json::Value::Number(
                serde_json::Number::from_f64(2.1)
                    .ok_or_else(|| crate::utils::error::AppError::InvalidInput("Invalid voltage value".to_string()))?
            ),
            min_value: Some(1.0),
            max_value: Some(5.0),
            unit: Some("V".to_string()),
            description: Some("Forward voltage drop".to_string()),
            required: false,
        })?;

        template.add_parameter("color".to_string(), ParameterTemplate {
            name: "Color".to_string(),
            parameter_type: ParameterType::Choice(vec![
                "Red".to_string(), 
                "Green".to_string(), 
                "Blue".to_string(), 
                "Yellow".to_string(), 
                "White".to_string()
            ]),
            default_value: serde_json::Value::String("Red".to_string()),
            min_value: None,
            max_value: None,
            unit: None,
            description: Some("LED color".to_string()),
            required: false,
        })?;

        // Add pins
        template.add_pin(PinTemplate {
            id: "anode".to_string(),
            name: "Anode".to_string(),
            number: "1".to_string(),
            x: -20.0,
            y: 0.0,
            pin_type: PinType::Input,
            electrical: ElectricalType {
                voltage: Some(3.3),
                current: Some(0.02),
                impedance: None,
            },
        })?;

        template.add_pin(PinTemplate {
            id: "cathode".to_string(),
            name: "Cathode".to_string(),
            number: "2".to_string(),
            x: 20.0,
            y: 0.0,
            pin_type: PinType::Output,
            electrical: ElectricalType {
                voltage: Some(0.0),
                current: Some(0.02),
                impedance: None,
            },
        })?;

        template.keywords = vec!["led".to_string(), "diode".to_string(), "light".to_string()];
        
        library.add_component_template(template)?;
        Ok(())
    }

    fn add_op_amp_template(&self, library: &mut ComponentLibrary) -> Result<()> {
        let symbol = ComponentSymbol {
            width: 60.0,
            height: 40.0,
            draw_commands: vec![
                // Triangle body
                DrawCommand {
                    command_type: DrawCommandType::Polygon,
                    parameters: vec![-20.0, -20.0, -20.0, 20.0, 20.0, 0.0],
                    style: Some(DrawStyle {
                        stroke_width: 2.0,
                        stroke_color: "#000000".to_string(),
                        fill_color: Some("#FFFFFF".to_string()),
                    }),
                },
                // + symbol
                DrawCommand {
                    command_type: DrawCommandType::Text,
                    parameters: vec![-10.0, -8.0],
                    style: None,
                },
                // - symbol  
                DrawCommand {
                    command_type: DrawCommandType::Text,
                    parameters: vec![-10.0, 8.0],
                    style: None,
                },
            ],
            graphics: Some(SymbolGraphics {
                bounds: GraphicsBounds { width: 40.0, height: 20.0 }
            }),
        };

        let mut template = ComponentTemplate::new(
            "Op-Amp".to_string(),
            "active".to_string(),
            symbol,
        )?;

        template.add_parameter("gain".to_string(), ParameterTemplate {
            name: "Gain".to_string(),
            parameter_type: ParameterType::Number,
            default_value: serde_json::Value::Number(serde_json::Number::from(100000)),
            min_value: Some(1.0),
            max_value: Some(1e9),
            unit: Some("V/V".to_string()),
            description: Some("Open loop gain".to_string()),
            required: false,
        })?;

        // Add pins
        template.add_pin(PinTemplate {
            id: "in_pos".to_string(),
            name: "V+".to_string(),
            number: "3".to_string(),
            x: -20.0,
            y: -8.0,
            pin_type: PinType::Input,
            electrical: ElectricalType {
                voltage: Some(5.0),
                current: Some(0.001),
                impedance: Some(1e6),
            },
        })?;

        template.add_pin(PinTemplate {
            id: "in_neg".to_string(),
            name: "V-".to_string(),
            number: "2".to_string(),
            x: -20.0,
            y: 8.0,
            pin_type: PinType::Input,
            electrical: ElectricalType {
                voltage: Some(5.0),
                current: Some(0.001),
                impedance: Some(1e6),
            },
        })?;

        template.add_pin(PinTemplate {
            id: "output".to_string(),
            name: "Out".to_string(),
            number: "6".to_string(),
            x: 20.0,
            y: 0.0,
            pin_type: PinType::Output,
            electrical: ElectricalType {
                voltage: Some(5.0),
                current: Some(0.025),
                impedance: Some(75.0),
            },
        })?;

        template.keywords = vec!["op-amp".to_string(), "amplifier".to_string(), "opamp".to_string()];
        
        library.add_component_template(template)?;
        Ok(())
    }

    fn add_connector_template(&self, library: &mut ComponentLibrary) -> Result<()> {
        let symbol = ComponentSymbol {
            width: 30.0,
            height: 60.0,
            draw_commands: vec![
                DrawCommand {
                    command_type: DrawCommandType::Rectangle,
                    parameters: vec![-10.0, -30.0, 20.0, 60.0],
                    style: Some(DrawStyle {
                        stroke_width: 2.0,
                        stroke_color: "#000000".to_string(),
                        fill_color: Some("#DDDDDD".to_string()),
                    }),
                },
            ],
            graphics: Some(SymbolGraphics {
                bounds: GraphicsBounds { width: 40.0, height: 20.0 }
            }),
        };

        let mut template = ComponentTemplate::new(
            "Header 4x1".to_string(),
            "connector".to_string(),
            symbol,
        )?;

        template.add_parameter("pin_count".to_string(), ParameterTemplate {
            name: "Pin Count".to_string(),
            parameter_type: ParameterType::Choice(vec!["2".to_string(), "4".to_string(), "8".to_string(), "16".to_string()]),
            default_value: serde_json::Value::String("4".to_string()),
            min_value: None,
            max_value: None,
            unit: None,
            description: Some("Number of pins".to_string()),
            required: true,
        })?;

        // Add pins
        for i in 1..=4 {
            template.add_pin(PinTemplate {
                id: format!("pin{}", i),
                name: format!("{}", i),
                number: format!("{}", i),
                x: -20.0,
                y: -22.5 + (i as f64 - 1.0) * 15.0,
                pin_type: PinType::Bidirectional,
                electrical: ElectricalType {
                    voltage: None,
                    current: None,
                    impedance: None,
                },
            })?;
        }

        template.keywords = vec!["header".to_string(), "connector".to_string(), "pin".to_string()];
        
        library.add_component_template(template)?;
        Ok(())
    }

    pub fn get_library(&self, id: &str) -> Option<ComponentLibrary> {
        let libraries = match self.libraries.lock() {
            Ok(guard) => guard,
            Err(_) => {
                eprintln!("Warning: Mutex poisoned in get_library, returning None");
                return None;
            }
        };
        libraries.get(id).cloned()
    }

    pub fn get_all_libraries(&self) -> Vec<ComponentLibrary> {
        match self.libraries.lock() {
            Ok(libraries) => libraries.values().cloned().collect(),
            Err(_) => {
                eprintln!("Warning: Mutex poisoned in get_all_libraries, returning empty list");
                Vec::new()
            }
        }
    }

    pub fn get_component_template(&self, library_id: &str, component_id: &str) -> Option<ComponentTemplate> {
        let libraries = match self.libraries.lock() {
            Ok(guard) => guard,
            Err(_) => {
                eprintln!("Warning: Mutex poisoned in get_component_template, returning None");
                return None;
            }
        };
        libraries.get(library_id)?
            .get_component_template(component_id)
            .cloned()
    }

    pub fn search_all_components(&self, query: &str) -> Vec<(String, ComponentTemplate)> {
        let libraries = match self.libraries.lock() {
            Ok(guard) => guard,
            Err(_) => {
                eprintln!("Warning: Mutex poisoned in search_all_components, returning empty results");
                return Vec::new();
            }
        };
        let mut results = Vec::new();
        
        for (lib_id, library) in libraries.iter() {
            let components = library.search_components(query);
            for component in components {
                results.push((lib_id.clone(), component.clone()));
            }
        }
        
        results
    }

    pub fn get_components_by_category(&self, category_id: &str) -> Vec<(String, ComponentTemplate)> {
        let libraries = match self.libraries.lock() {
            Ok(guard) => guard,
            Err(_) => {
                eprintln!("Warning: Mutex poisoned in get_components_by_category, returning empty results");
                return Vec::new();
            }
        };
        let mut results = Vec::new();
        
        for (lib_id, library) in libraries.iter() {
            let components = library.get_components_by_category(category_id);
            for component in components {
                results.push((lib_id.clone(), component.clone()));
            }
        }
        
        results
    }
}

impl Default for LibraryService {
    fn default() -> Self {
        Self::new()
    }
}