use crate::models::{Project, Schematic};
use crate::utils::error::{AppError, Result};
use std::path::Path;
use serde::{Deserialize, Serialize};
use image::{ImageBuffer, Rgba, RgbaImage};

pub struct ExportService;

impl ExportService {
    pub async fn export_to_pdf(_schematic: &Schematic, _path: &Path) -> Result<()> {
        // TODO: Implement PDF export using a different library
        // For now, we can export to SVG and users can convert to PDF
        Err(AppError::InvalidOperation("PDF export not yet implemented. Please use SVG export and convert to PDF.".to_string()))
    }
    
    pub async fn export_to_png(schematic: &Schematic, path: &Path, width: u32, height: u32) -> Result<()> {
        // Create a new image buffer
        let mut img: RgbaImage = ImageBuffer::new(width, height);
        
        // Fill background
        for pixel in img.pixels_mut() {
            *pixel = Rgba([255, 255, 255, 255]); // White background
        }
        
        // Scale factors
        let scale_x = width as f64 / 1000.0;
        let scale_y = height as f64 / 1000.0;
        
        // Draw grid if visible
        if schematic.metadata.grid_visible {
            let grid_size = (schematic.metadata.grid_size as f64 * scale_x) as u32;
            for x in (0..width).step_by(grid_size as usize) {
                for y in 0..height {
                    let pixel = img.get_pixel_mut(x, y);
                    *pixel = Rgba([240, 240, 240, 255]); // Light gray
                }
            }
            for y in (0..height).step_by(grid_size as usize) {
                for x in 0..width {
                    let pixel = img.get_pixel_mut(x, y);
                    *pixel = Rgba([240, 240, 240, 255]); // Light gray
                }
            }
        }
        
        // Draw components
        for component in &schematic.components {
            let x = (component.x * scale_x) as u32;
            let y = (component.y * scale_y) as u32;
            
            // Draw component rectangle (simplified)
            let comp_width = 40;
            let comp_height = 20;
            
            // Draw rectangle outline
            for dx in 0..comp_width {
                if x >= comp_width/2 && x + comp_width/2 < width {
                    if y >= comp_height/2 && y + comp_height/2 < height {
                        // Top edge
                        let pixel = img.get_pixel_mut(x - comp_width/2 + dx, y - comp_height/2);
                        *pixel = Rgba([0, 0, 0, 255]);
                        // Bottom edge
                        let pixel = img.get_pixel_mut(x - comp_width/2 + dx, y + comp_height/2);
                        *pixel = Rgba([0, 0, 0, 255]);
                    }
                }
            }
            
            for dy in 0..comp_height {
                if x >= comp_width/2 && x + comp_width/2 < width {
                    if y >= comp_height/2 && y + comp_height/2 < height {
                        // Left edge
                        let pixel = img.get_pixel_mut(x - comp_width/2, y - comp_height/2 + dy);
                        *pixel = Rgba([0, 0, 0, 255]);
                        // Right edge
                        let pixel = img.get_pixel_mut(x + comp_width/2, y - comp_height/2 + dy);
                        *pixel = Rgba([0, 0, 0, 255]);
                    }
                }
            }
            
            // Draw pins
            for pin in &component.pins {
                let pin_x = (pin.x * scale_x) as u32;
                let pin_y = (pin.y * scale_y) as u32;
                
                // Draw pin as small circle (3x3 pixels)
                for dx in 0..3 {
                    for dy in 0..3 {
                        if pin_x + dx > 0 && pin_x + dx < width && 
                           pin_y + dy > 0 && pin_y + dy < height {
                            let pixel = img.get_pixel_mut(pin_x + dx - 1, pin_y + dy - 1);
                            *pixel = Rgba([255, 0, 0, 255]); // Red for pins
                        }
                    }
                }
            }
        }
        
        // Draw wires
        for wire in &schematic.wires {
            for window in wire.points.windows(2) {
                let x1 = (window[0].x * scale_x) as i32;
                let y1 = (window[0].y * scale_y) as i32;
                let x2 = (window[1].x * scale_x) as i32;
                let y2 = (window[1].y * scale_y) as i32;
                
                // Draw line using Bresenham's algorithm
                let dx = (x2 - x1).abs();
                let dy = (y2 - y1).abs();
                let sx = if x1 < x2 { 1 } else { -1 };
                let sy = if y1 < y2 { 1 } else { -1 };
                let mut err = dx - dy;
                
                let mut x = x1;
                let mut y = y1;
                
                loop {
                    if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                        let pixel = img.get_pixel_mut(x as u32, y as u32);
                        *pixel = Rgba([0, 0, 255, 255]); // Blue for wires
                    }
                    
                    if x == x2 && y == y2 {
                        break;
                    }
                    
                    let e2 = 2 * err;
                    if e2 > -dy {
                        err -= dy;
                        x += sx;
                    }
                    if e2 < dx {
                        err += dx;
                        y += sy;
                    }
                }
            }
        }
        
        // Save the image
        img.save(path)
            .map_err(|e| AppError::IoError(e.to_string()))?;
        
        Ok(())
    }

    pub async fn export_to_svg(schematic: &Schematic, path: &Path) -> Result<String> {
        let mut svg = String::new();
        
        // SVG header with proper namespace and viewport
        svg.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
        svg.push_str("\n");
        svg.push_str(r#"<svg xmlns="http://www.w3.org/2000/svg" "#);
        svg.push_str(r#"xmlns:xlink="http://www.w3.org/1999/xlink" "#);
        svg.push_str(r#"width="1000" height="1000" viewBox="0 0 1000 1000">"#);
        svg.push_str("\n");
        
        // Add title
        svg.push_str(&format!("  <title>{}</title>\n", schematic.name));
        svg.push_str(&format!("  <desc>Schematic generated by Rust EDA</desc>\n"));
        
        // Add grid if visible
        if schematic.metadata.grid_visible {
            svg.push_str(&Self::generate_grid_svg(schematic.metadata.grid_size));
        }
        
        // Group for components
        svg.push_str("  <g id=\"components\">\n");
        for component in &schematic.components {
            svg.push_str(&format!(
                r#"    <g transform="translate({}, {}) rotate({} 0 0)" id="{}">"#,
                component.x, component.y, component.rotation, component.id
            ));
            svg.push_str("\n");
            
            // Component body with proper styling
            svg.push_str(r#"      <rect x="-20" y="-10" width="40" height="20" "#);
            svg.push_str(r#"fill="white" stroke="black" stroke-width="1.5"/>"#);
            svg.push_str("\n");
            
            // Reference text
            svg.push_str(&format!(
                r#"      <text x="0" y="-15" text-anchor="middle" font-family="Arial" font-size="10" fill="black">{}</text>"#,
                component.reference
            ));
            svg.push_str("\n");
            
            // Value text
            svg.push_str(&format!(
                r#"      <text x="0" y="25" text-anchor="middle" font-family="Arial" font-size="10" fill="black">{}</text>"#,
                component.value
            ));
            svg.push_str("\n");
            
            // Pins
            for pin in &component.pins {
                svg.push_str(&format!(
                    r#"      <circle cx="{}" cy="{}" r="2" fill="red" stroke="darkred" stroke-width="0.5"/>"#,
                    pin.x - component.x, pin.y - component.y
                ));
                svg.push_str("\n");
            }
            
            svg.push_str("    </g>\n");
        }
        svg.push_str("  </g>\n");
        
        // Group for wires
        svg.push_str("  <g id=\"wires\">\n");
        for wire in &schematic.wires {
            svg.push_str(r#"    <polyline points=""#);
            for point in &wire.points {
                svg.push_str(&format!("{},{} ", point.x, point.y));
            }
            svg.push_str(r#"" fill="none" stroke="blue" stroke-width="1.5" stroke-linejoin="round"/>"#);
            svg.push_str("\n");
            
            // Add junction points at wire endpoints
            if let Some(first) = wire.points.first() {
                svg.push_str(&format!(
                    r#"    <circle cx="{}" cy="{}" r="2" fill="blue"/>"#,
                    first.x, first.y
                ));
                svg.push_str("\n");
            }
            if let Some(last) = wire.points.last() {
                svg.push_str(&format!(
                    r#"    <circle cx="{}" cy="{}" r="2" fill="blue"/>"#,
                    last.x, last.y
                ));
                svg.push_str("\n");
            }
        }
        svg.push_str("  </g>\n");
        
        svg.push_str("</svg>");
        
        // Write to file
        tokio::fs::write(path, &svg)
            .await
            .map_err(|e| AppError::IoError(e.to_string()))?;
        
        Ok(svg)
    }

    fn generate_grid_svg(grid_size: u32) -> String {
        let mut grid = String::new();
        grid.push_str("  <defs>\n");
        grid.push_str(&format!(r#"    <pattern id="grid" width="{}" height="{}" patternUnits="userSpaceOnUse">"#, 
            grid_size, grid_size));
        grid.push_str("\n");
        grid.push_str(&format!("      <path d=\"M {} 0 L 0 0 0 {}\" fill=\"none\" stroke=\"#e0e0e0\" stroke-width=\"0.5\"/>",
            grid_size, grid_size));
        grid.push_str("\n");
        grid.push_str("    </pattern>\n");
        grid.push_str("  </defs>\n");
        grid.push_str(r#"  <rect width="100%" height="100%" fill="url(#grid)"/>"#);
        grid.push_str("\n");
        grid
    }

    pub async fn export_netlist(schematic: &Schematic, path: &Path, format: NetlistFormat) -> Result<()> {
        let netlist = match format {
            NetlistFormat::Spice => Self::generate_spice_netlist(schematic),
            NetlistFormat::Verilog => Self::generate_verilog_netlist(schematic),
            NetlistFormat::KiCad => Self::generate_kicad_netlist(schematic),
        };
        
        tokio::fs::write(path, netlist)
            .await
            .map_err(|e| AppError::IoError(e.to_string()))?;
        
        Ok(())
    }

    fn generate_spice_netlist(schematic: &Schematic) -> String {
        let mut netlist = String::new();
        
        netlist.push_str("* SPICE Netlist Generated by Rust EDA\n");
        netlist.push_str(&format!("* Schematic: {}\n", schematic.name));
        netlist.push_str(&format!("* Date: {}\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S")));
        netlist.push_str("*\n");
        
        // Build net connections
        let mut net_counter = 1;
        let mut net_map = std::collections::HashMap::new();
        
        // Analyze wires and create net names
        for _wire in &schematic.wires {
            // Generate net name since Wire doesn't have net_name field
            let net_name = format!("N{}", net_counter);
            
            if !net_map.contains_key(&net_name) {
                net_map.insert(net_name.clone(), net_counter);
                net_counter += 1;
            }
        }
        
        // Add components with proper SPICE syntax
        for component in &schematic.components {
            let comp_type = component.reference.chars().next().unwrap_or('X');
            
            match comp_type {
                'R' => {
                    // Resistor: Rxxx n1 n2 value
                    netlist.push_str(&format!("{} ", component.reference));
                    for (i, _pin) in component.pins.iter().enumerate() {
                        netlist.push_str(&format!("N{} ", i + 1));
                    }
                    netlist.push_str(&format!("{}\n", component.value));
                },
                'C' => {
                    // Capacitor: Cxxx n1 n2 value
                    netlist.push_str(&format!("{} ", component.reference));
                    for (i, _pin) in component.pins.iter().enumerate() {
                        netlist.push_str(&format!("N{} ", i + 1));
                    }
                    netlist.push_str(&format!("{}\n", component.value));
                },
                'L' => {
                    // Inductor: Lxxx n1 n2 value
                    netlist.push_str(&format!("{} ", component.reference));
                    for (i, _pin) in component.pins.iter().enumerate() {
                        netlist.push_str(&format!("N{} ", i + 1));
                    }
                    netlist.push_str(&format!("{}\n", component.value));
                },
                'D' => {
                    // Diode: Dxxx n+ n- model
                    netlist.push_str(&format!("{} ", component.reference));
                    for (i, _pin) in component.pins.iter().enumerate() {
                        netlist.push_str(&format!("N{} ", i + 1));
                    }
                    netlist.push_str("DIODE\n");
                },
                'Q' => {
                    // Transistor: Qxxx nc nb ne model
                    netlist.push_str(&format!("{} ", component.reference));
                    for (i, _pin) in component.pins.iter().enumerate() {
                        netlist.push_str(&format!("N{} ", i + 1));
                    }
                    netlist.push_str("NPN\n");
                },
                'M' => {
                    // MOSFET: Mxxx nd ng ns nb model
                    netlist.push_str(&format!("{} ", component.reference));
                    for (i, _pin) in component.pins.iter().enumerate() {
                        netlist.push_str(&format!("N{} ", i + 1));
                    }
                    netlist.push_str("NMOS\n");
                },
                'V' => {
                    // Voltage source: Vxxx n+ n- value
                    netlist.push_str(&format!("{} ", component.reference));
                    for (i, _pin) in component.pins.iter().enumerate() {
                        netlist.push_str(&format!("N{} ", i + 1));
                    }
                    netlist.push_str(&format!("{}\n", component.value));
                },
                'I' => {
                    // Current source: Ixxx n+ n- value
                    netlist.push_str(&format!("{} ", component.reference));
                    for (i, _pin) in component.pins.iter().enumerate() {
                        netlist.push_str(&format!("N{} ", i + 1));
                    }
                    netlist.push_str(&format!("{}\n", component.value));
                },
                _ => {
                    // Generic component
                    netlist.push_str(&format!("X{} ", component.reference));
                    for (i, _pin) in component.pins.iter().enumerate() {
                        netlist.push_str(&format!("N{} ", i + 1));
                    }
                    netlist.push_str(&format!("{}\n", component.type_id));
                }
            }
        }
        
        netlist.push_str("\n");
        netlist.push_str("* End of netlist\n");
        netlist.push_str(".end\n");
        
        netlist
    }

    fn generate_verilog_netlist(schematic: &Schematic) -> String {
        let mut netlist = String::new();
        
        netlist.push_str("// Verilog netlist generated by Rust EDA\n");
        netlist.push_str(&format!("// Schematic: {}\n", schematic.name));
        netlist.push_str(&format!("// Date: {}\n\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S")));
        
        netlist.push_str(&format!("module {} (\n", schematic.name.replace(' ', "_")));
        netlist.push_str("  // Define module ports here\n");
        netlist.push_str(");\n\n");
        
        // Declare wires
        netlist.push_str("  // Wire declarations\n");
        for (i, _wire) in schematic.wires.iter().enumerate() {
            // Generate wire name
            let wire_name = format!("wire_{}", i);
            netlist.push_str(&format!("  wire {};\n", wire_name));
        }
        
        netlist.push_str("\n  // Component instantiations\n");
        for component in &schematic.components {
            netlist.push_str(&format!("  // {} - {}\n", component.reference, component.value));
        }
        
        netlist.push_str("\nendmodule\n");
        
        netlist
    }

    fn generate_kicad_netlist(schematic: &Schematic) -> String {
        let mut netlist = String::new();
        
        netlist.push_str("(export (version D)\n");
        netlist.push_str("  (design\n");
        netlist.push_str(&format!("    (source \"{}\")\n", schematic.name));
        netlist.push_str(&format!("    (date \"{}\")\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S")));
        netlist.push_str("    (tool \"Rust EDA\")\n");
        netlist.push_str("  )\n");
        
        netlist.push_str("  (components\n");
        for component in &schematic.components {
            netlist.push_str(&format!("    (comp (ref {})\n", component.reference));
            netlist.push_str(&format!("      (value {})\n", component.value));
            netlist.push_str("    )\n");
        }
        netlist.push_str("  )\n");
        
        netlist.push_str("  (nets\n");
        for (i, _wire) in schematic.wires.iter().enumerate() {
            // Generate net name
            let net_name = format!("Net-{}", i);
            netlist.push_str(&format!("    (net (code {}) (name \"{}\")\n", i + 1, net_name));
            netlist.push_str("    )\n");
        }
        netlist.push_str("  )\n");
        
        netlist.push_str(")\n");
        
        netlist
    }

    pub async fn export_bom(project: &Project, path: &Path, format: BomFormat) -> Result<()> {
        let bom = match format {
            BomFormat::Csv => Self::generate_csv_bom(project),
            BomFormat::Json => Self::generate_json_bom(project)?,
            BomFormat::Html => Self::generate_html_bom(project),
        };
        
        tokio::fs::write(path, bom)
            .await
            .map_err(|e| AppError::IoError(e.to_string()))?;
        
        Ok(())
    }

    fn generate_csv_bom(project: &Project) -> String {
        let mut csv = String::new();
        csv.push_str("Reference,Value,Footprint,Quantity,Description\n");
        
        // Collect all components from all schematics
        let mut component_map = std::collections::HashMap::new();
        
        for schematic in &project.schematics {
            for component in &schematic.components {
                let key = format!("{}_{}", component.value, component.type_id);
                let entry = component_map.entry(key).or_insert((
                    component.value.clone(),
                    component.type_id.clone(),
                    Vec::new(),
                    0
                ));
                entry.2.push(component.reference.clone());
                entry.3 += 1;
            }
        }
        
        // Sort by reference
        let mut sorted_components: Vec<_> = component_map.into_iter().collect();
        sorted_components.sort_by_key(|k| k.0.clone());
        
        for (_key, (value, type_id, refs, count)) in sorted_components {
            csv.push_str(&format!("\"{}\",\"{}\",\"{}\",{},\"{}\"\n", 
                refs.join(", "), 
                value, 
                type_id,
                count,
                "Component" // Description placeholder
            ));
        }
        
        csv
    }

    fn generate_json_bom(project: &Project) -> Result<String> {
        #[derive(Serialize)]
        struct BomEntry {
            references: Vec<String>,
            value: String,
            footprint: String,
            quantity: usize,
            description: String,
        }
        
        #[derive(Serialize)]
        struct Bom {
            project_name: String,
            date: String,
            total_components: usize,
            entries: Vec<BomEntry>,
        }
        
        let mut component_map = std::collections::HashMap::new();
        
        for schematic in &project.schematics {
            for component in &schematic.components {
                let key = format!("{}_{}", component.value, component.type_id);
                let entry = component_map.entry(key).or_insert(BomEntry {
                    references: Vec::new(),
                    value: component.value.clone(),
                    footprint: component.type_id.clone(),
                    quantity: 0,
                    description: "Component".to_string(),
                });
                entry.references.push(component.reference.clone());
                entry.quantity += 1;
            }
        }
        
        let entries: Vec<BomEntry> = component_map.into_values().collect();
        let total_components: usize = entries.iter().map(|e| e.quantity).sum();
        
        let bom = Bom {
            project_name: project.name.clone(),
            date: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            total_components,
            entries,
        };
        
        serde_json::to_string_pretty(&bom)
            .map_err(|e| AppError::SerializationError(e.to_string()))
    }

    fn generate_html_bom(project: &Project) -> String {
        let mut html = String::new();
        
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html lang=\"en\">\n");
        html.push_str("<head>\n");
        html.push_str("  <meta charset=\"UTF-8\">\n");
        html.push_str("  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
        html.push_str("  <title>Bill of Materials - ");
        html.push_str(&project.name);
        html.push_str("</title>\n");
        html.push_str("  <style>\n");
        html.push_str("    body { font-family: Arial, sans-serif; margin: 20px; }\n");
        html.push_str("    h1 { color: #333; }\n");
        html.push_str("    table { border-collapse: collapse; width: 100%; margin-top: 20px; }\n");
        html.push_str("    th, td { border: 1px solid #ddd; padding: 12px; text-align: left; }\n");
        html.push_str("    th { background-color: #4CAF50; color: white; }\n");
        html.push_str("    tr:nth-child(even) { background-color: #f2f2f2; }\n");
        html.push_str("    .metadata { margin: 20px 0; color: #666; }\n");
        html.push_str("    .total { font-weight: bold; background-color: #e0e0e0; }\n");
        html.push_str("  </style>\n");
        html.push_str("</head>\n");
        html.push_str("<body>\n");
        
        html.push_str(&format!("  <h1>Bill of Materials: {}</h1>\n", project.name));
        
        html.push_str("  <div class=\"metadata\">\n");
        html.push_str(&format!("    <p>Generated: {}</p>\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S")));
        html.push_str(&format!("    <p>Version: {}</p>\n", project.version));
        html.push_str(&format!("    <p>Author: {}</p>\n", project.author.as_deref().unwrap_or("Unknown")));
        html.push_str("  </div>\n");
        
        html.push_str("  <table>\n");
        html.push_str("    <thead>\n");
        html.push_str("      <tr>\n");
        html.push_str("        <th>Reference</th>\n");
        html.push_str("        <th>Value</th>\n");
        html.push_str("        <th>Footprint</th>\n");
        html.push_str("        <th>Quantity</th>\n");
        html.push_str("        <th>Description</th>\n");
        html.push_str("      </tr>\n");
        html.push_str("    </thead>\n");
        html.push_str("    <tbody>\n");
        
        // Collect and process components
        let mut component_map = std::collections::HashMap::new();
        let mut total_quantity = 0;
        
        for schematic in &project.schematics {
            for component in &schematic.components {
                let key = format!("{}_{}", component.value, component.type_id);
                let entry = component_map.entry(key).or_insert((
                    component.value.clone(),
                    component.type_id.clone(),
                    Vec::new(),
                    0
                ));
                entry.2.push(component.reference.clone());
                entry.3 += 1;
                total_quantity += 1;
            }
        }
        
        // Sort and add rows
        let mut sorted_components: Vec<_> = component_map.into_iter().collect();
        sorted_components.sort_by_key(|k| k.0.clone());
        
        for (_key, (value, footprint, refs, quantity)) in sorted_components {
            html.push_str("      <tr>\n");
            html.push_str(&format!("        <td>{}</td>\n", refs.join(", ")));
            html.push_str(&format!("        <td>{}</td>\n", value));
            html.push_str(&format!("        <td>{}</td>\n", footprint));
            html.push_str(&format!("        <td>{}</td>\n", quantity));
            html.push_str("        <td>Component</td>\n");
            html.push_str("      </tr>\n");
        }
        
        // Add total row
        html.push_str("      <tr class=\"total\">\n");
        html.push_str("        <td colspan=\"3\">Total Components</td>\n");
        html.push_str(&format!("        <td>{}</td>\n", total_quantity));
        html.push_str("        <td></td>\n");
        html.push_str("      </tr>\n");
        
        html.push_str("    </tbody>\n");
        html.push_str("  </table>\n");
        html.push_str("</body>\n");
        html.push_str("</html>");
        
        html
    }
    
    pub async fn export_to_gerber(_schematic: &Schematic, _path: &Path) -> Result<()> {
        // Gerber export is for PCB, not schematic
        Err(AppError::InvalidOperation("Gerber export is only available for PCB layouts".to_string()))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetlistFormat {
    Spice,
    Verilog,
    KiCad,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BomFormat {
    Csv,
    Json,
    Html,
}

// Note: Tauri commands are defined in commands/export.rs to avoid duplication