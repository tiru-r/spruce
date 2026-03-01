use anyhow::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;
use std::fs;
use crate::templates;

pub async fn create_project(name: String, template: String, output: Option<PathBuf>) -> Result<()> {
    println!("{} Creating Spruce app: {}", "🚀".bright_green(), name.bright_cyan());
    
    let project_dir = output.unwrap_or_else(|| PathBuf::from(&name));
    
    if project_dir.exists() {
        return Err(anyhow::anyhow!("Directory {} already exists", project_dir.display()));
    }
    
    let pb = ProgressBar::new(6);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {msg}")?
            .progress_chars("#>-"),
    );
    
    pb.set_message("Creating project structure...");
    pb.inc(1);
    
    // Create project directory
    fs::create_dir_all(&project_dir)?;
    
    pb.set_message("Generating Vue app files...");
    pb.inc(1);
    
    // Generate template files based on selected template
    match template.as_str() {
        "basic" => templates::create_basic_template(&project_dir, &name)?,
        "navigation" => templates::create_navigation_template(&project_dir, &name)?,
        "tabs" => templates::create_tabs_template(&project_dir, &name)?,
        _ => return Err(anyhow::anyhow!("Unknown template: {}", template)),
    }
    
    pb.set_message("Setting up package.json...");
    pb.inc(1);
    
    create_package_json(&project_dir, &name)?;
    
    pb.set_message("Creating platform config...");
    pb.inc(1);
    
    create_platform_config(&project_dir)?;
    
    pb.set_message("Setting up development tools...");
    pb.inc(1);
    
    create_dev_config(&project_dir)?;
    
    pb.set_message("Installing dependencies...");
    pb.inc(1);
    
    // Dependencies managed by SpruceVM runtime
    
    pb.finish_with_message("Project created successfully! 🎉");
    
    println!("\n{} Next steps:", "📋".bright_blue());
    println!("  cd {}", project_dir.display());
    println!("  spruce dev");
    println!("\n{} Happy coding! 🚀", "🎉".bright_green());
    
    Ok(())
}

fn create_package_json(project_dir: &PathBuf, name: &str) -> Result<()> {
    let package_json = format!(r#"{{
  "name": "{}",
  "version": "1.0.0",
  "type": "module",
  "scripts": {{
    "dev": "spruce dev",
    "build": "spruce build",
    "run:ios": "spruce run --platform ios",
    "run:android": "spruce run --platform android",
    "doctor": "spruce doctor"
  }},
  "dependencies": {{
    "vue": "3.6.0-beta.7",
    "@vue/reactivity": "3.6.0-beta.7",
    "@vue/runtime-core": "3.6.0-beta.7",
    "@vue/runtime-dom": "3.6.0-beta.7"
  }},
  "devDependencies": {{
    "typescript": "^5.3.0"
  }}
}}"#, name);

    fs::write(project_dir.join("package.json"), package_json)?;
    Ok(())
}

fn create_platform_config(project_dir: &PathBuf) -> Result<()> {
    let config = r#"{
  "name": "Spruce App",
  "displayName": "Spruce App",
  "version": "1.0.0",
  "platforms": {
    "ios": {
      "bundleIdentifier": "com.spruce.app",
      "deploymentTarget": "13.0"
    },
    "android": {
      "packageName": "com.spruce.app",
      "minSdkVersion": 21,
      "compileSdkVersion": 34
    }
  },
  "build": {
    "rust": {
      "target": "mobile",
      "ui_renderer": "rust"
    },
    "js": {
      "runtime": "sprucevm",
      "mode": "vapor"
    }
  },
  "spruce": {
    "ui_system": "pure_rust",
    "performance": "3x_faster",
    "vue_version": "3.6_vapor"
  }
}"#;

    fs::write(project_dir.join("spruce.config.json"), config)?;
    Ok(())
}

fn create_dev_config(project_dir: &PathBuf) -> Result<()> {
    let typescript_config = r#"{
  "compilerOptions": {
    "target": "ES2022",
    "lib": ["ES2022"],
    "module": "ESNext",
    "moduleResolution": "node",
    "allowImportingTsExtensions": true,
    "noEmit": true,
    "strict": true,
    "skipLibCheck": true,
    "isolatedModules": true,
    "types": ["vue"]
  },
  "include": ["src/**/*"],
  "exclude": ["node_modules"]
}"#;

    fs::write(project_dir.join("tsconfig.json"), typescript_config)?;

    let gitignore = r#"# Dependencies
node_modules/
*.lockb

# Build outputs
dist/
build/
target/

# Native builds
ios/build/
android/build/
android/.gradle/

# Development
.env
.env.local

# IDE
.vscode/
.idea/

# OS
.DS_Store
Thumbs.db
"#;
    fs::write(project_dir.join(".gitignore"), gitignore)?;
    Ok(())
}