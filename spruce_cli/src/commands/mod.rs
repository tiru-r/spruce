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
        return Err(SpruceError::Build(format!("Command '{}' failed: {}", command, stderr)));
    }
    
    Ok(())
}

pub fn prompt_user(message: &str) -> Result<String> {
    print!("{}", message);
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    Ok(input.trim().to_string())
}

pub fn check_prerequisites() -> Result<()> {
    let required_tools = vec![
        ("cargo", "Rust toolchain"),
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
    
    Ok(())
}