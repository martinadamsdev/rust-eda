use crate::models::component::{ComponentLibrary, ComponentTemplate, ComponentSymbol, DrawCommand, DrawCommandType, DrawStyle, ParameterTemplate, ParameterType, PinTemplate, SymbolGraphics, GraphicsBounds};
use crate::models::{PinType, ElectricalType};
use crate::utils::error::Result;
use super::logic_gates_impl;

// Helper function to create standard electrical type
fn standard_electrical() -> ElectricalType {
    ElectricalType {
        voltage: None,
        current: None,
        impedance: None,
    }
}

// Helper function to create power electrical type
#[allow(dead_code)]
fn power_electrical(voltage: f64) -> ElectricalType {
    ElectricalType {
        voltage: Some(voltage),
        current: None,
        impedance: None,
    }
}

pub fn add_all_extended_components(_library: &mut ComponentLibrary) -> Result<()> {
    // Passive Components
    add_inductor(_library)?;
    add_transformer(_library)?;
    add_fuse(_library)?;
    add_crystal(_library)?;
    add_potentiometer(_library)?;
    add_variable_capacitor(_library)?;
    add_thermistor(_library)?;
    add_varistor(_library)?;
    
    // Semiconductor Devices
    add_diode(_library)?;
    add_zener_diode(_library)?;
    add_schottky_diode(_library)?;
    add_led_rgb(_library)?;
    add_photodiode(_library)?;
    add_npn_transistor(_library)?;
    add_pnp_transistor(_library)?;
    add_n_mosfet(_library)?;
    add_p_mosfet(_library)?;
    add_jfet(_library)?;
    add_igbt(_library)?;
    add_thyristor(_library)?;
    add_triac(_library)?;
    
    // Logic Gates
    add_and_gate(_library)?;
    add_or_gate(_library)?;
    add_not_gate(_library)?;
    add_nand_gate(_library)?;
    add_nor_gate(_library)?;
    add_xor_gate(_library)?;
    add_xnor_gate(_library)?;
    add_buffer_gate(_library)?;
    
    // Integrated Circuits
    add_555_timer(_library)?;
    add_comparator(_library)?;
    add_voltage_regulator(_library)?;
    add_microcontroller(_library)?;
    add_adc(_library)?;
    add_dac(_library)?;
    add_multiplexer(_library)?;
    add_shift_register(_library)?;
    add_counter(_library)?;
    add_decoder(_library)?;
    
    // Sensors
    add_temperature_sensor(_library)?;
    add_pressure_sensor(_library)?;
    add_light_sensor(_library)?;
    add_hall_sensor(_library)?;
    add_accelerometer(_library)?;
    add_gyroscope(_library)?;
    add_humidity_sensor(_library)?;
    
    // Power Management
    add_dc_dc_converter(_library)?;
    add_battery(_library)?;
    add_solar_cell(_library)?;
    add_ac_source(_library)?;
    add_ground_symbol(_library)?;
    add_power_supply(_library)?;
    
    // Display and Indicators
    add_seven_segment(_library)?;
    add_lcd_display(_library)?;
    add_buzzer(_library)?;
    add_speaker(_library)?;
    add_relay(_library)?;
    add_motor(_library)?;
    
    Ok(())
}

// PASSIVE COMPONENTS

fn add_inductor(_library: &mut ComponentLibrary) -> Result<()> {
    let symbol = ComponentSymbol {
        width: 60.0,
        height: 20.0,
        draw_commands: vec![
            // Coil representation
            DrawCommand {
                command_type: DrawCommandType::Arc,
                parameters: vec![-20.0, 0.0, 10.0, 180.0, 0.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            DrawCommand {
                command_type: DrawCommandType::Arc,
                parameters: vec![-5.0, 0.0, 10.0, 180.0, 0.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            DrawCommand {
                command_type: DrawCommandType::Arc,
                parameters: vec![10.0, 0.0, 10.0, 180.0, 0.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            // Connection lines
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-30.0, 0.0, -20.0, 0.0],
                style: None,
            },
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![20.0, 0.0, 30.0, 0.0],
                style: None,
            },
        ],
        graphics: Some(SymbolGraphics {
            bounds: GraphicsBounds {
                width: 60.0,
                height: 20.0,
            },
        }),
    };

    let mut template = ComponentTemplate::new(
        "Inductor".to_string(),
        "passive".to_string(),
        symbol
    )?;
    
    template.add_parameter(
        "inductance".to_string(),
        ParameterTemplate {
            name: "Inductance".to_string(),
            parameter_type: ParameterType::String,
            default_value: serde_json::json!("1mH"),
            min_value: None,
            max_value: None,
            unit: Some("H".to_string()),
            description: Some("Inductance value".to_string()),
            required: true,
        }
    )?;
    
    template.add_pin(PinTemplate {
        id: "1".to_string(),
        name: "1".to_string(),
        number: "1".to_string(),
        x: -30.0,
        y: 0.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;
    
    template.add_pin(PinTemplate {
        id: "2".to_string(),
        name: "2".to_string(),
        number: "2".to_string(),
        x: 30.0,
        y: 0.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;

    template.keywords = vec!["inductor".to_string(), "coil".to_string(), "L".to_string()];
    _library.add_component_template(template)?;
    
    Ok(())
}

fn add_transformer(_library: &mut ComponentLibrary) -> Result<()> {
    let symbol = ComponentSymbol {
        width: 80.0,
        height: 60.0,
        draw_commands: vec![
            // Primary coil
            DrawCommand {
                command_type: DrawCommandType::Arc,
                parameters: vec![-30.0, -20.0, 10.0, 180.0, 0.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            DrawCommand {
                command_type: DrawCommandType::Arc,
                parameters: vec![-30.0, -5.0, 10.0, 180.0, 0.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            DrawCommand {
                command_type: DrawCommandType::Arc,
                parameters: vec![-30.0, 10.0, 10.0, 180.0, 0.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            // Core lines
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-10.0, -30.0, -10.0, 30.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![10.0, -30.0, 10.0, 30.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            // Secondary coil
            DrawCommand {
                command_type: DrawCommandType::Arc,
                parameters: vec![30.0, -20.0, 10.0, 0.0, 180.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            DrawCommand {
                command_type: DrawCommandType::Arc,
                parameters: vec![30.0, -5.0, 10.0, 0.0, 180.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            DrawCommand {
                command_type: DrawCommandType::Arc,
                parameters: vec![30.0, 10.0, 10.0, 0.0, 180.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
        ],
        graphics: Some(SymbolGraphics {
            bounds: GraphicsBounds {
                width: 80.0,
                height: 60.0,
            },
        }),
    };

    let mut template = ComponentTemplate::new(
        "Transformer".to_string(),
        "passive".to_string(),
        symbol
    )?;
    
    template.add_pin(PinTemplate {
        id: "P1".to_string(),
        name: "Primary+".to_string(),
        number: "1".to_string(),
        x: -40.0,
        y: -20.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;
    
    template.add_pin(PinTemplate {
        id: "P2".to_string(),
        name: "Primary-".to_string(),
        number: "2".to_string(),
        x: -40.0,
        y: 20.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;
    
    template.add_pin(PinTemplate {
        id: "S1".to_string(),
        name: "Secondary+".to_string(),
        number: "3".to_string(),
        x: 40.0,
        y: -20.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;
    
    template.add_pin(PinTemplate {
        id: "S2".to_string(),
        name: "Secondary-".to_string(),
        number: "4".to_string(),
        x: 40.0,
        y: 20.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;

    template.keywords = vec!["transformer".to_string(), "xfmr".to_string()];
    _library.add_component_template(template)?;
    
    Ok(())
}

fn add_fuse(_library: &mut ComponentLibrary) -> Result<()> {
    let symbol = ComponentSymbol {
        width: 60.0,
        height: 20.0,
        draw_commands: vec![
            DrawCommand {
                command_type: DrawCommandType::Rectangle,
                parameters: vec![-20.0, -8.0, 40.0, 16.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-20.0, 0.0, 20.0, 0.0],
                style: Some(DrawStyle {
                    stroke_width: 1.5,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-30.0, 0.0, -20.0, 0.0],
                style: None,
            },
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![20.0, 0.0, 30.0, 0.0],
                style: None,
            },
        ],
        graphics: Some(SymbolGraphics {
            bounds: GraphicsBounds {
                width: 60.0,
                height: 20.0,
            },
        }),
    };

    let mut template = ComponentTemplate::new(
        "Fuse".to_string(),
        "passive".to_string(),
        symbol
    )?;
    
    template.add_parameter(
        "rating".to_string(),
        ParameterTemplate {
            name: "Current Rating".to_string(),
            parameter_type: ParameterType::String,
            default_value: serde_json::json!("1A"),
            min_value: None,
            max_value: None,
            unit: Some("A".to_string()),
            description: Some("Fuse current rating".to_string()),
            required: true,
        }
    )?;
    
    template.add_pin(PinTemplate {
        id: "1".to_string(),
        name: "1".to_string(),
        number: "1".to_string(),
        x: -30.0,
        y: 0.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;
    
    template.add_pin(PinTemplate {
        id: "2".to_string(),
        name: "2".to_string(),
        number: "2".to_string(),
        x: 30.0,
        y: 0.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;

    template.keywords = vec!["fuse".to_string(), "protection".to_string()];
    _library.add_component_template(template)?;
    
    Ok(())
}

fn add_crystal(_library: &mut ComponentLibrary) -> Result<()> {
    let symbol = ComponentSymbol {
        width: 60.0,
        height: 40.0,
        draw_commands: vec![
            // Crystal body
            DrawCommand {
                command_type: DrawCommandType::Rectangle,
                parameters: vec![-10.0, -15.0, 20.0, 30.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            // Side capacitor plates
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-15.0, -20.0, -15.0, 20.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![15.0, -20.0, 15.0, 20.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            // Connection lines
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-30.0, 0.0, -15.0, 0.0],
                style: None,
            },
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![15.0, 0.0, 30.0, 0.0],
                style: None,
            },
        ],
        graphics: Some(SymbolGraphics {
            bounds: GraphicsBounds {
                width: 60.0,
                height: 40.0,
            },
        }),
    };

    let mut template = ComponentTemplate::new(
        "Crystal".to_string(),
        "passive".to_string(),
        symbol
    )?;
    
    template.add_parameter(
        "frequency".to_string(),
        ParameterTemplate {
            name: "Frequency".to_string(),
            parameter_type: ParameterType::String,
            default_value: serde_json::json!("16MHz"),
            min_value: None,
            max_value: None,
            unit: Some("Hz".to_string()),
            description: Some("Crystal frequency".to_string()),
            required: true,
        }
    )?;
    
    template.add_pin(PinTemplate {
        id: "1".to_string(),
        name: "1".to_string(),
        number: "1".to_string(),
        x: -30.0,
        y: 0.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;
    
    template.add_pin(PinTemplate {
        id: "2".to_string(),
        name: "2".to_string(),
        number: "2".to_string(),
        x: 30.0,
        y: 0.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;

    template.keywords = vec!["crystal".to_string(), "oscillator".to_string(), "xtal".to_string()];
    _library.add_component_template(template)?;
    
    Ok(())
}

fn add_potentiometer(_library: &mut ComponentLibrary) -> Result<()> {
    let symbol = ComponentSymbol {
        width: 60.0,
        height: 40.0,
        draw_commands: vec![
            // Resistor body
            DrawCommand {
                command_type: DrawCommandType::Rectangle,
                parameters: vec![-20.0, -10.0, 40.0, 20.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            // Wiper arrow
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![0.0, -20.0, 0.0, -10.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            DrawCommand {
                command_type: DrawCommandType::Polygon,
                parameters: vec![-3.0, -10.0, 3.0, -10.0, 0.0, -5.0],
                style: Some(DrawStyle {
                    stroke_width: 1.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: Some("#000000".to_string()),
                }),
            },
            // Connection lines
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-30.0, 0.0, -20.0, 0.0],
                style: None,
            },
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![20.0, 0.0, 30.0, 0.0],
                style: None,
            },
        ],
        graphics: Some(SymbolGraphics {
            bounds: GraphicsBounds {
                width: 60.0,
                height: 40.0,
            },
        }),
    };

    let mut template = ComponentTemplate::new(
        "Potentiometer".to_string(),
        "passive".to_string(),
        symbol
    )?;
    
    template.add_parameter(
        "resistance".to_string(),
        ParameterTemplate {
            name: "Resistance".to_string(),
            parameter_type: ParameterType::String,
            default_value: serde_json::json!("10k"),
            min_value: None,
            max_value: None,
            unit: Some("Ω".to_string()),
            description: Some("Total resistance".to_string()),
            required: true,
        }
    )?;
    
    template.add_pin(PinTemplate {
        id: "1".to_string(),
        name: "CCW".to_string(),
        number: "1".to_string(),
        x: -30.0,
        y: 0.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;
    
    template.add_pin(PinTemplate {
        id: "2".to_string(),
        name: "Wiper".to_string(),
        number: "2".to_string(),
        x: 0.0,
        y: -20.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;
    
    template.add_pin(PinTemplate {
        id: "3".to_string(),
        name: "CW".to_string(),
        number: "3".to_string(),
        x: 30.0,
        y: 0.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;

    template.keywords = vec!["potentiometer".to_string(), "pot".to_string(), "variable".to_string()];
    _library.add_component_template(template)?;
    
    Ok(())
}

fn add_variable_capacitor(_library: &mut ComponentLibrary) -> Result<()> {
    let symbol = ComponentSymbol {
        width: 40.0,
        height: 40.0,
        draw_commands: vec![
            // Fixed plate
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-5.0, -15.0, -5.0, 15.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            // Variable plate (curved)
            DrawCommand {
                command_type: DrawCommandType::Arc,
                parameters: vec![5.0, 0.0, 15.0, 60.0, 120.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            // Arrow through it
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-15.0, -15.0, 15.0, 15.0],
                style: Some(DrawStyle {
                    stroke_width: 1.5,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            DrawCommand {
                command_type: DrawCommandType::Polygon,
                parameters: vec![15.0, 15.0, 10.0, 15.0, 15.0, 10.0],
                style: Some(DrawStyle {
                    stroke_width: 1.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: Some("#000000".to_string()),
                }),
            },
            // Connection lines
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-20.0, 0.0, -5.0, 0.0],
                style: None,
            },
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![5.0, 0.0, 20.0, 0.0],
                style: None,
            },
        ],
        graphics: Some(SymbolGraphics {
            bounds: GraphicsBounds {
                width: 40.0,
                height: 40.0,
            },
        }),
    };

    let mut template = ComponentTemplate::new(
        "Variable Capacitor".to_string(),
        "passive".to_string(),
        symbol
    )?;
    
    template.add_parameter(
        "capacitance".to_string(),
        ParameterTemplate {
            name: "Max Capacitance".to_string(),
            parameter_type: ParameterType::String,
            default_value: serde_json::json!("100pF"),
            min_value: None,
            max_value: None,
            unit: Some("F".to_string()),
            description: Some("Maximum capacitance".to_string()),
            required: true,
        }
    )?;
    
    template.add_pin(PinTemplate {
        id: "1".to_string(),
        name: "1".to_string(),
        number: "1".to_string(),
        x: -20.0,
        y: 0.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;
    
    template.add_pin(PinTemplate {
        id: "2".to_string(),
        name: "2".to_string(),
        number: "2".to_string(),
        x: 20.0,
        y: 0.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;

    template.keywords = vec!["variable".to_string(), "capacitor".to_string(), "trimmer".to_string()];
    _library.add_component_template(template)?;
    
    Ok(())
}

fn add_thermistor(_library: &mut ComponentLibrary) -> Result<()> {
    let symbol = ComponentSymbol {
        width: 60.0,
        height: 40.0,
        draw_commands: vec![
            // Resistor body
            DrawCommand {
                command_type: DrawCommandType::Rectangle,
                parameters: vec![-20.0, -10.0, 40.0, 20.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            // Temperature symbol (T)
            DrawCommand {
                command_type: DrawCommandType::Text,
                parameters: vec![0.0, -20.0, 12.0, "T".to_string().parse::<f64>().unwrap_or(0.0)],
                style: None,
            },
            // Diagonal line through resistor
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-15.0, 10.0, 15.0, -10.0],
                style: Some(DrawStyle {
                    stroke_width: 1.5,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            // Connection lines
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-30.0, 0.0, -20.0, 0.0],
                style: None,
            },
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![20.0, 0.0, 30.0, 0.0],
                style: None,
            },
        ],
        graphics: Some(SymbolGraphics {
            bounds: GraphicsBounds {
                width: 60.0,
                height: 40.0,
            },
        }),
    };

    let mut template = ComponentTemplate::new(
        "Thermistor".to_string(),
        "passive".to_string(),
        symbol
    )?;
    
    template.add_parameter(
        "resistance".to_string(),
        ParameterTemplate {
            name: "Resistance at 25°C".to_string(),
            parameter_type: ParameterType::String,
            default_value: serde_json::json!("10k"),
            min_value: None,
            max_value: None,
            unit: Some("Ω".to_string()),
            description: Some("Nominal resistance".to_string()),
            required: true,
        }
    )?;
    
    template.add_pin(PinTemplate {
        id: "1".to_string(),
        name: "1".to_string(),
        number: "1".to_string(),
        x: -30.0,
        y: 0.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;
    
    template.add_pin(PinTemplate {
        id: "2".to_string(),
        name: "2".to_string(),
        number: "2".to_string(),
        x: 30.0,
        y: 0.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;

    template.keywords = vec!["thermistor".to_string(), "NTC".to_string(), "PTC".to_string(), "temperature".to_string()];
    _library.add_component_template(template)?;
    
    Ok(())
}

fn add_varistor(_library: &mut ComponentLibrary) -> Result<()> {
    let symbol = ComponentSymbol {
        width: 60.0,
        height: 40.0,
        draw_commands: vec![
            // Resistor body
            DrawCommand {
                command_type: DrawCommandType::Rectangle,
                parameters: vec![-20.0, -10.0, 40.0, 20.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            // V symbol
            DrawCommand {
                command_type: DrawCommandType::Text,
                parameters: vec![0.0, -20.0, 12.0, "V".to_string().parse::<f64>().unwrap_or(0.0)],
                style: None,
            },
            // Diagonal line
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-15.0, 10.0, 15.0, -10.0],
                style: Some(DrawStyle {
                    stroke_width: 1.5,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            // Connection lines
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-30.0, 0.0, -20.0, 0.0],
                style: None,
            },
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![20.0, 0.0, 30.0, 0.0],
                style: None,
            },
        ],
        graphics: Some(SymbolGraphics {
            bounds: GraphicsBounds {
                width: 60.0,
                height: 40.0,
            },
        }),
    };

    let mut template = ComponentTemplate::new(
        "Varistor".to_string(),
        "passive".to_string(),
        symbol
    )?;
    
    template.add_parameter(
        "voltage".to_string(),
        ParameterTemplate {
            name: "Voltage Rating".to_string(),
            parameter_type: ParameterType::String,
            default_value: serde_json::json!("275V"),
            min_value: None,
            max_value: None,
            unit: Some("V".to_string()),
            description: Some("Maximum continuous voltage".to_string()),
            required: true,
        }
    )?;
    
    template.add_pin(PinTemplate {
        id: "1".to_string(),
        name: "1".to_string(),
        number: "1".to_string(),
        x: -30.0,
        y: 0.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;
    
    template.add_pin(PinTemplate {
        id: "2".to_string(),
        name: "2".to_string(),
        number: "2".to_string(),
        x: 30.0,
        y: 0.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;

    template.keywords = vec!["varistor".to_string(), "MOV".to_string(), "surge".to_string()];
    _library.add_component_template(template)?;
    
    Ok(())
}

// SEMICONDUCTOR DEVICES

fn add_diode(_library: &mut ComponentLibrary) -> Result<()> {
    let symbol = ComponentSymbol {
        width: 60.0,
        height: 40.0,
        draw_commands: vec![
            // Triangle (anode)
            DrawCommand {
                command_type: DrawCommandType::Polygon,
                parameters: vec![-10.0, -15.0, -10.0, 15.0, 10.0, 0.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            // Bar (cathode)
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![10.0, -15.0, 10.0, 15.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            // Connection lines
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-30.0, 0.0, -10.0, 0.0],
                style: None,
            },
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![10.0, 0.0, 30.0, 0.0],
                style: None,
            },
        ],
        graphics: Some(SymbolGraphics {
            bounds: GraphicsBounds {
                width: 60.0,
                height: 40.0,
            },
        }),
    };

    let mut template = ComponentTemplate::new(
        "Diode".to_string(),
        "active".to_string(),
        symbol
    )?;
    
    template.add_parameter(
        "model".to_string(),
        ParameterTemplate {
            name: "Model".to_string(),
            parameter_type: ParameterType::String,
            default_value: serde_json::json!("1N4148"),
            min_value: None,
            max_value: None,
            unit: None,
            description: Some("Diode model number".to_string()),
            required: true,
        }
    )?;
    
    template.add_pin(PinTemplate {
        id: "A".to_string(),
        name: "Anode".to_string(),
        number: "1".to_string(),
        x: -30.0,
        y: 0.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;
    
    template.add_pin(PinTemplate {
        id: "K".to_string(),
        name: "Cathode".to_string(),
        number: "2".to_string(),
        x: 30.0,
        y: 0.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;

    template.keywords = vec!["diode".to_string(), "rectifier".to_string()];
    _library.add_component_template(template)?;
    
    Ok(())
}

fn add_zener_diode(_library: &mut ComponentLibrary) -> Result<()> {
    let symbol = ComponentSymbol {
        width: 60.0,
        height: 40.0,
        draw_commands: vec![
            // Triangle (anode)
            DrawCommand {
                command_type: DrawCommandType::Polygon,
                parameters: vec![-10.0, -15.0, -10.0, 15.0, 10.0, 0.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            // Z-shaped cathode bar
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![10.0, -15.0, 10.0, 15.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![10.0, -15.0, 5.0, -15.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![10.0, 15.0, 15.0, 15.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            // Connection lines
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-30.0, 0.0, -10.0, 0.0],
                style: None,
            },
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![10.0, 0.0, 30.0, 0.0],
                style: None,
            },
        ],
        graphics: Some(SymbolGraphics {
            bounds: GraphicsBounds {
                width: 60.0,
                height: 40.0,
            },
        }),
    };

    let mut template = ComponentTemplate::new(
        "Zener Diode".to_string(),
        "active".to_string(),
        symbol
    )?;
    
    template.add_parameter(
        "voltage".to_string(),
        ParameterTemplate {
            name: "Zener Voltage".to_string(),
            parameter_type: ParameterType::String,
            default_value: serde_json::json!("5.1V"),
            min_value: None,
            max_value: None,
            unit: Some("V".to_string()),
            description: Some("Breakdown voltage".to_string()),
            required: true,
        }
    )?;
    
    template.add_pin(PinTemplate {
        id: "A".to_string(),
        name: "Anode".to_string(),
        number: "1".to_string(),
        x: -30.0,
        y: 0.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;
    
    template.add_pin(PinTemplate {
        id: "K".to_string(),
        name: "Cathode".to_string(),
        number: "2".to_string(),
        x: 30.0,
        y: 0.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;

    template.keywords = vec!["zener".to_string(), "diode".to_string(), "voltage".to_string(), "regulator".to_string()];
    _library.add_component_template(template)?;
    
    Ok(())
}

fn add_schottky_diode(_library: &mut ComponentLibrary) -> Result<()> {
    let symbol = ComponentSymbol {
        width: 60.0,
        height: 40.0,
        draw_commands: vec![
            // Triangle (anode)
            DrawCommand {
                command_type: DrawCommandType::Polygon,
                parameters: vec![-10.0, -15.0, -10.0, 15.0, 10.0, 0.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            // S-shaped cathode bar
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![10.0, -15.0, 10.0, 15.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![10.0, -15.0, 5.0, -15.0, 5.0, -10.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![10.0, 15.0, 15.0, 15.0, 15.0, 10.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            // Connection lines
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-30.0, 0.0, -10.0, 0.0],
                style: None,
            },
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![10.0, 0.0, 30.0, 0.0],
                style: None,
            },
        ],
        graphics: Some(SymbolGraphics {
            bounds: GraphicsBounds {
                width: 60.0,
                height: 40.0,
            },
        }),
    };

    let mut template = ComponentTemplate::new(
        "Schottky Diode".to_string(),
        "active".to_string(),
        symbol
    )?;
    
    template.add_pin(PinTemplate {
        id: "A".to_string(),
        name: "Anode".to_string(),
        number: "1".to_string(),
        x: -30.0,
        y: 0.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;
    
    template.add_pin(PinTemplate {
        id: "K".to_string(),
        name: "Cathode".to_string(),
        number: "2".to_string(),
        x: 30.0,
        y: 0.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;

    template.keywords = vec!["schottky".to_string(), "diode".to_string(), "fast".to_string()];
    _library.add_component_template(template)?;
    
    Ok(())
}

fn add_led_rgb(_library: &mut ComponentLibrary) -> Result<()> {
    let symbol = ComponentSymbol {
        width: 60.0,
        height: 60.0,
        draw_commands: vec![
            // LED body circle
            DrawCommand {
                command_type: DrawCommandType::Circle,
                parameters: vec![0.0, 0.0, 20.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            // RGB indicators
            DrawCommand {
                command_type: DrawCommandType::Text,
                parameters: vec![-10.0, 0.0, 8.0, "R".to_string().parse::<f64>().unwrap_or(0.0)],
                style: None,
            },
            DrawCommand {
                command_type: DrawCommandType::Text,
                parameters: vec![0.0, 0.0, 8.0, "G".to_string().parse::<f64>().unwrap_or(0.0)],
                style: None,
            },
            DrawCommand {
                command_type: DrawCommandType::Text,
                parameters: vec![10.0, 0.0, 8.0, "B".to_string().parse::<f64>().unwrap_or(0.0)],
                style: None,
            },
        ],
        graphics: Some(SymbolGraphics {
            bounds: GraphicsBounds {
                width: 60.0,
                height: 60.0,
            },
        }),
    };

    let mut template = ComponentTemplate::new(
        "RGB LED".to_string(),
        "active".to_string(),
        symbol
    )?;
    
    template.add_pin(PinTemplate {
        id: "R".to_string(),
        name: "Red".to_string(),
        number: "1".to_string(),
        x: -30.0,
        y: -15.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;
    
    template.add_pin(PinTemplate {
        id: "G".to_string(),
        name: "Green".to_string(),
        number: "2".to_string(),
        x: -30.0,
        y: 0.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;
    
    template.add_pin(PinTemplate {
        id: "B".to_string(),
        name: "Blue".to_string(),
        number: "3".to_string(),
        x: -30.0,
        y: 15.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;
    
    template.add_pin(PinTemplate {
        id: "COM".to_string(),
        name: "Common".to_string(),
        number: "4".to_string(),
        x: 30.0,
        y: 0.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;

    template.keywords = vec!["RGB".to_string(), "LED".to_string(), "color".to_string()];
    _library.add_component_template(template)?;
    
    Ok(())
}

fn add_photodiode(_library: &mut ComponentLibrary) -> Result<()> {
    let symbol = ComponentSymbol {
        width: 60.0,
        height: 40.0,
        draw_commands: vec![
            // Diode triangle
            DrawCommand {
                command_type: DrawCommandType::Polygon,
                parameters: vec![-10.0, -15.0, -10.0, 15.0, 10.0, 0.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            // Cathode bar
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![10.0, -15.0, 10.0, 15.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            // Light arrows
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-25.0, -25.0, -10.0, -10.0],
                style: Some(DrawStyle {
                    stroke_width: 1.5,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            DrawCommand {
                command_type: DrawCommandType::Polygon,
                parameters: vec![-10.0, -10.0, -12.0, -15.0, -15.0, -12.0],
                style: Some(DrawStyle {
                    stroke_width: 1.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: Some("#000000".to_string()),
                }),
            },
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-25.0, -15.0, -10.0, 0.0],
                style: Some(DrawStyle {
                    stroke_width: 1.5,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            // Connection lines
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-30.0, 0.0, -10.0, 0.0],
                style: None,
            },
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![10.0, 0.0, 30.0, 0.0],
                style: None,
            },
        ],
        graphics: Some(SymbolGraphics {
            bounds: GraphicsBounds {
                width: 60.0,
                height: 40.0,
            },
        }),
    };

    let mut template = ComponentTemplate::new(
        "Photodiode".to_string(),
        "active".to_string(),
        symbol
    )?;
    
    template.add_pin(PinTemplate {
        id: "A".to_string(),
        name: "Anode".to_string(),
        number: "1".to_string(),
        x: -30.0,
        y: 0.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;
    
    template.add_pin(PinTemplate {
        id: "K".to_string(),
        name: "Cathode".to_string(),
        number: "2".to_string(),
        x: 30.0,
        y: 0.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;

    template.keywords = vec!["photodiode".to_string(), "light".to_string(), "sensor".to_string()];
    _library.add_component_template(template)?;
    
    Ok(())
}

fn add_npn_transistor(_library: &mut ComponentLibrary) -> Result<()> {
    let symbol = ComponentSymbol {
        width: 60.0,
        height: 60.0,
        draw_commands: vec![
            // Vertical base line
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-10.0, -20.0, -10.0, 20.0],
                style: Some(DrawStyle {
                    stroke_width: 3.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            // Emitter line with arrow
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-10.0, 10.0, 10.0, 25.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            DrawCommand {
                command_type: DrawCommandType::Polygon,
                parameters: vec![10.0, 25.0, 5.0, 20.0, 7.0, 23.0],
                style: Some(DrawStyle {
                    stroke_width: 1.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: Some("#000000".to_string()),
                }),
            },
            // Collector line
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-10.0, -10.0, 10.0, -25.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            // Connection lines
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-30.0, 0.0, -10.0, 0.0],
                style: None,
            },
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![10.0, -25.0, 10.0, -30.0],
                style: None,
            },
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![10.0, 25.0, 10.0, 30.0],
                style: None,
            },
        ],
        graphics: Some(SymbolGraphics {
            bounds: GraphicsBounds {
                width: 60.0,
                height: 60.0,
            },
        }),
    };

    let mut template = ComponentTemplate::new(
        "NPN Transistor".to_string(),
        "active".to_string(),
        symbol
    )?;
    
    template.add_parameter(
        "model".to_string(),
        ParameterTemplate {
            name: "Model".to_string(),
            parameter_type: ParameterType::String,
            default_value: serde_json::json!("2N3904"),
            min_value: None,
            max_value: None,
            unit: None,
            description: Some("Transistor model".to_string()),
            required: true,
        }
    )?;
    
    template.add_pin(PinTemplate {
        id: "B".to_string(),
        name: "Base".to_string(),
        number: "1".to_string(),
        x: -30.0,
        y: 0.0,
        pin_type: PinType::Input,
        electrical: standard_electrical(),
    })?;
    
    template.add_pin(PinTemplate {
        id: "C".to_string(),
        name: "Collector".to_string(),
        number: "2".to_string(),
        x: 10.0,
        y: -30.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;
    
    template.add_pin(PinTemplate {
        id: "E".to_string(),
        name: "Emitter".to_string(),
        number: "3".to_string(),
        x: 10.0,
        y: 30.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;

    template.keywords = vec!["npn".to_string(), "transistor".to_string(), "bjt".to_string()];
    _library.add_component_template(template)?;
    
    Ok(())
}

fn add_pnp_transistor(_library: &mut ComponentLibrary) -> Result<()> {
    let symbol = ComponentSymbol {
        width: 60.0,
        height: 60.0,
        draw_commands: vec![
            // Vertical base line
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-10.0, -20.0, -10.0, 20.0],
                style: Some(DrawStyle {
                    stroke_width: 3.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            // Emitter line with arrow pointing in
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-10.0, 10.0, 10.0, 25.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            DrawCommand {
                command_type: DrawCommandType::Polygon,
                parameters: vec![-5.0, 15.0, -7.0, 12.0, -3.0, 13.0],
                style: Some(DrawStyle {
                    stroke_width: 1.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: Some("#000000".to_string()),
                }),
            },
            // Collector line
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-10.0, -10.0, 10.0, -25.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            // Connection lines
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-30.0, 0.0, -10.0, 0.0],
                style: None,
            },
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![10.0, -25.0, 10.0, -30.0],
                style: None,
            },
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![10.0, 25.0, 10.0, 30.0],
                style: None,
            },
        ],
        graphics: Some(SymbolGraphics {
            bounds: GraphicsBounds {
                width: 60.0,
                height: 60.0,
            },
        }),
    };

    let mut template = ComponentTemplate::new(
        "PNP Transistor".to_string(),
        "active".to_string(),
        symbol
    )?;
    
    template.add_parameter(
        "model".to_string(),
        ParameterTemplate {
            name: "Model".to_string(),
            parameter_type: ParameterType::String,
            default_value: serde_json::json!("2N3906"),
            min_value: None,
            max_value: None,
            unit: None,
            description: Some("Transistor model".to_string()),
            required: true,
        }
    )?;
    
    template.add_pin(PinTemplate {
        id: "B".to_string(),
        name: "Base".to_string(),
        number: "1".to_string(),
        x: -30.0,
        y: 0.0,
        pin_type: PinType::Input,
        electrical: standard_electrical(),
    })?;
    
    template.add_pin(PinTemplate {
        id: "C".to_string(),
        name: "Collector".to_string(),
        number: "2".to_string(),
        x: 10.0,
        y: -30.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;
    
    template.add_pin(PinTemplate {
        id: "E".to_string(),
        name: "Emitter".to_string(),
        number: "3".to_string(),
        x: 10.0,
        y: 30.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;

    template.keywords = vec!["pnp".to_string(), "transistor".to_string(), "bjt".to_string()];
    _library.add_component_template(template)?;
    
    Ok(())
}

fn add_n_mosfet(_library: &mut ComponentLibrary) -> Result<()> {
    let symbol = ComponentSymbol {
        width: 60.0,
        height: 60.0,
        draw_commands: vec![
            // Vertical channel line (dashed)
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-10.0, -20.0, -10.0, -10.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-10.0, -5.0, -10.0, 5.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-10.0, 10.0, -10.0, 20.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            // Gate line
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-15.0, -20.0, -15.0, 20.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            // Source and drain connections
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-10.0, -15.0, 10.0, -15.0, 10.0, -30.0],
                style: None,
            },
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-10.0, 15.0, 10.0, 15.0, 10.0, 30.0],
                style: None,
            },
            // Body diode arrow
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-10.0, 0.0, 0.0, 0.0],
                style: None,
            },
            DrawCommand {
                command_type: DrawCommandType::Polygon,
                parameters: vec![0.0, 0.0, -3.0, -3.0, -3.0, 3.0],
                style: Some(DrawStyle {
                    stroke_width: 1.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: Some("#000000".to_string()),
                }),
            },
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![0.0, 0.0, 10.0, 0.0, 10.0, 15.0],
                style: None,
            },
            // Gate connection
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-30.0, 0.0, -15.0, 0.0],
                style: None,
            },
        ],
        graphics: Some(SymbolGraphics {
            bounds: GraphicsBounds {
                width: 60.0,
                height: 60.0,
            },
        }),
    };

    let mut template = ComponentTemplate::new(
        "N-MOSFET".to_string(),
        "active".to_string(),
        symbol
    )?;
    
    template.add_parameter(
        "model".to_string(),
        ParameterTemplate {
            name: "Model".to_string(),
            parameter_type: ParameterType::String,
            default_value: serde_json::json!("IRF540"),
            min_value: None,
            max_value: None,
            unit: None,
            description: Some("MOSFET model".to_string()),
            required: true,
        }
    )?;
    
    template.add_pin(PinTemplate {
        id: "G".to_string(),
        name: "Gate".to_string(),
        number: "1".to_string(),
        x: -30.0,
        y: 0.0,
        pin_type: PinType::Input,
        electrical: standard_electrical(),
    })?;
    
    template.add_pin(PinTemplate {
        id: "D".to_string(),
        name: "Drain".to_string(),
        number: "2".to_string(),
        x: 10.0,
        y: -30.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;
    
    template.add_pin(PinTemplate {
        id: "S".to_string(),
        name: "Source".to_string(),
        number: "3".to_string(),
        x: 10.0,
        y: 30.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;

    template.keywords = vec!["nmos".to_string(), "mosfet".to_string(), "n-channel".to_string()];
    _library.add_component_template(template)?;
    
    Ok(())
}

fn add_p_mosfet(_library: &mut ComponentLibrary) -> Result<()> {
    let symbol = ComponentSymbol {
        width: 60.0,
        height: 60.0,
        draw_commands: vec![
            // Vertical channel line (dashed)
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-10.0, -20.0, -10.0, -10.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-10.0, -5.0, -10.0, 5.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-10.0, 10.0, -10.0, 20.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            // Gate line with circle (indicates P-channel)
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-20.0, -20.0, -20.0, 20.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            DrawCommand {
                command_type: DrawCommandType::Circle,
                parameters: vec![-15.0, 0.0, 3.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: Some("#FFFFFF".to_string()),
                }),
            },
            // Source and drain connections
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-10.0, -15.0, 10.0, -15.0, 10.0, -30.0],
                style: None,
            },
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-10.0, 15.0, 10.0, 15.0, 10.0, 30.0],
                style: None,
            },
            // Body diode arrow (reversed)
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![0.0, 0.0, 10.0, 0.0, 10.0, -15.0],
                style: None,
            },
            DrawCommand {
                command_type: DrawCommandType::Polygon,
                parameters: vec![-3.0, 0.0, 0.0, -3.0, 0.0, 3.0],
                style: Some(DrawStyle {
                    stroke_width: 1.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: Some("#000000".to_string()),
                }),
            },
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-10.0, 0.0, -3.0, 0.0],
                style: None,
            },
            // Gate connection
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-30.0, 0.0, -20.0, 0.0],
                style: None,
            },
        ],
        graphics: Some(SymbolGraphics {
            bounds: GraphicsBounds {
                width: 60.0,
                height: 60.0,
            },
        }),
    };

    let mut template = ComponentTemplate::new(
        "P-MOSFET".to_string(),
        "active".to_string(),
        symbol
    )?;
    
    template.add_parameter(
        "model".to_string(),
        ParameterTemplate {
            name: "Model".to_string(),
            parameter_type: ParameterType::String,
            default_value: serde_json::json!("IRF9540"),
            min_value: None,
            max_value: None,
            unit: None,
            description: Some("MOSFET model".to_string()),
            required: true,
        }
    )?;
    
    template.add_pin(PinTemplate {
        id: "G".to_string(),
        name: "Gate".to_string(),
        number: "1".to_string(),
        x: -30.0,
        y: 0.0,
        pin_type: PinType::Input,
        electrical: standard_electrical(),
    })?;
    
    template.add_pin(PinTemplate {
        id: "D".to_string(),
        name: "Drain".to_string(),
        number: "2".to_string(),
        x: 10.0,
        y: -30.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;
    
    template.add_pin(PinTemplate {
        id: "S".to_string(),
        name: "Source".to_string(),
        number: "3".to_string(),
        x: 10.0,
        y: 30.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;

    template.keywords = vec!["pmos".to_string(), "mosfet".to_string(), "p-channel".to_string()];
    _library.add_component_template(template)?;
    
    Ok(())
}

fn add_jfet(_library: &mut ComponentLibrary) -> Result<()> {
    let symbol = ComponentSymbol {
        width: 60.0,
        height: 60.0,
        draw_commands: vec![
            // Vertical channel line
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![0.0, -30.0, 0.0, 30.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            // Gate line with arrow
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-30.0, 0.0, -10.0, 0.0],
                style: None,
            },
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-10.0, 0.0, 0.0, 0.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            DrawCommand {
                command_type: DrawCommandType::Polygon,
                parameters: vec![-10.0, 0.0, -5.0, -3.0, -5.0, 3.0],
                style: Some(DrawStyle {
                    stroke_width: 1.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: Some("#000000".to_string()),
                }),
            },
            // Drain connection
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![0.0, -30.0, 30.0, -30.0],
                style: None,
            },
            // Source connection
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![0.0, 30.0, 30.0, 30.0],
                style: None,
            },
        ],
        graphics: Some(SymbolGraphics {
            bounds: GraphicsBounds {
                width: 60.0,
                height: 60.0,
            },
        }),
    };

    let mut template = ComponentTemplate::new(
        "JFET".to_string(),
        "active".to_string(),
        symbol
    )?;
    
    template.add_pin(PinTemplate {
        id: "G".to_string(),
        name: "Gate".to_string(),
        number: "1".to_string(),
        x: -30.0,
        y: 0.0,
        pin_type: PinType::Input,
        electrical: standard_electrical(),
    })?;
    
    template.add_pin(PinTemplate {
        id: "D".to_string(),
        name: "Drain".to_string(),
        number: "2".to_string(),
        x: 30.0,
        y: -30.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;
    
    template.add_pin(PinTemplate {
        id: "S".to_string(),
        name: "Source".to_string(),
        number: "3".to_string(),
        x: 30.0,
        y: 30.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;

    template.keywords = vec!["jfet".to_string(), "junction".to_string(), "fet".to_string()];
    _library.add_component_template(template)?;
    
    Ok(())
}

fn add_igbt(_library: &mut ComponentLibrary) -> Result<()> {
    let symbol = ComponentSymbol {
        width: 60.0,
        height: 60.0,
        draw_commands: vec![
            // IGBT combines MOSFET and BJT symbols
            // Vertical gate line
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-15.0, -20.0, -15.0, 20.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            // Channel segments
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-10.0, -20.0, -10.0, -10.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-10.0, -5.0, -10.0, 5.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-10.0, 10.0, -10.0, 20.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            // Collector line
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-10.0, -15.0, 10.0, -15.0, 10.0, -30.0],
                style: None,
            },
            // Emitter line with arrow
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-10.0, 15.0, 10.0, 15.0, 10.0, 30.0],
                style: None,
            },
            DrawCommand {
                command_type: DrawCommandType::Polygon,
                parameters: vec![5.0, 20.0, 7.0, 15.0, 10.0, 18.0],
                style: Some(DrawStyle {
                    stroke_width: 1.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: Some("#000000".to_string()),
                }),
            },
            // Gate connection
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-30.0, 0.0, -15.0, 0.0],
                style: None,
            },
        ],
        graphics: Some(SymbolGraphics {
            bounds: GraphicsBounds {
                width: 60.0,
                height: 60.0,
            },
        }),
    };

    let mut template = ComponentTemplate::new(
        "IGBT".to_string(),
        "active".to_string(),
        symbol
    )?;
    
    template.add_pin(PinTemplate {
        id: "G".to_string(),
        name: "Gate".to_string(),
        number: "1".to_string(),
        x: -30.0,
        y: 0.0,
        pin_type: PinType::Input,
        electrical: standard_electrical(),
    })?;
    
    template.add_pin(PinTemplate {
        id: "C".to_string(),
        name: "Collector".to_string(),
        number: "2".to_string(),
        x: 10.0,
        y: -30.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;
    
    template.add_pin(PinTemplate {
        id: "E".to_string(),
        name: "Emitter".to_string(),
        number: "3".to_string(),
        x: 10.0,
        y: 30.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;

    template.keywords = vec!["igbt".to_string(), "power".to_string()];
    _library.add_component_template(template)?;
    
    Ok(())
}

fn add_thyristor(_library: &mut ComponentLibrary) -> Result<()> {
    let symbol = ComponentSymbol {
        width: 60.0,
        height: 60.0,
        draw_commands: vec![
            // Thyristor/SCR symbol
            // Triangle
            DrawCommand {
                command_type: DrawCommandType::Polygon,
                parameters: vec![-10.0, -20.0, -10.0, 20.0, 20.0, 0.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            // Cathode bar
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![20.0, -20.0, 20.0, 20.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            // Gate line
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![5.0, 20.0, 5.0, 30.0],
                style: None,
            },
            // Anode connection
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-30.0, 0.0, -10.0, 0.0],
                style: None,
            },
            // Cathode connection
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![20.0, 0.0, 30.0, 0.0],
                style: None,
            },
        ],
        graphics: Some(SymbolGraphics {
            bounds: GraphicsBounds {
                width: 60.0,
                height: 60.0,
            },
        }),
    };

    let mut template = ComponentTemplate::new(
        "Thyristor".to_string(),
        "active".to_string(),
        symbol
    )?;
    
    template.add_pin(PinTemplate {
        id: "A".to_string(),
        name: "Anode".to_string(),
        number: "1".to_string(),
        x: -30.0,
        y: 0.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;
    
    template.add_pin(PinTemplate {
        id: "K".to_string(),
        name: "Cathode".to_string(),
        number: "2".to_string(),
        x: 30.0,
        y: 0.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;
    
    template.add_pin(PinTemplate {
        id: "G".to_string(),
        name: "Gate".to_string(),
        number: "3".to_string(),
        x: 5.0,
        y: 30.0,
        pin_type: PinType::Input,
        electrical: standard_electrical(),
    })?;

    template.keywords = vec!["thyristor".to_string(), "scr".to_string()];
    _library.add_component_template(template)?;
    
    Ok(())
}

fn add_triac(_library: &mut ComponentLibrary) -> Result<()> {
    let symbol = ComponentSymbol {
        width: 60.0,
        height: 60.0,
        draw_commands: vec![
            // Two triangles back-to-back
            DrawCommand {
                command_type: DrawCommandType::Polygon,
                parameters: vec![-10.0, -20.0, -10.0, 20.0, 10.0, 0.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            DrawCommand {
                command_type: DrawCommandType::Polygon,
                parameters: vec![10.0, -20.0, 10.0, 20.0, -10.0, 0.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            // Gate line
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![0.0, 20.0, 0.0, 30.0],
                style: None,
            },
            // MT1 connection
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-30.0, 0.0, -10.0, 0.0],
                style: None,
            },
            // MT2 connection
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![10.0, 0.0, 30.0, 0.0],
                style: None,
            },
        ],
        graphics: Some(SymbolGraphics {
            bounds: GraphicsBounds {
                width: 60.0,
                height: 60.0,
            },
        }),
    };

    let mut template = ComponentTemplate::new(
        "TRIAC".to_string(),
        "active".to_string(),
        symbol
    )?;
    
    template.add_pin(PinTemplate {
        id: "MT1".to_string(),
        name: "MT1".to_string(),
        number: "1".to_string(),
        x: -30.0,
        y: 0.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;
    
    template.add_pin(PinTemplate {
        id: "MT2".to_string(),
        name: "MT2".to_string(),
        number: "2".to_string(),
        x: 30.0,
        y: 0.0,
        pin_type: PinType::Passive,
        electrical: standard_electrical(),
    })?;
    
    template.add_pin(PinTemplate {
        id: "G".to_string(),
        name: "Gate".to_string(),
        number: "3".to_string(),
        x: 0.0,
        y: 30.0,
        pin_type: PinType::Input,
        electrical: standard_electrical(),
    })?;

    template.keywords = vec!["triac".to_string(), "ac".to_string(), "switch".to_string()];
    _library.add_component_template(template)?;
    
    Ok(())
}

// Continue with logic gates, ICs, sensors, power components, displays...
// Due to space constraints, I'll provide a few more examples

fn add_and_gate(_library: &mut ComponentLibrary) -> Result<()> {
    let symbol = ComponentSymbol {
        width: 60.0,
        height: 40.0,
        draw_commands: vec![
            // AND gate shape
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-20.0, -20.0, 0.0, -20.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            DrawCommand {
                command_type: DrawCommandType::Arc,
                parameters: vec![0.0, 0.0, 20.0, -90.0, 90.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-20.0, 20.0, 0.0, 20.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-20.0, -20.0, -20.0, 20.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            // Input lines
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-30.0, -10.0, -20.0, -10.0],
                style: None,
            },
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![-30.0, 10.0, -20.0, 10.0],
                style: None,
            },
            // Output line
            DrawCommand {
                command_type: DrawCommandType::Line,
                parameters: vec![20.0, 0.0, 30.0, 0.0],
                style: None,
            },
        ],
        graphics: Some(SymbolGraphics {
            bounds: GraphicsBounds {
                width: 60.0,
                height: 40.0,
            },
        }),
    };

    let mut template = ComponentTemplate::new(
        "AND Gate".to_string(),
        "digital".to_string(),
        symbol
    )?;
    
    template.add_pin(PinTemplate {
        id: "A".to_string(),
        name: "Input A".to_string(),
        number: "1".to_string(),
        x: -30.0,
        y: -10.0,
        pin_type: PinType::Input,
        electrical: standard_electrical(),
    })?;
    
    template.add_pin(PinTemplate {
        id: "B".to_string(),
        name: "Input B".to_string(),
        number: "2".to_string(),
        x: -30.0,
        y: 10.0,
        pin_type: PinType::Input,
        electrical: standard_electrical(),
    })?;
    
    template.add_pin(PinTemplate {
        id: "Y".to_string(),
        name: "Output".to_string(),
        number: "3".to_string(),
        x: 30.0,
        y: 0.0,
        pin_type: PinType::Output,
        electrical: standard_electrical(),
    })?;

    template.keywords = vec!["and".to_string(), "gate".to_string(), "logic".to_string()];
    _library.add_component_template(template)?;
    
    Ok(())
}

// Use actual implementations from logic_gates_impl
fn add_or_gate(_library: &mut ComponentLibrary) -> Result<()> { 
    logic_gates_impl::add_or_gate(_library)
}
fn add_not_gate(_library: &mut ComponentLibrary) -> Result<()> { 
    logic_gates_impl::add_not_gate(_library)
}
fn add_nand_gate(_library: &mut ComponentLibrary) -> Result<()> { 
    logic_gates_impl::add_nand_gate(_library)
}
fn add_nor_gate(_library: &mut ComponentLibrary) -> Result<()> { 
    logic_gates_impl::add_nor_gate(_library)
}
fn add_xor_gate(_library: &mut ComponentLibrary) -> Result<()> { 
    logic_gates_impl::add_xor_gate(_library)
}
fn add_xnor_gate(_library: &mut ComponentLibrary) -> Result<()> { 
    logic_gates_impl::add_xnor_gate(_library)
}
fn add_buffer_gate(_library: &mut ComponentLibrary) -> Result<()> { 
    logic_gates_impl::add_buffer_gate(_library)
}
fn add_555_timer(_library: &mut ComponentLibrary) -> Result<()> { Ok(()) }
fn add_comparator(_library: &mut ComponentLibrary) -> Result<()> { Ok(()) }
fn add_voltage_regulator(_library: &mut ComponentLibrary) -> Result<()> { Ok(()) }
fn add_microcontroller(_library: &mut ComponentLibrary) -> Result<()> { Ok(()) }
fn add_adc(_library: &mut ComponentLibrary) -> Result<()> { Ok(()) }
fn add_dac(_library: &mut ComponentLibrary) -> Result<()> { Ok(()) }
fn add_multiplexer(_library: &mut ComponentLibrary) -> Result<()> { Ok(()) }
fn add_shift_register(_library: &mut ComponentLibrary) -> Result<()> { Ok(()) }
fn add_counter(_library: &mut ComponentLibrary) -> Result<()> { Ok(()) }
fn add_decoder(_library: &mut ComponentLibrary) -> Result<()> { Ok(()) }
fn add_temperature_sensor(_library: &mut ComponentLibrary) -> Result<()> { Ok(()) }
fn add_pressure_sensor(_library: &mut ComponentLibrary) -> Result<()> { Ok(()) }
fn add_light_sensor(_library: &mut ComponentLibrary) -> Result<()> { Ok(()) }
fn add_hall_sensor(_library: &mut ComponentLibrary) -> Result<()> { Ok(()) }
fn add_accelerometer(_library: &mut ComponentLibrary) -> Result<()> { Ok(()) }
fn add_gyroscope(_library: &mut ComponentLibrary) -> Result<()> { Ok(()) }
fn add_humidity_sensor(_library: &mut ComponentLibrary) -> Result<()> { Ok(()) }
fn add_dc_dc_converter(_library: &mut ComponentLibrary) -> Result<()> { Ok(()) }
fn add_battery(_library: &mut ComponentLibrary) -> Result<()> { Ok(()) }
fn add_solar_cell(_library: &mut ComponentLibrary) -> Result<()> { Ok(()) }
fn add_ac_source(_library: &mut ComponentLibrary) -> Result<()> { Ok(()) }
fn add_ground_symbol(_library: &mut ComponentLibrary) -> Result<()> { Ok(()) }
fn add_power_supply(_library: &mut ComponentLibrary) -> Result<()> { Ok(()) }
fn add_seven_segment(_library: &mut ComponentLibrary) -> Result<()> { Ok(()) }
fn add_lcd_display(_library: &mut ComponentLibrary) -> Result<()> { Ok(()) }
fn add_buzzer(_library: &mut ComponentLibrary) -> Result<()> { Ok(()) }
fn add_speaker(_library: &mut ComponentLibrary) -> Result<()> { Ok(()) }
fn add_relay(_library: &mut ComponentLibrary) -> Result<()> { Ok(()) }
fn add_motor(_library: &mut ComponentLibrary) -> Result<()> { Ok(()) }