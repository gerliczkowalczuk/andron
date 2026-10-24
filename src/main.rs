use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod binder;
mod wayland;
mod sandbox;
mod loader;

#[derive(Parser)]
#[command(name = "andron")]
#[command(about = "Ultra-lightweight Android translation & compatibility runtime for Linux", long_about = None)]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Launch an Android APK through the optimized direct execution layer
    Run {
        /// Path to the APK file
        apk_path: PathBuf,
        /// Passthrough arguments to the guest application
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Inspect Android application headers, DEX byte-codes, and native ELF libraries
    Inspect {
        /// Path to the APK file
        apk_path: PathBuf,
    },
    /// Audit host system kernel nodes, Binder, Wayland, and namespace capabilities
    Check,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { apk_path, args } => {
            println!("==> [andron] Initializing ultra-lightweight Android execution environment...");
            sandbox::setup_namespaces()?;
            binder::init_binder_subsystem()?;
            wayland::connect_display_bridge()?;
            loader::execute_app(&apk_path, &args)?;
        }
        Commands::Inspect { apk_path } => {
            loader::inspect_package(&apk_path)?;
        }
        Commands::Check => {
            sandbox::check_environment();
        }
    }

    Ok(())
}
