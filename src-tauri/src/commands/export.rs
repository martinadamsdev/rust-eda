use crate::services::export_service::{ExportService, NetlistFormat, BomFormat};
use std::path::PathBuf;
use std::sync::Arc;
use tauri::State;
use crate::services::project_service::ProjectService;

#[tauri::command]
pub async fn export_to_pdf(
    schematic_id: String,
    output_path: String,
    project_service: State<'_, Arc<ProjectService>>,
) -> Result<(), String> {
    let path = PathBuf::from(output_path);
    
    // Get the schematic from current project
    let project = project_service
        .get_current_project()
        .await
        .ok_or("No project currently open")?;
    
    let schematic = project.schematics.iter()
        .find(|s| s.id == schematic_id)
        .ok_or("Schematic not found")?;
    
    ExportService::export_to_pdf(schematic, &path)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn export_to_svg(
    schematic_id: String,
    output_path: String,
    project_service: State<'_, Arc<ProjectService>>,
) -> Result<String, String> {
    let path = PathBuf::from(output_path);
    
    // Get the schematic from current project
    let project = project_service
        .get_current_project()
        .await
        .ok_or("No project currently open")?;
    
    let schematic = project.schematics.iter()
        .find(|s| s.id == schematic_id)
        .ok_or("Schematic not found")?;
    
    ExportService::export_to_svg(schematic, &path)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn export_netlist(
    schematic_id: String,
    output_path: String,
    format: String,
    project_service: State<'_, Arc<ProjectService>>,
) -> Result<(), String> {
    let netlist_format = match format.as_str() {
        "spice" => NetlistFormat::Spice,
        "verilog" => NetlistFormat::Verilog,
        "kicad" => NetlistFormat::KiCad,
        _ => return Err(format!("Unknown netlist format: {}", format)),
    };
    
    let path = PathBuf::from(output_path);
    
    // Get the schematic from current project
    let project = project_service
        .get_current_project()
        .await
        .ok_or("No project currently open")?;
    
    let schematic = project.schematics.iter()
        .find(|s| s.id == schematic_id)
        .ok_or("Schematic not found")?;
    
    ExportService::export_netlist(schematic, &path, netlist_format)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn export_bom(
    output_path: String,
    format: String,
    project_service: State<'_, Arc<ProjectService>>,
) -> Result<(), String> {
    let bom_format = match format.as_str() {
        "csv" => BomFormat::Csv,
        "json" => BomFormat::Json,
        "html" => BomFormat::Html,
        _ => return Err(format!("Unknown BOM format: {}", format)),
    };
    
    let path = PathBuf::from(output_path);
    
    // Get the current project
    let project = project_service
        .get_current_project()
        .await
        .ok_or("No project currently open")?;
    
    ExportService::export_bom(&project, &path, bom_format)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn export_project_archive(
    _output_path: String,
    _include_libraries: bool,
    project_service: State<'_, Arc<ProjectService>>,
) -> Result<(), String> {
    let _project = project_service
        .get_current_project()
        .await
        .ok_or("No project currently open")?;
    
    // TODO: Implement project archive export with all assets
    // This would create a ZIP file with the project, all schematics,
    // and optionally include component libraries
    Err("Project archive export not yet implemented".to_string())
}