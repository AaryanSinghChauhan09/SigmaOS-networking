//! SigmaOS-networking - SigmaOS Component CLI
//!
//! Command-line interface for SigmaOS-networking component.

use anyhow::Result;
use clap::{Parser, Subcommand};
use log::info;
use networking::{Component, ComponentStatus, Config, networking};

#[derive(Parser)]
#[command(name = "networking")]
#[command(about = "SigmaOS SigmaOS-networking component", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the component
    Init {
        /// Enable debug mode
        #[arg(short, long)]
        debug: bool,
    },
    /// Start the component
    Start,
    /// Stop the component
    Stop,
    /// Get component status
    Status,
    /// Run the component
    Run {
        /// Enable debug mode
        #[arg(short, long)]
        debug: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    let config = Config::default();
    let mut component = networking::new(config);

    match cli.command {
        Commands::Init { debug } => {
            if debug {
                println!("Initializing SigmaOS-networking in debug mode...");
            } else {
                println!("Initializing SigmaOS-networking...");
            }
            component.initialize()?;
            println!("SigmaOS-networking initialized successfully");
        }
        Commands::Start => {
            component.initialize()?;
            component.start()?;
            println!("SigmaOS-networking started successfully");
        }
        Commands::Stop => {
            component.stop()?;
            println!("SigmaOS-networking stopped successfully");
        }
        Commands::Status => {
            let status = component.status();
            println!("SigmaOS-networking status: {:?}", status);
        }
        Commands::Run { debug } => {
            if debug {
                println!("Running SigmaOS-networking in debug mode...");
            } else {
                println!("Running SigmaOS-networking...");
            }
            component.initialize()?;
            component.start()?;
            println!("SigmaOS-networking is running. Press Ctrl+C to stop.");
            
            // Keep running
            tokio::runtime::Runtime::new()?.block_on(async {
                tokio::signal::ctrl_c().await.unwrap();
                info!("Received Ctrl+C, shutting down...");
            });
            
            component.stop()?;
            println!("SigmaOS-networking stopped");
        }
    }

    Ok(())
}
