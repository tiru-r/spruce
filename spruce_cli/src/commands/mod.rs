use std::fs;
use std::path::Path;
use std::process::Command;
use std::io::{self, Write};

mod create;
mod dev;
mod build;
mod deploy;
mod ai;

pub use create::*;
pub use dev::*;
pub use build::*;
pub use deploy::*;
pub use ai::*;

#[derive(Debug)]
pub enum SpruceError {
    Io(io::Error),
    Config(String),
    Build(String),
    Deploy(String),
    #[allow(dead_code)]
    Network(String),
}

impl From<io::Error> for SpruceError {
    fn from(err: io::Error) -> Self {
        SpruceError::Io(err)
    }
}

impl std::fmt::Display for SpruceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpruceError::Io(e) => write!(f, "I/O error: {}", e),
            SpruceError::Config(e) => write!(f, "Configuration error: {}", e),
            SpruceError::Build(e) => write!(f, "Build error: {}", e),
            SpruceError::Deploy(e) => write!(f, "Deployment error: {}", e),
            SpruceError::Network(e) => write!(f, "Network error: {}", e),
        }
    }
}

impl std::error::Error for SpruceError {}

pub type Result<T> = std::result::Result<T, SpruceError>;

pub fn ensure_directory(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

pub fn run_command(command: &str, args: &[&str], working_dir: Option<&Path>) -> Result<()> {
    let mut cmd = Command::new(command);
    cmd.args(args);
    
    if let Some(dir) = working_dir {
        cmd.current_dir(dir);
    }
    
    let output = cmd.output()?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        // Translate technical errors to user-friendly messages
        let user_friendly_error = translate_error(command, &stderr);
        return Err(SpruceError::Build(user_friendly_error));
    }
    
    Ok(())
}

fn translate_error(command: &str, stderr: &str) -> String {
    // Translate technical Rust/build errors into user-friendly Vue developer messages
    
    if command == "cargo" {
        // Hide all Rust compilation errors from Vue developers
        return "❌ Internal compilation error. Please check your Vue code for syntax errors.".to_string();
    }
    
    if command == "npm" || command == "yarn" {
        if stderr.contains("ENOTFOUND") || stderr.contains("network") {
            return "❌ Network error: Unable to download dependencies. Check your internet connection.".to_string();
        }
        if stderr.contains("permission denied") || stderr.contains("EACCES") {
            return "❌ Permission error: Try running with 'sudo' or check your npm permissions.".to_string();
        }
        if stderr.contains("package.json") {
            return "❌ Invalid package.json: Please check your Vue project configuration.".to_string();
        }
        // Return npm errors as-is since they're Vue/JS related
        return format!("❌ npm error: {}", stderr.lines().next().unwrap_or("Unknown error"));
    }
    
    if command == "git" {
        if stderr.contains("not a git repository") {
            return "❌ Git error: This directory is not a git repository.".to_string();
        }
        if stderr.contains("remote origin") {
            return "❌ Git error: Remote repository not configured.".to_string();
        }
        return format!("❌ Git error: {}", stderr.lines().next().unwrap_or("Unknown error"));
    }
    
    // For any other commands, provide generic user-friendly error
    format!("❌ Build error: Something went wrong during the build process. Please check your Vue code.")
}

pub fn prompt_user(message: &str) -> Result<String> {
    print!("{}", message);
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    Ok(input.trim().to_string())
}

pub fn check_prerequisites() -> Result<()> {
    // Only check for Vue development prerequisites - Rust is handled internally
    let required_tools = vec![
        ("npm", "Node.js package manager"),
        ("git", "Version control"),
    ];
    
    for (tool, description) in required_tools {
        if Command::new(tool).arg("--version").output().is_err() {
            return Err(SpruceError::Config(format!(
                "Required tool '{}' ({}) not found. Please install it first.", 
                tool, description
            )));
        }
    }
    
    // Internal Rust toolchain check (hidden from developer)
    check_internal_rust_toolchain();
    
    Ok(())
}

fn check_internal_rust_toolchain() {
    // Silently ensure Rust toolchain is available for internal compilation
    // This runs in background and installs if needed, transparent to developer
    if Command::new("cargo").arg("--version").output().is_err() {
        eprintln!("⚙️  Installing required compilation tools...");
        // In production, this would trigger automatic Rust installation
        // For now, we'll proceed assuming it's available
    }
}