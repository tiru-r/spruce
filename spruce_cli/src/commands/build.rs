use super::*;
use std::path::Path;
use std::fs;

pub fn build_project(release: bool, platform: Option<&str>) -> Result<()> {
    println!("🔨 Building Spruce project...");
    
    // Check if we're in a Spruce project
    if !Path::new("spruce.config.ts").exists() {
        return Err(SpruceError::Config(
            "Not in a Spruce project directory".to_string()
        ));
    }
    
    // Load configuration
    let config = load_config()?;
    println!("📋 Building: {} v{}", config.app.name, config.app.version);
    
    // Determine build targets
    let targets = match platform {
        Some("android") => vec!["android"],
        Some("ios") => vec!["ios"],
        Some("desktop") => vec!["desktop"],
        Some("web") => vec!["web"],
        None => vec!["android", "ios"], // Default to mobile platforms
        Some(p) => return Err(SpruceError::Config(format!("Unknown platform: {}", p))),
    };
    
    // Create build output directory
    let build_dir = Path::new("dist");
    ensure_directory(build_dir)?;
    
    // Build for each target
    for target in targets {
        build_for_target(target, release, &config)?;
    }
    
    println!("✅ Build completed successfully");
    print_build_summary(&config, release);
    
    Ok(())
}

fn load_config() -> Result<crate::config::SpruceConfig> {
    let config_content = fs::read_to_string("spruce.config.ts")?;
    
    // Extract JSON from TypeScript config
    let json_start = config_content.find('{').unwrap();
    let json_end = config_content.rfind('}').unwrap();
    let json_str = &config_content[json_start..=json_end];
    
    let config: crate::config::SpruceConfig = serde_json::from_str(json_str)
        .map_err(|e| SpruceError::Config(format!("Failed to parse config: {}", e)))?;
    
    Ok(config)
}

fn build_for_target(target: &str, release: bool, config: &crate::config::SpruceConfig) -> Result<()> {
    match target {
        "android" => build_android(release, config),
        "ios" => build_ios(release, config),
        "desktop" => build_desktop(release, config),
        "web" => build_web(release, config),
        _ => Err(SpruceError::Build(format!("Unknown target: {}", target))),
    }
}

fn build_android(release: bool, config: &crate::config::SpruceConfig) -> Result<()> {
    println!("🤖 Building for Android...");
    
    // Check Android prerequisites
    check_android_sdk()?;
    
    let android_config = config.platforms.android.as_ref()
        .ok_or_else(|| SpruceError::Config("No Android configuration found".to_string()))?;
    
    // Build Rust library for Android
    let rust_args = if release {
        vec!["build", "--release", "--target", "aarch64-linux-android"]
    } else {
        vec!["build", "--target", "aarch64-linux-android"]
    };
    
    println!("🦀 Building Rust library for Android...");
    run_command("cargo", &rust_args, Some(Path::new(".")))?;
    
    // Build ARM7 version as well
    let arm7_args = if release {
        vec!["build", "--release", "--target", "armv7-linux-androideabi"]
    } else {
        vec!["build", "--target", "armv7-linux-androideabi"]
    };
    
    run_command("cargo", &arm7_args, Some(Path::new(".")))?;
    
    // Compile Vue app with Vapor mode
    println!("⚡ Compiling Vue app with Vapor mode...");
    compile_vue_for_mobile(config)?;
    
    // Create Android project structure
    create_android_project(config, android_config)?;
    
    // Copy native libraries
    copy_android_libraries(release)?;
    
    // Build APK using Gradle
    let gradle_task = if release { "assembleRelease" } else { "assembleDebug" };
    println!("📦 Building APK with Gradle...");
    run_command("./gradlew", &[gradle_task], Some(Path::new("android")))?;
    
    // Copy APK to dist folder
    let apk_name = if release { "app-release.apk" } else { "app-debug.apk" };
    let src_apk = Path::new("android/app/build/outputs/apk").join(if release { "release" } else { "debug" }).join(apk_name);
    let dest_apk = Path::new("dist").join(format!("{}-android.apk", config.app.name));
    
    fs::copy(src_apk, dest_apk)?;
    
    println!("✅ Android build completed");
    Ok(())
}

fn build_ios(release: bool, config: &crate::config::SpruceConfig) -> Result<()> {
    println!("🍎 Building for iOS...");
    
    if !cfg!(target_os = "macos") {
        return Err(SpruceError::Build(
            "iOS builds require macOS".to_string()
        ));
    }
    
    // Check iOS prerequisites
    check_ios_tools()?;
    
    let ios_config = config.platforms.ios.as_ref()
        .ok_or_else(|| SpruceError::Config("No iOS configuration found".to_string()))?;
    
    // Build Rust library for iOS
    let rust_args = if release {
        vec!["build", "--release", "--target", "aarch64-apple-ios"]
    } else {
        vec!["build", "--target", "aarch64-apple-ios"]
    };
    
    println!("🦀 Building Rust library for iOS...");
    run_command("cargo", &rust_args, Some(Path::new(".")))?;
    
    // Also build for x86_64 for simulator
    let sim_args = if release {
        vec!["build", "--release", "--target", "x86_64-apple-ios"]
    } else {
        vec!["build", "--target", "x86_64-apple-ios"]
    };
    
    run_command("cargo", &sim_args, Some(Path::new(".")))?;
    
    // Compile Vue app
    println!("⚡ Compiling Vue app with Vapor mode...");
    compile_vue_for_mobile(config)?;
    
    // Create iOS project structure
    create_ios_project(config, ios_config)?;
    
    // Build with Xcode
    let scheme = if release { "Release" } else { "Debug" };
    println!("📱 Building with Xcode...");
    run_command("xcodebuild", &[
        "-project", "ios/SpruceApp.xcodeproj",
        "-scheme", "SpruceApp",
        "-configuration", scheme,
        "-destination", "generic/platform=iOS",
        "archive"
    ], Some(Path::new(".")))?;
    
    println!("✅ iOS build completed");
    Ok(())
}

fn build_desktop(release: bool, config: &crate::config::SpruceConfig) -> Result<()> {
    println!("🖥️  Building for desktop...");
    
    // Build Rust executable
    let rust_args = if release {
        vec!["build", "--release"]
    } else {
        vec!["build"]
    };
    
    println!("🦀 Building Rust executable...");
    run_command("cargo", &rust_args, Some(Path::new(".")))?;
    
    // Compile Vue app for desktop
    compile_vue_for_desktop(config)?;
    
    // Copy executable to dist
    let exe_name = if cfg!(windows) { 
        format!("{}.exe", config.app.name) 
    } else { 
        config.app.name.clone() 
    };
    
    let build_type = if release { "release" } else { "debug" };
    let src_exe = Path::new("target").join(build_type).join(&exe_name);
    let dest_exe = Path::new("dist").join(format!("{}-desktop{}", 
        config.app.name,
        if cfg!(windows) { ".exe" } else { "" }
    ));
    
    fs::copy(src_exe, dest_exe)?;
    
    println!("✅ Desktop build completed");
    Ok(())
}

fn build_web(release: bool, config: &crate::config::SpruceConfig) -> Result<()> {
    println!("🌐 Building for web...");
    
    // Use Vite to build web version
    let vite_command = if release { "build" } else { "build:dev" };
    
    println!("📦 Building with Vite...");
    run_command("npm", &["run", vite_command], Some(Path::new(".")))?;
    
    // Copy to dist folder
    let web_dist = Path::new("dist/web");
    ensure_directory(web_dist)?;
    
    // Copy built files from Vite output
    copy_directory(Path::new("dist"), web_dist)?;
    
    println!("✅ Web build completed");
    Ok(())
}

fn compile_vue_for_mobile(_config: &crate::config::SpruceConfig) -> Result<()> {
    // Compile Vue components to Vapor bytecode for mobile
    println!("⚡ Compiling Vue components with Vapor mode...");
    
    // This would integrate with the Vue 3.6 Vapor compiler
    // and produce optimized bytecode for mobile execution
    
    run_command("npx", &["vue-vapor-compile", "src/", "--output", ".spruce/compiled/"], None)?;
    
    Ok(())
}

fn compile_vue_for_desktop(_config: &crate::config::SpruceConfig) -> Result<()> {
    // Compile Vue components for desktop
    run_command("npm", &["run", "build:vue"], None)?;
    Ok(())
}

fn check_android_sdk() -> Result<()> {
    if std::env::var("ANDROID_HOME").is_err() && std::env::var("ANDROID_SDK_ROOT").is_err() {
        return Err(SpruceError::Build(
            "Android SDK not found. Please set ANDROID_HOME".to_string()
        ));
    }
    Ok(())
}

fn check_ios_tools() -> Result<()> {
    if Command::new("xcodebuild").arg("-version").output().is_err() {
        return Err(SpruceError::Build(
            "Xcode not found. Please install Xcode".to_string()
        ));
    }
    Ok(())
}

fn create_android_project(config: &crate::config::SpruceConfig, android_config: &crate::config::AndroidConfig) -> Result<()> {
    let android_dir = Path::new("android");
    ensure_directory(android_dir)?;
    
    // Create build.gradle
    let build_gradle = format!(r#"
plugins {{
    id 'com.android.application'
    id 'org.jetbrains.kotlin.android'
}}

android {{
    compileSdk {}
    ndkVersion "25.1.8937393"
    
    defaultConfig {{
        applicationId "{}"
        minSdk {}
        targetSdk {}
        versionCode 1
        versionName "{}"
        
        ndk {{
            abiFilters 'arm64-v8a', 'armeabi-v7a'
        }}
    }}
    
    buildTypes {{
        release {{
            minifyEnabled true
            proguardFiles getDefaultProguardFile('proguard-android-optimize.txt'), 'proguard-rules.pro'
        }}
    }}
}}

dependencies {{
    implementation 'androidx.core:core-ktx:1.12.0'
    implementation 'androidx.lifecycle:lifecycle-runtime-ktx:2.7.0'
    implementation 'androidx.activity:activity-compose:1.8.2'
}}
"#, android_config.target_sdk, config.app.package, android_config.min_sdk, android_config.target_sdk, config.app.version);
    
    fs::write(android_dir.join("app/build.gradle"), build_gradle)?;
    
    // Create MainActivity.kt
    create_android_main_activity(android_dir, config)?;
    
    Ok(())
}

fn create_android_main_activity(android_dir: &Path, config: &crate::config::SpruceConfig) -> Result<()> {
    let main_activity = format!(r#"
package {}

import android.app.Activity
import android.os.Bundle
import android.view.SurfaceView

class MainActivity : Activity() {{
    
    companion object {{
        init {{
            System.loadLibrary("spruce")
        }}
    }}
    
    private external fun initRust(): Boolean
    private external fun createSurface(surface: android.view.Surface, width: Int, height: Int): Boolean
    
    override fun onCreate(savedInstanceState: Bundle?) {{
        super.onCreate(savedInstanceState)
        
        initRust()
        
        val surfaceView = SurfaceView(this)
        setContentView(surfaceView)
        
        surfaceView.holder.addCallback(object : android.view.SurfaceHolder.Callback {{
            override fun surfaceCreated(holder: android.view.SurfaceHolder) {{
                createSurface(holder.surface, surfaceView.width, surfaceView.height)
            }}
            override fun surfaceDestroyed(holder: android.view.SurfaceHolder) {{}}
            override fun surfaceChanged(holder: android.view.SurfaceHolder, format: Int, width: Int, height: Int) {{}}
        }})
    }}
}}
"#, config.app.package);
    
    let activity_dir = android_dir.join("app/src/main/java").join(config.app.package.replace(".", "/"));
    ensure_directory(&activity_dir)?;
    fs::write(activity_dir.join("MainActivity.kt"), main_activity)?;
    
    Ok(())
}

fn create_ios_project(_config: &crate::config::SpruceConfig, _ios_config: &crate::config::IosConfig) -> Result<()> {
    // Create iOS project structure
    let ios_dir = Path::new("ios");
    ensure_directory(ios_dir)?;
    
    // This would create a complete Xcode project
    // For brevity, we'll skip the full implementation
    
    Ok(())
}

fn copy_android_libraries(release: bool) -> Result<()> {
    let build_type = if release { "release" } else { "debug" };
    
    // Copy ARM64 library
    let src_arm64 = Path::new("target/aarch64-linux-android").join(build_type).join("libspruce.so");
    let dest_arm64 = Path::new("android/app/src/main/jniLibs/arm64-v8a/libspruce.so");
    ensure_directory(dest_arm64.parent().unwrap())?;
    fs::copy(src_arm64, dest_arm64)?;
    
    // Copy ARM7 library
    let src_arm7 = Path::new("target/armv7-linux-androideabi").join(build_type).join("libspruce.so");
    let dest_arm7 = Path::new("android/app/src/main/jniLibs/armeabi-v7a/libspruce.so");
    ensure_directory(dest_arm7.parent().unwrap())?;
    fs::copy(src_arm7, dest_arm7)?;
    
    Ok(())
}

fn copy_directory(src: &Path, dst: &Path) -> Result<()> {
    ensure_directory(dst)?;
    
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if src_path.is_dir() {
            copy_directory(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    
    Ok(())
}

fn print_build_summary(config: &crate::config::SpruceConfig, release: bool) {
    println!();
    println!("📊 Build Summary");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📱 App: {} v{}", config.app.name, config.app.version);
    println!("🔧 Mode: {}", if release { "Release" } else { "Debug" });
    println!("📁 Output: ./dist/");
    println!();
    
    if Path::new("dist").join(format!("{}-android.apk", config.app.name)).exists() {
        println!("✅ Android APK: dist/{}-android.apk", config.app.name);
    }
    
    println!();
    println!("🚀 Next steps:");
    println!("   spruce deploy --target play-store  # Deploy to Google Play");
    println!("   spruce deploy --target app-store   # Deploy to App Store");
}