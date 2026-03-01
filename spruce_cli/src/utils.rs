use std::path::Path;
use std::fs;
use std::io::{self, Write};

/// Utility functions for Spruce CLI

pub fn print_success(message: &str) {
    println!("✅ {}", message);
}

pub fn print_error(message: &str) {
    eprintln!("❌ {}", message);
}

pub fn print_warning(message: &str) {
    println!("⚠️  {}", message);
}

pub fn print_info(message: &str) {
    println!("ℹ️  {}", message);
}

pub fn print_progress(message: &str) {
    print!("🔄 {}...", message);
    io::stdout().flush().unwrap();
}

pub fn print_progress_done() {
    println!(" ✅");
}

pub fn validate_project_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("Project name cannot be empty".to_string());
    }
    
    if name.len() > 50 {
        return Err("Project name is too long (max 50 characters)".to_string());
    }
    
    if !name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
        return Err("Project name can only contain letters, numbers, hyphens, and underscores".to_string());
    }
    
    if name.starts_with('-') || name.starts_with('_') || name.starts_with(char::is_numeric) {
        return Err("Project name cannot start with a hyphen, underscore, or number".to_string());
    }
    
    // Check for reserved names
    let reserved_names = vec![
        "spruce", "node_modules", "target", "dist", "build", "src", "lib", "bin",
        "con", "prn", "aux", "nul", // Windows reserved names
        "com1", "com2", "com3", "com4", "com5", "com6", "com7", "com8", "com9",
        "lpt1", "lpt2", "lpt3", "lpt4", "lpt5", "lpt6", "lpt7", "lpt8", "lpt9",
    ];
    
    if reserved_names.contains(&name.to_lowercase().as_str()) {
        return Err(format!("'{}' is a reserved name", name));
    }
    
    Ok(())
}

pub fn check_system_requirements() -> Result<(), String> {
    let mut missing_tools = Vec::new();
    
    // Check for required tools
    let required_tools = vec![
        ("node", "Node.js"),
        ("npm", "npm"),
        ("cargo", "Rust"),
        ("git", "Git"),
    ];
    
    for (command, name) in required_tools {
        if !command_exists(command) {
            missing_tools.push(name);
        }
    }
    
    if !missing_tools.is_empty() {
        return Err(format!(
            "Missing required tools: {}. Please install them first.",
            missing_tools.join(", ")
        ));
    }
    
    // Check versions
    check_node_version()?;
    check_rust_version()?;
    
    Ok(())
}

fn command_exists(command: &str) -> bool {
    std::process::Command::new(command)
        .arg("--version")
        .output()
        .is_ok()
}

fn check_node_version() -> Result<(), String> {
    let output = std::process::Command::new("node")
        .arg("--version")
        .output()
        .map_err(|_| "Failed to check Node.js version".to_string())?;
    
    let version_str = String::from_utf8_lossy(&output.stdout);
    let version = version_str.trim().trim_start_matches('v');
    
    if !version_str.starts_with('v') {
        return Err("Invalid Node.js version format".to_string());
    }
    
    let major_version: u32 = version
        .split('.')
        .next()
        .and_then(|v| v.parse().ok())
        .ok_or("Failed to parse Node.js version")?;
    
    if major_version < 18 {
        return Err(format!(
            "Node.js version {} is too old. Spruce requires Node.js 18 or higher.",
            version
        ));
    }
    
    Ok(())
}

fn check_rust_version() -> Result<(), String> {
    let output = std::process::Command::new("rustc")
        .arg("--version")
        .output()
        .map_err(|_| "Failed to check Rust version".to_string())?;
    
    let version_str = String::from_utf8_lossy(&output.stdout);
    
    // Extract version from "rustc 1.70.0 (90c541806 2023-05-31)"
    let version = version_str
        .split_whitespace()
        .nth(1)
        .ok_or("Failed to parse Rust version")?;
    
    let version_parts: Vec<u32> = version
        .split('.')
        .take(2)
        .map(|v| v.parse().unwrap_or(0))
        .collect();
    
    if version_parts.len() < 2 {
        return Err("Invalid Rust version format".to_string());
    }
    
    let (major, minor) = (version_parts[0], version_parts[1]);
    
    // Require Rust 1.70+
    if major < 1 || (major == 1 && minor < 70) {
        return Err(format!(
            "Rust version {} is too old. Spruce requires Rust 1.70 or higher.",
            version
        ));
    }
    
    Ok(())
}

pub fn get_user_input(prompt: &str) -> Result<String, io::Error> {
    print!("{}: ", prompt);
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    Ok(input.trim().to_string())
}

pub fn confirm_action(message: &str) -> Result<bool, io::Error> {
    loop {
        print!("{} (y/N): ", message);
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return Ok(true),
            "n" | "no" | "" => return Ok(false),
            _ => println!("Please enter 'y' for yes or 'n' for no."),
        }
    }
}

pub fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if size >= 100.0 {
        format!("{:.0} {}", size, UNITS[unit_index])
    } else if size >= 10.0 {
        format!("{:.1} {}", size, UNITS[unit_index])
    } else {
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}

pub fn calculate_directory_size(path: &Path) -> Result<u64, io::Error> {
    let mut total_size = 0;
    
    if path.is_file() {
        return Ok(fs::metadata(path)?.len());
    }
    
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            total_size += calculate_directory_size(&entry_path)?;
        }
    }
    
    Ok(total_size)
}

pub fn clean_path(path: &str) -> String {
    path.replace('\\', "/")
        .trim_start_matches('/')
        .trim_end_matches('/')
        .to_string()
}

pub fn get_relative_path(from: &Path, to: &Path) -> Result<String, String> {
    let from = fs::canonicalize(from)
        .map_err(|e| format!("Failed to canonicalize 'from' path: {}", e))?;
    let to = fs::canonicalize(to)
        .map_err(|e| format!("Failed to canonicalize 'to' path: {}", e))?;
    
    match to.strip_prefix(&from) {
        Ok(relative) => Ok(relative.to_string_lossy().to_string()),
        Err(_) => {
            // If paths don't share a common prefix, return absolute path
            Ok(to.to_string_lossy().to_string())
        }
    }
}

pub fn ensure_file_parent_dir(file_path: &Path) -> Result<(), io::Error> {
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

pub fn copy_file_with_progress(src: &Path, dst: &Path) -> Result<u64, io::Error> {
    ensure_file_parent_dir(dst)?;
    
    let metadata = fs::metadata(src)?;
    let file_size = metadata.len();
    
    if file_size > 10 * 1024 * 1024 { // 10MB
        print!("📁 Copying {} ({})", 
               src.file_name().unwrap().to_string_lossy(),
               format_file_size(file_size));
        io::stdout().flush()?;
    }
    
    let bytes_copied = fs::copy(src, dst)?;
    
    if file_size > 10 * 1024 * 1024 {
        println!(" ✅");
    }
    
    Ok(bytes_copied)
}

pub fn is_spruce_project() -> bool {
    Path::new("spruce.config.ts").exists() || 
    Path::new("spruce.config.js").exists()
}

pub fn find_spruce_root() -> Option<std::path::PathBuf> {
    let mut current = std::env::current_dir().ok()?;
    
    loop {
        if current.join("spruce.config.ts").exists() || 
           current.join("spruce.config.js").exists() {
            return Some(current);
        }
        
        current = current.parent()?.to_path_buf();
        
        // Stop at filesystem root
        if current.parent().is_none() {
            break;
        }
    }
    
    None
}

pub fn get_spruce_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn print_banner() {
    println!(r#"
    🌲 Spruce Platform v{}
    ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    Next-generation mobile development
    Pure Rust UI • Vue 3.6 • 60+ FPS
    "#, get_spruce_version());
}

pub fn print_completion_message(app_name: &str) {
    println!();
    println!("🎉 Welcome to the future of mobile development!");
    println!();
    println!("📱 Your {} app is ready with:", app_name);
    println!("   ✅ Pure Rust UI rendering (60+ FPS)");
    println!("   ✅ Vue 3.6 Vapor mode (reactive signals)");
    println!("   ✅ TypeScript support");
    println!("   ✅ Hot reload development");
    println!("   ✅ Mobile optimizations");
    println!();
    println!("🚀 Get started:");
    println!("   cd {}", app_name);
    println!("   spruce dev");
    println!();
    println!("📚 Learn more: https://spruce.dev/docs");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_project_name() {
        assert!(validate_project_name("my-app").is_ok());
        assert!(validate_project_name("my_app").is_ok());
        assert!(validate_project_name("myapp123").is_ok());
        
        assert!(validate_project_name("").is_err());
        assert!(validate_project_name("my app").is_err()); // spaces
        assert!(validate_project_name("-myapp").is_err()); // starts with hyphen
        assert!(validate_project_name("123app").is_err()); // starts with number
        assert!(validate_project_name("spruce").is_err()); // reserved name
    }
    
    #[test]
    fn test_format_file_size() {
        assert_eq!(format_file_size(0), "0.00 B");
        assert_eq!(format_file_size(512), "512 B");
        assert_eq!(format_file_size(1024), "1.00 KB");
        assert_eq!(format_file_size(1536), "1.50 KB");
        assert_eq!(format_file_size(1024 * 1024), "1.00 MB");
        assert_eq!(format_file_size(1024 * 1024 * 1024), "1.00 GB");
    }
    
    #[test]
    fn test_clean_path() {
        assert_eq!(clean_path("/path/to/file"), "path/to/file");
        assert_eq!(clean_path("path/to/file/"), "path/to/file");
        assert_eq!(clean_path("\\path\\to\\file"), "path/to/file");
        assert_eq!(clean_path("/path\\to/file\\"), "path/to/file");
    }
}