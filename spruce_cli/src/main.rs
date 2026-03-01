use std::env;
use std::process;

mod commands;
mod config;
mod templates;
mod utils;

use commands::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "create" => {
            if args.len() < 3 {
                eprintln!("❌ Error: App name required");
                eprintln!("Usage: spruce create <app-name> [--template <template>]");
                process::exit(1);
            }
            
            let app_name = &args[2];
            let template = get_template_arg(&args);
            
            match create_app(app_name, template) {
                Ok(_) => println!("✅ Created Spruce app: {}", app_name),
                Err(e) => {
                    eprintln!("❌ Error creating app: {}", e);
                    process::exit(1);
                }
            }
        }
        
        "dev" => {
            let platform = get_platform_arg(&args);
            let device = get_device_arg(&args);
            
            match start_dev_server(platform, device) {
                Ok(_) => println!("🚀 Development server started"),
                Err(e) => {
                    eprintln!("❌ Error starting dev server: {}", e);
                    process::exit(1);
                }
            }
        }
        
        "build" => {
            let release = args.contains(&"--release".to_string());
            let platform = get_platform_arg(&args);
            
            match build_project(release, platform) {
                Ok(_) => println!("🎯 Build completed successfully"),
                Err(e) => {
                    eprintln!("❌ Build failed: {}", e);
                    process::exit(1);
                }
            }
        }
        
        "deploy" => {
            let target = get_deploy_target(&args);
            
            match deploy_app(target) {
                Ok(_) => println!("🚀 Deployment successful"),
                Err(e) => {
                    eprintln!("❌ Deployment failed: {}", e);
                    process::exit(1);
                }
            }
        }
        
        "ai" => {
            if args.len() < 3 {
                eprintln!("❌ Error: AI command required");
                eprintln!("Usage: spruce ai <generate|optimize|review|debug>");
                process::exit(1);
            }
            
            let ai_command = &args[2];
            let ai_args = &args[3..];
            
            match run_ai_command(ai_command, ai_args) {
                Ok(_) => println!("🤖 AI task completed"),
                Err(e) => {
                    eprintln!("❌ AI task failed: {}", e);
                    process::exit(1);
                }
            }
        }
        
        "version" | "--version" | "-v" => {
            println!("🌲 Spruce CLI v1.0.0");
            println!("The next-generation mobile development platform");
        }
        
        "help" | "--help" | "-h" => {
            print_help();
        }
        
        _ => {
            eprintln!("❌ Unknown command: {}", args[1]);
            eprintln!("Run 'spruce help' for available commands");
            process::exit(1);
        }
    }
}

fn print_help() {
    println!("🌲 Spruce CLI - Next-generation mobile development");
    println!();
    println!("USAGE:");
    println!("    spruce <COMMAND> [OPTIONS]");
    println!();
    println!("COMMANDS:");
    println!("    create    Create a new Spruce app");
    println!("    dev       Start development server with hot reload");
    println!("    build     Build the project for production");
    println!("    deploy    Deploy to app stores or cloud");
    println!("    ai        AI-powered development assistance");
    println!("    version   Show version information");
    println!("    help      Show this help message");
    println!();
    println!("EXAMPLES:");
    println!("    spruce create MyApp --template vue-mobile");
    println!("    spruce dev --platform android --device pixel-7");
    println!("    spruce build --release");
    println!("    spruce deploy --target app-store,play-store");
    println!("    spruce ai generate --feature \"user auth\"");
    println!();
    println!("For more information, visit: https://spruce.dev/docs");
}

fn get_template_arg(args: &[String]) -> Option<&str> {
    for i in 0..args.len() {
        if args[i] == "--template" && i + 1 < args.len() {
            return Some(&args[i + 1]);
        }
    }
    None
}

fn get_platform_arg(args: &[String]) -> Option<&str> {
    for i in 0..args.len() {
        if args[i] == "--platform" && i + 1 < args.len() {
            return Some(&args[i + 1]);
        }
    }
    None
}

fn get_device_arg(args: &[String]) -> Option<&str> {
    for i in 0..args.len() {
        if args[i] == "--device" && i + 1 < args.len() {
            return Some(&args[i + 1]);
        }
    }
    None
}

fn get_deploy_target(args: &[String]) -> Option<&str> {
    for i in 0..args.len() {
        if args[i] == "--target" && i + 1 < args.len() {
            return Some(&args[i + 1]);
        }
    }
    None
}