use clap::{Parser, Subcommand};
use colored::*;
use anyhow::Result;
use std::path::PathBuf;

mod commands;
mod templates;

#[derive(Parser)]
#[command(
    name = "spruce",
    about = "Spruce - Ultra-Fast Vue 3 + Rust + SpruceVM Mobile Framework",
    version = "0.1.0"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Spruce app
    Create {
        /// App name
        name: String,
        /// Template to use
        #[arg(short, long, default_value = "basic")]
        template: String,
        /// Output directory
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Start development server
    Dev {
        /// Port to run on
        #[arg(short, long, default_value = "3000")]
        port: u16,
        /// Platform to target
        #[arg(short = 't', long, default_value = "ios")]
        platform: String,
    },
    /// Build for production
    Build {
        /// Platform to build for
        #[arg(short, long, default_value = "ios")]
        platform: String,
        /// Release mode
        #[arg(short, long)]
        release: bool,
    },
    /// Run on device/simulator
    Run {
        /// Platform to run on
        #[arg(short, long, default_value = "ios")]
        platform: String,
        /// Device ID
        #[arg(short, long)]
        device: Option<String>,
    },
    /// Doctor - check environment setup
    Doctor,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    print_banner();
    
    match cli.command {
        Commands::Create { name, template, output } => {
            commands::create::create_project(name, template, output).await?;
        }
        Commands::Dev { port, platform } => {
            println!("Dev server not implemented yet");
        }
        Commands::Build { platform, release } => {
            println!("Build not implemented yet");
        }
        Commands::Run { platform, device } => {
            println!("Run not implemented yet");
        }
        Commands::Doctor => {
            println!("Doctor not implemented yet");
        }
    }
    
    Ok(())
}

fn print_banner() {
    println!("{}", "
███████╗██████╗ ██████╗ ██╗   ██╗ ██████╗███████╗
██╔════╝██╔══██╗██╔══██╗██║   ██║██╔════╝██╔════╝
███████╗██████╔╝██████╔╝██║   ██║██║     █████╗  
╚════██║██╔═══╝ ██╔══██╗██║   ██║██║     ██╔══╝  
███████║██║     ██║  ██║╚██████╔╝╚██████╗███████╗
╚══════╝╚═╝     ╚═╝  ╚═╝ ╚═════╝  ╚═════╝╚══════╝

      🌲 Rust + 🟢 Vue 3 + ⚡ SpruceVM = 🚀 Ultra Performance
".bright_cyan());
}