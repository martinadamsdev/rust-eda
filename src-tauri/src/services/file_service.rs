use std::path::{Path, PathBuf};
use tokio::fs;
use crate::utils::error::{AppError, Result};

pub struct FileService;

impl FileService {
    pub async fn read_text_file(path: &Path) -> Result<String> {
        fs::read_to_string(path)
            .await
            .map_err(|e| AppError::IoError(e.to_string()))
    }

    pub async fn write_text_file(path: &Path, content: &str) -> Result<()> {
        fs::write(path, content)
            .await
            .map_err(|e| AppError::IoError(e.to_string()))
    }

    pub async fn read_binary_file(path: &Path) -> Result<Vec<u8>> {
        fs::read(path)
            .await
            .map_err(|e| AppError::IoError(e.to_string()))
    }

    pub async fn write_binary_file(path: &Path, content: &[u8]) -> Result<()> {
        fs::write(path, content)
            .await
            .map_err(|e| AppError::IoError(e.to_string()))
    }

    pub async fn create_directory(path: &Path) -> Result<()> {
        fs::create_dir_all(path)
            .await
            .map_err(|e| AppError::IoError(e.to_string()))
    }

    pub async fn file_exists(path: &Path) -> bool {
        path.exists()
    }

    pub async fn is_directory(path: &Path) -> bool {
        path.is_dir()
    }

    pub async fn list_files(path: &Path, extension: Option<&str>) -> Result<Vec<PathBuf>> {
        let mut entries = fs::read_dir(path)
            .await
            .map_err(|e| AppError::IoError(e.to_string()))?;
        
        let mut files = Vec::new();
        
        while let Some(entry) = entries.next_entry().await
            .map_err(|e| AppError::IoError(e.to_string()))? {
            let path = entry.path();
            
            if path.is_file() {
                if let Some(ext) = extension {
                    if path.extension().and_then(|s| s.to_str()) == Some(ext) {
                        files.push(path);
                    }
                } else {
                    files.push(path);
                }
            }
        }
        
        Ok(files)
    }

    pub async fn copy_file(from: &Path, to: &Path) -> Result<()> {
        fs::copy(from, to)
            .await
            .map_err(|e| AppError::IoError(e.to_string()))?;
        Ok(())
    }

    pub async fn move_file(from: &Path, to: &Path) -> Result<()> {
        fs::rename(from, to)
            .await
            .map_err(|e| AppError::IoError(e.to_string()))
    }

    pub async fn delete_file(path: &Path) -> Result<()> {
        if path.is_dir() {
            fs::remove_dir_all(path)
                .await
                .map_err(|e| AppError::IoError(e.to_string()))
        } else {
            fs::remove_file(path)
                .await
                .map_err(|e| AppError::IoError(e.to_string()))
        }
    }

    pub fn get_file_extension(path: &Path) -> Option<String> {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_lowercase())
    }

    pub fn get_file_name(path: &Path) -> Option<String> {
        path.file_name()
            .and_then(|name| name.to_str())
            .map(|s| s.to_string())
    }

    pub fn get_file_stem(path: &Path) -> Option<String> {
        path.file_stem()
            .and_then(|stem| stem.to_str())
            .map(|s| s.to_string())
    }
}