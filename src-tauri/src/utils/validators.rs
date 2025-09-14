use regex::Regex;
use std::path::Path;
use std::env;
use once_cell::sync::Lazy;

pub fn validate_project_name(name: &str) -> bool {
    if name.is_empty() || name.len() > 100 {
        return false;
    }
    
    // Allow alphanumeric, spaces, hyphens, and underscores
    static PROJECT_NAME_RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"^[\w\s\-]+$").expect("Invalid regex for project name validation")
    });
    PROJECT_NAME_RE.is_match(name)
}

pub fn validate_file_path(path: &str) -> bool {
    if path.is_empty() || path.len() > 4096 {
        return false;
    }
    
    // Check for dangerous characters and patterns
    if path.contains("../") || path.contains("..\\") 
        || path.contains("\0") || path.contains("\r") || path.contains("\n") {
        return false;
    }
    
    let path_obj = Path::new(path);
    
    // Define allowed base directories for security
    let allowed_dirs = get_allowed_directories();
    
    // Canonicalize the path to resolve any remaining traversal attempts
    if let Ok(canonical) = path_obj.canonicalize() {
        // Only allow access within explicitly allowed directories
        return allowed_dirs.iter().any(|allowed_dir| {
            if let Ok(allowed_canonical) = allowed_dir.canonicalize() {
                canonical.starts_with(&allowed_canonical)
            } else {
                false
            }
        });
    }
    
    // If canonicalization fails, check if it's a relative path within current directory
    if path_obj.is_relative() {
        if let Ok(current_dir) = env::current_dir() {
            if let Ok(full_path) = current_dir.join(path_obj).canonicalize() {
                return allowed_dirs.iter().any(|allowed_dir| {
                    if let Ok(allowed_canonical) = allowed_dir.canonicalize() {
                        full_path.starts_with(&allowed_canonical)
                    } else {
                        false
                    }
                });
            }
        }
    }
    
    false
}

fn get_allowed_directories() -> Vec<std::path::PathBuf> {
    let mut allowed = Vec::new();
    
    // Allow current working directory
    if let Ok(current_dir) = env::current_dir() {
        allowed.push(current_dir);
    }
    
    // Allow user's documents directory for project files
    if let Some(home_dir) = dirs::home_dir() {
        allowed.push(home_dir.join("Documents"));
        allowed.push(home_dir.join("Desktop"));
    }
    
    // Allow system temp directory for temporary exports
    allowed.push(env::temp_dir());
    
    allowed
}

pub fn validate_component_id(id: &str) -> bool {
    if id.is_empty() || id.len() > 100 {
        return false;
    }
    
    // Must be alphanumeric with hyphens or underscores
    static COMPONENT_ID_RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"^[a-zA-Z0-9_-]+$").expect("Invalid regex for component ID validation")
    });
    COMPONENT_ID_RE.is_match(id)
}

pub fn validate_coordinate(value: f64) -> bool {
    value.is_finite() && 
    (-50000.0..=50000.0).contains(&value) && 
    !value.is_nan() && !value.is_infinite()
}

pub fn validate_coordinate_strict(value: f64) -> Result<(), crate::utils::error::AppError> {
    if !validate_coordinate(value) {
        return Err(crate::utils::error::AppError::InvalidInput("Invalid coordinate value".to_string()));
    }
    Ok(())
}

pub fn validate_component_count(count: usize) -> bool {
    count <= 10000
}

pub fn validate_wire_count(count: usize) -> bool {
    count <= 50000
}

pub fn validate_string_length(s: &str, max_len: usize) -> bool {
    !s.is_empty() && s.len() <= max_len && !s.contains('\0')
}

pub fn validate_string_length_range(s: &str, min_len: usize, max_len: usize) -> Result<(), crate::utils::error::AppError> {
    if s.is_empty() || s.len() < min_len || s.len() > max_len || s.contains('\0') {
        return Err(crate::utils::error::AppError::InvalidInput(format!("String length must be between {} and {} characters", min_len, max_len)));
    }
    Ok(())
}

pub fn validate_json_size(json: &str) -> bool {
    json.len() <= 10 * 1024 * 1024 // 10MB limit
}

pub fn validate_version(version: &str) -> bool {
    // Semantic versioning pattern
    static VERSION_RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"^\d+\.\d+\.\d+(-[\w.]+)?(\+[\w.]+)?$").expect("Invalid regex for version validation")
    });
    VERSION_RE.is_match(version)
}

pub fn sanitize_filename(filename: &str) -> String {
    // Remove invalid characters for filenames
    static FILENAME_RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r#"[<>:"/\\|?*\x00-\x1f]"#).expect("Invalid regex for filename sanitization")
    });
    FILENAME_RE.replace_all(filename, "_").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_project_name() {
        assert!(validate_project_name("My Project"));
        assert!(validate_project_name("project-123"));
        assert!(validate_project_name("test_project"));
        assert!(!validate_project_name(""));
        assert!(!validate_project_name("project/name"));
    }

    #[test]
    fn test_validate_file_path() {
        assert!(validate_file_path("./project.eda"));
        assert!(!validate_file_path("../../../etc/passwd"));
        assert!(!validate_file_path("path/with/null\0byte"));
        assert!(!validate_file_path(""));
        assert!(!validate_file_path(&"x".repeat(5000)));
    }
    
    #[test]
    fn test_validate_coordinate() {
        assert!(validate_coordinate(100.0));
        assert!(validate_coordinate(-100.0));
        assert!(!validate_coordinate(f64::NAN));
        assert!(!validate_coordinate(f64::INFINITY));
        assert!(!validate_coordinate(60000.0));
    }
    
    #[test]
    fn test_validate_counts() {
        assert!(validate_component_count(100));
        assert!(!validate_component_count(20000));
        assert!(validate_wire_count(1000));
        assert!(!validate_wire_count(100000));
    }
    
    #[test]
    fn test_validate_string_length() {
        assert!(validate_string_length("hello", 10));
        assert!(!validate_string_length("", 10)); // Empty string
        assert!(!validate_string_length("x".repeat(20).as_str(), 10)); // Too long
        assert!(!validate_string_length("null\0byte", 20)); // Null byte
    }
    
    #[test]
    fn test_validate_json_size() {
        assert!(validate_json_size("{\"test\": \"value\"}"));
        assert!(!validate_json_size(&"x".repeat(11 * 1024 * 1024))); // > 10MB
    }
    
    #[test]
    fn test_security_edge_cases() {
        // Path traversal attempts
        assert!(!validate_file_path("../"));
        assert!(!validate_file_path("..\\"));
        assert!(!validate_file_path("path/../other"));
        assert!(!validate_file_path("path\\..\\other"));
        
        // Null bytes and control characters
        assert!(!validate_file_path("file\0name"));
        assert!(!validate_file_path("file\rname"));
        assert!(!validate_file_path("file\nname"));
        
        // Coordinate overflow attempts
        assert!(!validate_coordinate(f64::INFINITY));
        assert!(!validate_coordinate(f64::NEG_INFINITY));
        assert!(!validate_coordinate(f64::NAN));
        assert!(!validate_coordinate(100000.0));
        assert!(!validate_coordinate(-100000.0));
        
        // Component ID injection attempts
        assert!(!validate_component_id("comp<script>"));
        assert!(!validate_component_id("comp&lt;script&gt;"));
        assert!(!validate_component_id("comp\0"));
    }

    #[test]
    fn test_validate_version() {
        assert!(validate_version("1.0.0"));
        assert!(validate_version("2.1.3-alpha"));
        assert!(validate_version("1.0.0+build123"));
        assert!(!validate_version("1.0"));
        assert!(!validate_version("v1.0.0"));
    }

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("file<name>.txt"), "file_name_.txt");
        assert_eq!(sanitize_filename("file/name"), "file_name");
    }
}