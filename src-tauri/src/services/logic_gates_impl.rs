use crate::models::component::{ComponentLibrary, ComponentTemplate, ComponentSymbol, DrawCommand, DrawCommandType, DrawStyle, PinTemplate, SymbolGraphics, GraphicsBounds};
use crate::models::{PinType, ElectricalType};
use crate::utils::error::Result;

fn standard_electrical() -> ElectricalType {
    ElectricalType { voltage: Some(5.0), current: Some(0.02), impedance: None }
}

pub fn add_or_gate(library: &mut ComponentLibrary) -> Result<()> {
    let symbol = ComponentSymbol {
        width: 60.0,
        height: 40.0,
        draw_commands: vec![
            // OR gate body (curved)
            DrawCommand {
                command_type: DrawCommandType::Polygon,
                parameters: vec![-30.0, -20.0, -10.0, -20.0, 10.0, -10.0, 30.0, 0.0, 10.0, 10.0, -10.0, 20.0, -30.0, 20.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: Some("#FFFFFF".to_string()),
                }),
            },
        ],
        graphics: Some(SymbolGraphics {
            bounds: GraphicsBounds { width: 60.0, height: 40.0 }
        }),
    };
    
    let mut template = ComponentTemplate::new("OR Gate".to_string(), "digital".to_string(), symbol)?;
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
    
    template.keywords = vec!["or".to_string(), "gate".to_string(), "logic".to_string()];
    library.add_component_template(template)?;
    Ok(())
}

pub fn add_not_gate(library: &mut ComponentLibrary) -> Result<()> {
    let symbol = ComponentSymbol {
        width: 50.0,
        height: 30.0,
        draw_commands: vec![
            // Triangle body
            DrawCommand {
                command_type: DrawCommandType::Polygon,
                parameters: vec![-20.0, -15.0, -20.0, 15.0, 15.0, 0.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: Some("#FFFFFF".to_string()),
                }),
            },
            // Inversion bubble
            DrawCommand {
                command_type: DrawCommandType::Circle,
                parameters: vec![20.0, 0.0, 5.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: Some("#FFFFFF".to_string()),
                }),
            },
        ],
        graphics: Some(SymbolGraphics {
            bounds: GraphicsBounds { width: 50.0, height: 30.0 }
        }),
    };
    
    let mut template = ComponentTemplate::new("NOT Gate".to_string(), "digital".to_string(), symbol)?;
    template.add_pin(PinTemplate {
        id: "A".to_string(),
        name: "Input".to_string(),
        number: "1".to_string(),
        x: -25.0,
        y: 0.0,
        pin_type: PinType::Input,
        electrical: standard_electrical(),
    })?;
    template.add_pin(PinTemplate {
        id: "Y".to_string(),
        name: "Output".to_string(),
        number: "2".to_string(),
        x: 25.0,
        y: 0.0,
        pin_type: PinType::Output,
        electrical: standard_electrical(),
    })?;
    
    template.keywords = vec!["not".to_string(), "inverter".to_string(), "gate".to_string(), "logic".to_string()];
    library.add_component_template(template)?;
    Ok(())
}

pub fn add_nand_gate(library: &mut ComponentLibrary) -> Result<()> {
    let symbol = ComponentSymbol {
        width: 60.0,
        height: 40.0,
        draw_commands: vec![
            // AND gate body with curved right side
            DrawCommand {
                command_type: DrawCommandType::Polygon,
                parameters: vec![-25.0, -20.0, 10.0, -20.0, 15.0, -15.0, 15.0, 15.0, 10.0, 20.0, -25.0, 20.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: Some("#FFFFFF".to_string()),
                }),
            },
            // Inversion bubble
            DrawCommand {
                command_type: DrawCommandType::Circle,
                parameters: vec![20.0, 0.0, 5.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: Some("#FFFFFF".to_string()),
                }),
            },
        ],
        graphics: Some(SymbolGraphics {
            bounds: GraphicsBounds { width: 60.0, height: 40.0 }
        }),
    };
    
    let mut template = ComponentTemplate::new("NAND Gate".to_string(), "digital".to_string(), symbol)?;
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
    
    template.keywords = vec!["nand".to_string(), "gate".to_string(), "logic".to_string()];
    library.add_component_template(template)?;
    Ok(())
}

pub fn add_nor_gate(library: &mut ComponentLibrary) -> Result<()> {
    let symbol = ComponentSymbol {
        width: 60.0,
        height: 40.0,
        draw_commands: vec![
            // OR gate body (curved)
            DrawCommand {
                command_type: DrawCommandType::Polygon,
                parameters: vec![-30.0, -20.0, -10.0, -20.0, 10.0, -10.0, 15.0, 0.0, 10.0, 10.0, -10.0, 20.0, -30.0, 20.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: Some("#FFFFFF".to_string()),
                }),
            },
            // Inversion bubble
            DrawCommand {
                command_type: DrawCommandType::Circle,
                parameters: vec![20.0, 0.0, 5.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: Some("#FFFFFF".to_string()),
                }),
            },
        ],
        graphics: Some(SymbolGraphics {
            bounds: GraphicsBounds { width: 60.0, height: 40.0 }
        }),
    };
    
    let mut template = ComponentTemplate::new("NOR Gate".to_string(), "digital".to_string(), symbol)?;
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
    
    template.keywords = vec!["nor".to_string(), "gate".to_string(), "logic".to_string()];
    library.add_component_template(template)?;
    Ok(())
}

pub fn add_xor_gate(library: &mut ComponentLibrary) -> Result<()> {
    let symbol = ComponentSymbol {
        width: 60.0,
        height: 40.0,
        draw_commands: vec![
            // XOR gate body (OR with extra curved input line)
            DrawCommand {
                command_type: DrawCommandType::Polygon,
                parameters: vec![-25.0, -20.0, -5.0, -20.0, 10.0, -10.0, 30.0, 0.0, 10.0, 10.0, -5.0, 20.0, -25.0, 20.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: Some("#FFFFFF".to_string()),
                }),
            },
            // Extra curved input line
            DrawCommand {
                command_type: DrawCommandType::Polygon,
                parameters: vec![-30.0, -20.0, -27.0, 0.0, -30.0, 20.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
        ],
        graphics: Some(SymbolGraphics {
            bounds: GraphicsBounds { width: 60.0, height: 40.0 }
        }),
    };
    
    let mut template = ComponentTemplate::new("XOR Gate".to_string(), "digital".to_string(), symbol)?;
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
    
    template.keywords = vec!["xor".to_string(), "exclusive or".to_string(), "gate".to_string(), "logic".to_string()];
    library.add_component_template(template)?;
    Ok(())
}

pub fn add_xnor_gate(library: &mut ComponentLibrary) -> Result<()> {
    let symbol = ComponentSymbol {
        width: 60.0,
        height: 40.0,
        draw_commands: vec![
            // XNOR gate body
            DrawCommand {
                command_type: DrawCommandType::Polygon,
                parameters: vec![-25.0, -20.0, -5.0, -20.0, 10.0, -10.0, 15.0, 0.0, 10.0, 10.0, -5.0, 20.0, -25.0, 20.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: Some("#FFFFFF".to_string()),
                }),
            },
            // Extra curved input line
            DrawCommand {
                command_type: DrawCommandType::Polygon,
                parameters: vec![-30.0, -20.0, -27.0, 0.0, -30.0, 20.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: None,
                }),
            },
            // Inversion bubble
            DrawCommand {
                command_type: DrawCommandType::Circle,
                parameters: vec![20.0, 0.0, 5.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: Some("#FFFFFF".to_string()),
                }),
            },
        ],
        graphics: Some(SymbolGraphics {
            bounds: GraphicsBounds { width: 60.0, height: 40.0 }
        }),
    };
    
    let mut template = ComponentTemplate::new("XNOR Gate".to_string(), "digital".to_string(), symbol)?;
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
    
    template.keywords = vec!["xnor".to_string(), "exclusive nor".to_string(), "gate".to_string(), "logic".to_string()];
    library.add_component_template(template)?;
    Ok(())
}

pub fn add_buffer_gate(library: &mut ComponentLibrary) -> Result<()> {
    let symbol = ComponentSymbol {
        width: 50.0,
        height: 30.0,
        draw_commands: vec![
            // Triangle body (no bubble)
            DrawCommand {
                command_type: DrawCommandType::Polygon,
                parameters: vec![-20.0, -15.0, -20.0, 15.0, 20.0, 0.0],
                style: Some(DrawStyle {
                    stroke_width: 2.0,
                    stroke_color: "#000000".to_string(),
                    fill_color: Some("#FFFFFF".to_string()),
                }),
            },
        ],
        graphics: Some(SymbolGraphics {
            bounds: GraphicsBounds { width: 50.0, height: 30.0 }
        }),
    };
    
    let mut template = ComponentTemplate::new("Buffer".to_string(), "digital".to_string(), symbol)?;
    template.add_pin(PinTemplate {
        id: "A".to_string(),
        name: "Input".to_string(),
        number: "1".to_string(),
        x: -25.0,
        y: 0.0,
        pin_type: PinType::Input,
        electrical: standard_electrical(),
    })?;
    template.add_pin(PinTemplate {
        id: "Y".to_string(),
        name: "Output".to_string(),
        number: "2".to_string(),
        x: 25.0,
        y: 0.0,
        pin_type: PinType::Output,
        electrical: standard_electrical(),
    })?;
    
    template.keywords = vec!["buffer".to_string(), "gate".to_string(), "logic".to_string()];
    library.add_component_template(template)?;
    Ok(())
}