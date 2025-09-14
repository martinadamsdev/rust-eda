use std::path::PathBuf;

#[tauri::command]
pub async fn show_open_dialog() -> Result<Option<PathBuf>, String> {
    // This will be handled by Tauri's dialog API on the frontend
    Ok(None)
}

#[tauri::command]
pub async fn show_save_dialog() -> Result<Option<PathBuf>, String> {
    // This will be handled by Tauri's dialog API on the frontend
    Ok(None)
}

#[tauri::command]
pub async fn read_file(path: String) -> Result<String, String> {
    if !crate::utils::validators::validate_file_path(&path) {
        return Err("Invalid file path".to_string());
    }
    
    std::fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read file: {}", e))
}

#[tauri::command]
pub async fn write_file(path: String, content: String) -> Result<(), String> {
    if !crate::utils::validators::validate_file_path(&path) {
        return Err("Invalid file path".to_string());
    }
    
    std::fs::write(&path, content)
        .map_err(|e| format!("Failed to write file: {}", e))
}

#[tauri::command]
pub async fn file_exists(path: String) -> Result<bool, String> {
    Ok(std::path::Path::new(&path).exists())
}

#[tauri::command]
pub async fn create_directory(path: String) -> Result<(), String> {
    if !crate::utils::validators::validate_file_path(&path) {
        return Err("Invalid directory path".to_string());
    }
    
    std::fs::create_dir_all(&path)
        .map_err(|e| format!("Failed to create directory: {}", e))
}

#[tauri::command]
pub async fn get_file_info(path: String) -> Result<FileInfo, String> {
    let metadata = std::fs::metadata(&path)
        .map_err(|e| format!("Failed to get file info: {}", e))?;
    
    Ok(FileInfo {
        size: metadata.len(),
        is_directory: metadata.is_dir(),
        is_file: metadata.is_file(),
        modified: metadata.modified()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs()),
    })
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileInfo {
    pub size: u64,
    pub is_directory: bool,
    pub is_file: bool,
    pub modified: Option<u64>,
}