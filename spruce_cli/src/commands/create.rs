use super::*;
use crate::templates::*;
use crate::config::SpruceConfig;
use std::fs;
use std::path::Path;

pub fn create_app(app_name: &str, template: Option<&str>) -> Result<()> {
    println!("🌲 Creating Spruce app: {}", app_name);
    
    // Check prerequisites
    check_prerequisites()?;
    
    // Validate app name
    validate_app_name(app_name)?;
    
    // Create project directory
    let project_path = Path::new(app_name);
    if project_path.exists() {
        return Err(SpruceError::Config(format!(
            "Directory '{}' already exists", app_name
        )));
    }
    
    ensure_directory(project_path)?;
    
    // Create project structure
    create_project_structure(project_path)?;
    
    // Apply template
    let template_name = template.unwrap_or("vue-mobile");
    apply_template(project_path, app_name, template_name)?;
    
    // Initialize configuration
    create_spruce_config(project_path, app_name)?;
    
    // Initialize git repository
    initialize_git(project_path)?;
    
    // Install dependencies
    install_dependencies(project_path)?;
    
    println!("✅ Successfully created Spruce app: {}", app_name);
    println!();
    println!("📁 Project structure:");
    print_project_tree(project_path);
    println!();
    println!("🚀 Next steps:");
    println!("    cd {}", app_name);
    println!("    spruce dev");
    
    Ok(())
}

fn validate_app_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(SpruceError::Config("App name cannot be empty".to_string()));
    }
    
    if !name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
        return Err(SpruceError::Config(
            "App name can only contain alphanumeric characters, hyphens, and underscores".to_string()
        ));
    }
    
    if name.starts_with('-') || name.starts_with('_') {
        return Err(SpruceError::Config(
            "App name cannot start with a hyphen or underscore".to_string()
        ));
    }
    
    Ok(())
}

fn create_project_structure(project_path: &Path) -> Result<()> {
    let directories = vec![
        "src",
        "src/components",
        "src/pages", 
        "src/stores",
        "src/assets",
        "src/utils",
        "native",
        "native/android",
        "native/ios",
        "native/shared",
        "tests",
        ".spruce",
    ];
    
    for dir in directories {
        let dir_path = project_path.join(dir);
        ensure_directory(&dir_path)?;
    }
    
    Ok(())
}

fn apply_template(project_path: &Path, app_name: &str, template_name: &str) -> Result<()> {
    match template_name {
        "vue-mobile" => create_vue_mobile_template(project_path, app_name)?,
        "shopping-app" => create_shopping_app_template(project_path, app_name)?,
        "blank" => create_blank_template(project_path, app_name)?,
        _ => {
            return Err(SpruceError::Config(format!(
                "Unknown template: {}. Available templates: vue-mobile, shopping-app, blank", 
                template_name
            )));
        }
    }
    
    Ok(())
}

fn create_spruce_config(project_path: &Path, app_name: &str) -> Result<()> {
    let config = SpruceConfig::new(app_name);
    let config_content = config.to_typescript()?;
    
    let config_path = project_path.join("spruce.config.ts");
    fs::write(config_path, config_content)?;
    
    Ok(())
}

fn initialize_git(project_path: &Path) -> Result<()> {
    run_command("git", &["init"], Some(project_path))?;
    
    // Create .gitignore
    let gitignore_content = r#"# Build outputs
target/
dist/
build/
*.apk
*.ipa

# Dependencies
node_modules/
.pnp
.pnp.js

# Development
.env.local
.env.development.local
.env.test.local
.env.production.local

# Logs
npm-debug.log*
yarn-debug.log*
yarn-error.log*
*.log

# Runtime data
pids
*.pid
*.seed
*.pid.lock

# Coverage directory used by tools like istanbul
coverage/
*.lcov

# IDE
.vscode/
.idea/
*.swp
*.swo

# OS
.DS_Store
.DS_Store?
._*
.Spotlight-V100
.Trashes
ehthumbs.db
Thumbs.db

# Spruce
.spruce/cache/
.spruce/temp/
"#;
    
    let gitignore_path = project_path.join(".gitignore");
    fs::write(gitignore_path, gitignore_content)?;
    
    Ok(())
}

fn install_dependencies(project_path: &Path) -> Result<()> {
    println!("📦 Installing dependencies...");
    
    // Install npm dependencies
    run_command("npm", &["install"], Some(project_path))?;
    
    // Add Rust targets for mobile development
    println!("🦀 Adding Rust targets...");
    run_command("rustup", &["target", "add", "aarch64-linux-android"], None)?;
    run_command("rustup", &["target", "add", "armv7-linux-androideabi"], None)?;
    run_command("rustup", &["target", "add", "aarch64-apple-ios"], None)?;
    run_command("rustup", &["target", "add", "x86_64-apple-ios"], None)?;
    
    Ok(())
}

fn print_project_tree(project_path: &Path) {
    println!("{}/ ", project_path.file_name().unwrap().to_str().unwrap());
    println!("├── src/");
    println!("│   ├── components/");
    println!("│   ├── pages/");
    println!("│   ├── stores/");
    println!("│   └── assets/");
    println!("├── native/");
    println!("│   ├── android/");
    println!("│   ├── ios/");
    println!("│   └── shared/");
    println!("├── spruce.config.ts");
    println!("├── package.json");
    println!("├── Cargo.toml");
    println!("└── README.md");
}