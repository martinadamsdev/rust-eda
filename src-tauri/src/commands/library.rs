use std::sync::Arc;
use tauri::State;
use crate::models::component::{ComponentLibrary, ComponentTemplate, ComponentCategory};
use crate::services::LibraryService;
use crate::utils::error::Result;

#[tauri::command]
pub async fn get_all_libraries(
    library_service: State<'_, Arc<LibraryService>>,
) -> Result<Vec<ComponentLibrary>> {
    Ok(library_service.get_all_libraries())
}

#[tauri::command]
pub async fn get_library(
    library_id: String,
    library_service: State<'_, Arc<LibraryService>>,
) -> Result<Option<ComponentLibrary>> {
    Ok(library_service.get_library(&library_id))
}

#[tauri::command]
pub async fn get_component_template(
    library_id: String,
    component_id: String,
    library_service: State<'_, Arc<LibraryService>>,
) -> Result<Option<ComponentTemplate>> {
    Ok(library_service.get_component_template(&library_id, &component_id))
}

#[tauri::command]
pub async fn search_components(
    query: String,
    library_service: State<'_, Arc<LibraryService>>,
) -> Result<Vec<(String, ComponentTemplate)>> {
    if query.trim().is_empty() {
        return Ok(Vec::new());
    }
    
    Ok(library_service.search_all_components(&query))
}

#[tauri::command]
pub async fn get_components_by_category(
    category_id: String,
    library_service: State<'_, Arc<LibraryService>>,
) -> Result<Vec<(String, ComponentTemplate)>> {
    Ok(library_service.get_components_by_category(&category_id))
}

#[tauri::command]
pub async fn get_library_categories(
    library_id: String,
    library_service: State<'_, Arc<LibraryService>>,
) -> Result<Vec<ComponentCategory>> {
    match library_service.get_library(&library_id) {
        Some(library) => Ok(library.categories),
        None => Ok(Vec::new()),
    }
}

#[tauri::command]
pub async fn get_all_categories(
    library_service: State<'_, Arc<LibraryService>>,
) -> Result<Vec<ComponentCategory>> {
    let libraries = library_service.get_all_libraries();
    let mut all_categories = Vec::new();
    
    for library in libraries {
        all_categories.extend(library.categories);
    }
    
    // Remove duplicates by category ID
    all_categories.sort_by(|a, b| a.id.cmp(&b.id));
    all_categories.dedup_by(|a, b| a.id == b.id);
    
    Ok(all_categories)
}