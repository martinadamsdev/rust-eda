use crate::models::{Project, Schematic};
use crate::services::project_service::ProjectService;
use tauri::State;
use std::sync::Arc;

#[tauri::command]
pub async fn create_project(name: String) -> Result<Project, String> {
    if !crate::utils::validators::validate_project_name(&name) {
        return Err("Invalid project name".to_string());
    }
    
    Project::new(name)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn open_project(
    path: String,
    project_service: State<'_, Arc<ProjectService>>
) -> Result<Project, String> {
    if !crate::utils::validators::validate_file_path(&path) {
        return Err("Invalid file path".to_string());
    }
    
    project_service.load_project(&path)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_project(
    project: Project,
    path: String,
    project_service: State<'_, Arc<ProjectService>>
) -> Result<(), String> {
    if !crate::utils::validators::validate_file_path(&path) {
        return Err("Invalid file path".to_string());
    }
    
    project_service.save_project(&project, &path)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_project_as(
    project: Project,
    new_path: String,
    project_service: State<'_, Arc<ProjectService>>
) -> Result<(), String> {
    if !crate::utils::validators::validate_file_path(&new_path) {
        return Err("Invalid file path".to_string());
    }
    
    project_service.save_project(&project, &new_path)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_recent_projects(
    project_service: State<'_, Arc<ProjectService>>
) -> Result<Vec<String>, String> {
    Ok(project_service.get_recent_projects().await)
}

#[tauri::command]
pub async fn add_schematic_to_project(
    project_id: String,
    schematic_name: String,
    project_service: State<'_, Arc<ProjectService>>
) -> Result<Schematic, String> {
    if !crate::utils::validators::validate_string_length(&schematic_name, 100) {
        return Err("Invalid schematic name".to_string());
    }
    
    let schematic = Schematic::new(schematic_name)
        .map_err(|e| e.to_string())?;
    
    project_service.add_schematic(&project_id, schematic.clone())
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(schematic)
}

#[tauri::command]
pub async fn get_project_info(
    project_id: String,
    project_service: State<'_, Arc<ProjectService>>
) -> Result<Project, String> {
    project_service.get_project(&project_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_project_settings(
    project_id: String,
    settings: serde_json::Value,
    project_service: State<'_, Arc<ProjectService>>
) -> Result<(), String> {
    project_service.update_settings(&project_id, settings)
        .await
        .map_err(|e| e.to_string())
}