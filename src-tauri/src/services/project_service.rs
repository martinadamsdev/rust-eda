use crate::models::{Project, Schematic};
use crate::utils::error::{AppError, Result};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct ProjectService {
    state: Arc<RwLock<ProjectState>>,
}

struct ProjectState {
    current_project: Option<Project>,
    project_path: Option<PathBuf>,
    recent_projects: Vec<String>,
    projects_cache: HashMap<String, Project>,
}

impl ProjectService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(ProjectState {
                current_project: None,
                project_path: None,
                recent_projects: Vec::with_capacity(10),
                projects_cache: HashMap::with_capacity(10),
            })),
        }
    }

    pub async fn load_project(&self, path: &str) -> Result<Project> {
        let content = fs::read_to_string(path)
            .await
            .map_err(|e| AppError::IoError(e.to_string()))?;
        
        let project = Project::from_json(&content)?;
        
        Ok(project)
    }

    pub async fn save_project(&self, project: &Project, path: &str) -> Result<()> {
        let json = project.to_json()?;
        
        fs::write(path, json)
            .await
            .map_err(|e| AppError::IoError(e.to_string()))?;
        
        Ok(())
    }

    pub async fn get_current_project(&self) -> Option<Project> {
        let state = self.state.read().await;
        state.current_project.clone()
    }

    pub async fn set_current_project(&self, project: Project, path: PathBuf) -> Result<()> {
        let mut state = self.state.write().await;
        let path_str = path.to_string_lossy().to_string();
        
        // Atomic update of all fields
        state.project_path = Some(path);
        state.current_project = Some(project);
        Self::add_to_recent_internal(&mut state.recent_projects, path_str);
        
        Ok(())
    }

    pub async fn get_project(&self, project_id: &str) -> Result<Project> {
        let state = self.state.read().await;
        state.projects_cache
            .get(project_id)
            .cloned()
            .ok_or_else(|| AppError::ProjectNotFound(project_id.to_string()))
    }

    pub async fn add_schematic(&self, project_id: &str, schematic: Schematic) -> Result<()> {
        let mut state = self.state.write().await;
        
        // Check if current project matches first
        let is_current = state.current_project.as_ref()
            .map(|p| p.id == project_id)
            .unwrap_or(false);
        
        if let Some(project) = state.projects_cache.get_mut(project_id) {
            project.add_schematic(schematic)
                .map_err(|e| AppError::InvalidOperation(e.to_string()))?;
            
            if is_current {
                state.current_project = Some(project.clone());
            }
            
            Ok(())
        } else {
            Err(AppError::ProjectNotFound(project_id.to_string()))
        }
    }

    pub async fn update_settings(&self, project_id: &str, settings: serde_json::Value) -> Result<()> {
        let mut state = self.state.write().await;
        
        // Check if current project matches first
        let should_update_current = state.current_project.as_ref()
            .map(|p| p.id == project_id)
            .unwrap_or(false);
        
        if let Some(project) = state.projects_cache.get_mut(project_id) {
            // Update project settings
            if let Ok(new_settings) = serde_json::from_value(settings) {
                project.settings = new_settings;
                project.update_modified();
                
                if should_update_current {
                    state.current_project = Some(project.clone());
                }
                
                Ok(())
            } else {
                Err(AppError::InvalidFormat("Invalid settings format".to_string()))
            }
        } else {
            Err(AppError::ProjectNotFound(project_id.to_string()))
        }
    }

    pub async fn get_recent_projects(&self) -> Vec<String> {
        let state = self.state.read().await;
        state.recent_projects.clone()
    }

    fn add_to_recent_internal(recent_projects: &mut Vec<String>, path: String) {
        // Remove if already exists
        recent_projects.retain(|p| p != &path);
        
        // Add to front
        recent_projects.insert(0, path);
        
        // Keep only last 10
        if recent_projects.len() > 10 {
            recent_projects.truncate(10);
        }
    }

    pub async fn auto_save(&self) -> Result<()> {
        let state = self.state.read().await;
        if let (Some(ref project), Some(ref path)) = (&state.current_project, &state.project_path) {
            let project_clone = project.clone();
            let path_str = path.to_str()
                .ok_or_else(|| AppError::InvalidInput("Invalid path encoding".to_string()))?
                .to_string();
            drop(state); // Release lock before async operation
            
            self.save_project(&project_clone, &path_str).await?;
        }
        Ok(())
    }

    pub async fn export_project(&self, project: &Project, path: &str, format: ExportFormat) -> Result<()> {
        match format {
            ExportFormat::Json => {
                self.save_project(project, path).await
            }
            ExportFormat::Zip => {
                // TODO: Implement ZIP export
                Err(AppError::InvalidOperation("ZIP export not yet implemented".to_string()))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExportFormat {
    Json,
    Zip,
}

impl Default for ProjectService {
    fn default() -> Self {
        Self::new()
    }
}