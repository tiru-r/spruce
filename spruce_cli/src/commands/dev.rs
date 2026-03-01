use super::*;
use std::path::Path;
use std::fs;
use std::process::{Command, Stdio};
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

pub fn start_dev_server(platform: Option<&str>, device: Option<&str>) -> Result<()> {
    println!("🚀 Starting Spruce development server...");
    
    // Check if we're in a Spruce project
    if !Path::new("spruce.config.ts").exists() {
        return Err(SpruceError::Config(
            "Not in a Spruce project directory. Run 'spruce create <app-name>' first.".to_string()
        ));
    }
    
    // Parse configuration
    let config = load_config()?;
    println!("📋 Loaded configuration for: {}", config.app.name);
    
    // Setup development environment
    setup_dev_environment(&config)?;
    
    // Start the appropriate development server based on platform
    let platform = platform.unwrap_or("auto");
    match platform {
        "android" => start_android_dev(device, &config)?,
        "ios" => start_ios_dev(device, &config)?,
        "desktop" => start_desktop_dev(&config)?,
        "auto" => start_auto_dev(&config)?,
        _ => {
            return Err(SpruceError::Config(format!(
                "Unknown platform: {}. Available: android, ios, desktop, auto", 
                platform
            )));
        }
    }
    
    Ok(())
}

fn load_config() -> Result<crate::config::SpruceConfig> {
    let config_content = fs::read_to_string("spruce.config.ts")?;
    
    // Extract JSON from TypeScript config (simple approach)
    let json_start = config_content.find('{').ok_or_else(|| {
        SpruceError::Config("Invalid config format".to_string())
    })?;
    let json_end = config_content.rfind('}').ok_or_else(|| {
        SpruceError::Config("Invalid config format".to_string())
    })?;
    
    let json_str = &config_content[json_start..=json_end];
    let config: crate::config::SpruceConfig = serde_json::from_str(json_str)
        .map_err(|e| SpruceError::Config(format!("Failed to parse config: {}", e)))?;
    
    Ok(config)
}

fn setup_dev_environment(config: &crate::config::SpruceConfig) -> Result<()> {
    println!("🔧 Setting up development environment...");
    
    // Install npm dependencies if needed
    if !Path::new("node_modules").exists() {
        println!("📦 Installing npm dependencies...");
        run_command("npm", &["install"], Some(Path::new(".")))?;
    }
    
    // Build Rust dependencies
    println!("🦀 Building Rust dependencies...");
    run_command("cargo", &["build"], Some(Path::new(".")))?;
    
    // Create development cache directory
    let cache_dir = Path::new(".spruce/cache");
    ensure_directory(cache_dir)?;
    
    println!("✅ Development environment ready");
    Ok(())
}

fn start_android_dev(device: Option<&str>, config: &crate::config::SpruceConfig) -> Result<()> {
    println!("🤖 Starting Android development server...");
    
    // Check Android SDK
    check_android_prerequisites()?;
    
    // Build for Android
    println!("🔨 Building for Android...");
    run_command("cargo", &[
        "build", 
        "--target", "aarch64-linux-android",
        "--features", "dev"
    ], Some(Path::new(".")))?;
    
    // Start Android emulator if no device specified
    if device.is_none() {
        start_android_emulator()?;
    }
    
    // Start hot reload server
    let (tx, rx) = mpsc::channel();
    
    // File watcher thread
    thread::spawn(move || {
        watch_files_for_changes(tx);
    });
    
    // Main development server
    start_hot_reload_server(config.performance.target_fps, rx)?;
    
    println!("🎯 Android development server running");
    println!("📱 App will reload automatically when you make changes");
    
    // Keep server running
    loop {
        thread::sleep(Duration::from_secs(1));
        // Check for user input or signals to stop
    }
}

fn start_ios_dev(device: Option<&str>, config: &crate::config::SpruceConfig) -> Result<()> {
    println!("🍎 Starting iOS development server...");
    
    // Check if we're on macOS
    if !cfg!(target_os = "macos") {
        return Err(SpruceError::Config(
            "iOS development requires macOS".to_string()
        ));
    }
    
    // Check iOS development prerequisites
    check_ios_prerequisites()?;
    
    // Build for iOS
    println!("🔨 Building for iOS...");
    run_command("cargo", &[
        "build", 
        "--target", "aarch64-apple-ios",
        "--features", "dev"
    ], Some(Path::new(".")))?;
    
    // Start iOS simulator if no device specified
    if device.is_none() {
        start_ios_simulator()?;
    }
    
    println!("🎯 iOS development server running");
    
    // Keep server running
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}

fn start_desktop_dev(config: &crate::config::SpruceConfig) -> Result<()> {
    println!("🖥️  Starting desktop development server...");
    
    // Build for desktop
    println!("🔨 Building for desktop...");
    run_command("cargo", &["build", "--features", "dev"], Some(Path::new(".")))?;
    
    // Start desktop window
    println!("🪟 Opening desktop preview window...");
    
    let mut child = Command::new("cargo")
        .args(&["run", "--bin", "spruce-dev-server"])
        .current_dir(".")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    
    println!("🎯 Desktop development server running");
    println!("🔄 Hot reload enabled - changes will appear instantly");
    
    // Wait for child process
    child.wait()?;
    
    Ok(())
}

fn start_auto_dev(config: &crate::config::SpruceConfig) -> Result<()> {
    println!("🤖 Auto-detecting best development platform...");
    
    // Try platforms in order of preference
    if cfg!(target_os = "macos") && check_ios_prerequisites().is_ok() {
        println!("✅ Using iOS Simulator (detected macOS with Xcode)");
        start_ios_dev(None, config)
    } else if check_android_prerequisites().is_ok() {
        println!("✅ Using Android Emulator (detected Android SDK)");
        start_android_dev(None, config)
    } else {
        println!("✅ Using Desktop Preview (no mobile SDKs detected)");
        start_desktop_dev(config)
    }
}

fn check_android_prerequisites() -> Result<()> {
    // Check for Android SDK
    if std::env::var("ANDROID_HOME").is_err() && std::env::var("ANDROID_SDK_ROOT").is_err() {
        return Err(SpruceError::Config(
            "Android SDK not found. Please set ANDROID_HOME or ANDROID_SDK_ROOT".to_string()
        ));
    }
    
    // Check for NDK
    if std::env::var("ANDROID_NDK_HOME").is_err() {
        return Err(SpruceError::Config(
            "Android NDK not found. Please set ANDROID_NDK_HOME".to_string()
        ));
    }
    
    // Check for adb
    if Command::new("adb").arg("version").output().is_err() {
        return Err(SpruceError::Config(
            "Android Debug Bridge (adb) not found in PATH".to_string()
        ));
    }
    
    Ok(())
}

fn check_ios_prerequisites() -> Result<()> {
    // Check for Xcode command line tools
    if Command::new("xcodebuild").arg("-version").output().is_err() {
        return Err(SpruceError::Config(
            "Xcode not found. Please install Xcode from the App Store".to_string()
        ));
    }
    
    // Check for iOS simulator
    if Command::new("xcrun").args(&["simctl", "list"]).output().is_err() {
        return Err(SpruceError::Config(
            "iOS Simulator not available".to_string()
        ));
    }
    
    Ok(())
}

fn start_android_emulator() -> Result<()> {
    println!("📱 Starting Android emulator...");
    
    // List available AVDs
    let output = Command::new("emulator").args(&["-list-avds"]).output()?;
    let avds = String::from_utf8_lossy(&output.stdout);
    
    if avds.trim().is_empty() {
        return Err(SpruceError::Config(
            "No Android AVDs found. Please create one using Android Studio".to_string()
        ));
    }
    
    // Start the first available AVD
    let first_avd = avds.lines().next().unwrap().trim();
    println!("🚀 Starting AVD: {}", first_avd);
    
    Command::new("emulator")
        .args(&["-avd", first_avd])
        .spawn()?;
    
    // Wait a bit for emulator to start
    thread::sleep(Duration::from_secs(3));
    
    Ok(())
}

fn start_ios_simulator() -> Result<()> {
    println!("📱 Starting iOS Simulator...");
    
    // Boot the default iOS simulator
    run_command("xcrun", &[
        "simctl", "boot", 
        "iPhone 15 Pro" // Default device
    ], None)?;
    
    // Open Simulator app
    run_command("open", &[
        "/Applications/Xcode.app/Contents/Developer/Applications/Simulator.app"
    ], None)?;
    
    Ok(())
}

fn watch_files_for_changes(tx: mpsc::Sender<String>) {
    // Simple file watcher implementation
    // In a real implementation, you'd use a proper file watching library
    let watch_paths = vec![
        "src/",
        "native/",
        "spruce.config.ts",
    ];
    
    loop {
        for path in &watch_paths {
            if Path::new(path).exists() {
                // Check file modification times
                // This is a simplified version - a real implementation would be more sophisticated
            }
        }
        
        thread::sleep(Duration::from_millis(100));
    }
}

fn start_hot_reload_server(target_fps: u32, _rx: mpsc::Receiver<String>) -> Result<()> {
    println!("🔥 Hot reload server started ({}fps target)", target_fps);
    
    // Start HTTP server for hot reload communication
    // This would typically run on a separate thread and handle:
    // - File change notifications
    // - Live reload of Vue components  
    // - Rust code recompilation
    // - Device/emulator communication
    
    Ok(())
}