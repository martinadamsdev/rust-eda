// Module declarations
pub mod commands;
pub mod models;
pub mod services;
pub mod utils;

use std::sync::Arc;
use services::{ProjectService, LibraryService};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Arc::new(ProjectService::new()))
        .manage(Arc::new(LibraryService::new()))
        .invoke_handler(tauri::generate_handler![
            // Project commands
            commands::project::create_project,
            commands::project::open_project,
            commands::project::save_project,
            commands::project::save_project_as,
            commands::project::get_recent_projects,
            commands::project::add_schematic_to_project,
            commands::project::get_project_info,
            commands::project::update_project_settings,
            
            // File commands
            commands::file::show_open_dialog,
            commands::file::show_save_dialog,
            commands::file::read_file,
            commands::file::write_file,
            commands::file::file_exists,
            commands::file::create_directory,
            commands::file::get_file_info,
            
            // Schematic commands
            commands::schematic::add_component,
            commands::schematic::update_component,
            commands::schematic::delete_component,
            commands::schematic::add_wire,
            commands::schematic::delete_wire,
            commands::schematic::move_component,
            commands::schematic::rotate_component,
            commands::schematic::generate_netlist,
            commands::schematic::validate_schematic,
            
            // Library commands
            commands::library::get_all_libraries,
            commands::library::get_library,
            commands::library::get_component_template,
            commands::library::search_components,
            commands::library::get_components_by_category,
            commands::library::get_library_categories,
            commands::library::get_all_categories,
            
            // Export commands
            commands::export::export_to_pdf,
            commands::export::export_to_svg,
            commands::export::export_netlist,
            commands::export::export_bom,
            commands::export::export_project_archive,
            
            // ERC commands
            services::run_erc_check,
            services::get_erc_rules,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}