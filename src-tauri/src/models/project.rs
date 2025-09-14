use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;
use crate::utils::{error::AppError, validators};
use crate::utils::error::Result;

const MAX_SCHEMATICS: usize = 100;
const MAX_METADATA_SIZE: usize = 1024 * 1024; // 1MB

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub version: String,
    pub author: Option<String>,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub schematics: Vec<super::Schematic>,
    pub settings: ProjectSettings,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectSettings {
    pub grid_size: u32,
    pub snap_to_grid: bool,
    pub auto_save: bool,
    pub auto_save_interval: u32,
    pub default_units: String,
    pub color_scheme: String,
}

impl Default for ProjectSettings {
    fn default() -> Self {
        Self {
            grid_size: 10,
            snap_to_grid: true,
            auto_save: true,
            auto_save_interval: 300, // 5 minutes
            default_units: "mm".to_string(),
            color_scheme: "light".to_string(),
        }
    }
}

impl Project {
    pub fn new(name: String) -> Result<Self> {
        if !validators::validate_string_length(&name, 100) {
            return Err(AppError::InvalidInput("Invalid project name".to_string()));
        }
        
        let now = Utc::now();
        Ok(Self {
            id: Uuid::new_v4().to_string(),
            name,
            description: None,
            version: "1.0.0".to_string(),
            author: None,
            created_at: now,
            modified_at: now,
            schematics: Vec::with_capacity(10),
            settings: ProjectSettings::default(),
            metadata: HashMap::with_capacity(10),
        })
    }
    

    pub fn add_schematic(&mut self, schematic: super::Schematic) -> Result<()> {
        if self.schematics.len() >= MAX_SCHEMATICS {
            return Err(AppError::InvalidOperation(
                format!("Maximum schematic limit ({}) exceeded", MAX_SCHEMATICS)
            ));
        }
        
        self.schematics.push(schematic);
        self.modified_at = Utc::now();
        Ok(())
    }
    

    pub fn update_modified(&mut self) {
        self.modified_at = Utc::now();
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| AppError::SerializationError(e.to_string()))
    }

    pub fn from_json(json: &str) -> Result<Self> {
        if !validators::validate_json_size(json) {
            return Err(AppError::InvalidInput("JSON size exceeds limit".to_string()));
        }
        
        serde_json::from_str(json)
            .map_err(|e| AppError::SerializationError(e.to_string()))
    }
    
    pub fn schematic_count(&self) -> usize {
        self.schematics.len()
    }
    
    pub fn can_add_schematic(&self) -> bool {
        self.schematics.len() < MAX_SCHEMATICS
    }
    
    pub fn add_metadata(&mut self, key: String, value: serde_json::Value) -> Result<()> {
        let serialized_size = serde_json::to_string(&value)
            .map_err(|e| AppError::SerializationError(e.to_string()))?
            .len();
        
        let current_size: usize = self.metadata.values()
            .map(|v| serde_json::to_string(v)
                .map(|s| s.len())
                .unwrap_or(0))
            .sum();
        
        if current_size + serialized_size > MAX_METADATA_SIZE {
            return Err(AppError::InvalidOperation(
                format!("Metadata size limit ({} bytes) exceeded", MAX_METADATA_SIZE)
            ));
        }
        
        self.metadata.insert(key, value);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_creation() {
        let project = Project::new("Test Project".to_string()).unwrap();
        assert_eq!(project.name, "Test Project");
        assert_eq!(project.version, "1.0.0");
        assert!(project.schematics.is_empty());
    }

    #[test]
    fn test_project_serialization() {
        let project = Project::new("Test Project".to_string()).unwrap();
        let json = project.to_json().unwrap();
        let deserialized = Project::from_json(&json).unwrap();
        assert_eq!(project.id, deserialized.id);
        assert_eq!(project.name, deserialized.name);
    }
}