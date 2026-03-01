use serde::{Serialize, Deserialize};
use super::commands::{Result, SpruceError};

#[derive(Debug, Serialize, Deserialize)]
pub struct SpruceConfig {
    pub app: AppConfig,
    pub performance: PerformanceConfig,
    pub platforms: PlatformsConfig,
    pub build: BuildConfig,
    pub dev: DevConfig,
    pub deploy: Option<DeployConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub name: String,
    pub package: String,
    pub version: String,
    pub icon: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceConfig {
    #[serde(rename = "targetFPS")]
    pub target_fps: u32,
    #[serde(rename = "maxMemoryMB")]
    pub max_memory_mb: u32,
    #[serde(rename = "batteryOptimized")]
    pub battery_optimized: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformsConfig {
    pub android: Option<AndroidConfig>,
    pub ios: Option<IosConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AndroidConfig {
    #[serde(rename = "minSdk")]
    pub min_sdk: u32,
    #[serde(rename = "targetSdk")]
    pub target_sdk: u32,
    pub features: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IosConfig {
    #[serde(rename = "minVersion")]
    pub min_version: String,
    pub features: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildConfig {
    pub optimization: String,
    #[serde(rename = "bundleAnalysis")]
    pub bundle_analysis: bool,
    pub treeshaking: bool,
    #[serde(rename = "rustOptimization")]
    pub rust_optimization: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DevConfig {
    #[serde(rename = "hotReload")]
    pub hot_reload: bool,
    #[serde(rename = "livePreview")]
    pub live_preview: bool,
    #[serde(rename = "debugTools")]
    pub debug_tools: bool,
    #[serde(rename = "aiAssistant")]
    pub ai_assistant: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployConfig {
    #[serde(rename = "appStore")]
    pub app_store: Option<AppStoreConfig>,
    #[serde(rename = "playStore")]
    pub play_store: Option<PlayStoreConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppStoreConfig {
    #[serde(rename = "teamId")]
    pub team_id: String,
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayStoreConfig {
    #[serde(rename = "packageName")]
    pub package_name: String,
    pub track: String,
}

impl SpruceConfig {
    pub fn new(app_name: &str) -> Self {
        let package_name = format!("com.example.{}", app_name.replace("-", "_").to_lowercase());
        
        Self {
            app: AppConfig {
                name: app_name.to_string(),
                package: package_name.clone(),
                version: "1.0.0".to_string(),
                icon: Some("./src/assets/icon.png".to_string()),
                description: Some(format!("A Spruce mobile app: {}", app_name)),
            },
            performance: PerformanceConfig {
                target_fps: 60,
                max_memory_mb: 100,
                battery_optimized: true,
            },
            platforms: PlatformsConfig {
                android: Some(AndroidConfig {
                    min_sdk: 21,
                    target_sdk: 34,
                    features: vec![
                        "camera".to_string(),
                        "location".to_string(),
                        "push-notifications".to_string(),
                    ],
                }),
                ios: Some(IosConfig {
                    min_version: "13.0".to_string(),
                    features: vec![
                        "camera".to_string(),
                        "location".to_string(),
                        "push-notifications".to_string(),
                    ],
                }),
            },
            build: BuildConfig {
                optimization: "aggressive".to_string(),
                bundle_analysis: true,
                treeshaking: true,
                rust_optimization: "release-lto".to_string(),
            },
            dev: DevConfig {
                hot_reload: true,
                live_preview: true,
                debug_tools: true,
                ai_assistant: true,
            },
            deploy: None, // User can configure this later
        }
    }
    
    pub fn to_typescript(&self) -> Result<String> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| SpruceError::Config(format!("Failed to serialize config: {}", e)))?;
        
        let typescript = format!(r#"export default {} satisfies SpruceConfig;

interface SpruceConfig {{
  app: {{
    name: string;
    package: string;
    version: string;
    icon?: string;
    description?: string;
  }};
  
  performance: {{
    targetFPS: number;
    maxMemoryMB: number;
    batteryOptimized: boolean;
  }};
  
  platforms: {{
    android?: {{
      minSdk: number;
      targetSdk: number;
      features: string[];
    }};
    ios?: {{
      minVersion: string;
      features: string[];
    }};
  }};
  
  build: {{
    optimization: string;
    bundleAnalysis: boolean;
    treeshaking: boolean;
    rustOptimization: string;
  }};
  
  dev: {{
    hotReload: boolean;
    livePreview: boolean;
    debugTools: boolean;
    aiAssistant: boolean;
  }};
  
  deploy?: {{
    appStore?: {{
      teamId: string;
      bundleId: string;
    }};
    playStore?: {{
      packageName: string;
      track: string;
    }};
  }};
}}
"#, json);
        
        Ok(typescript)
    }
}