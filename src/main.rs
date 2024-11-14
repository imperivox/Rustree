use anyhow::Result;
use clap::{Parser, Subcommand};
use dialoguer::{Select, Input, Confirm};
use rustree::{
    branch::{list_branches, cleanup_stale_branches},
    config::Config,
    visualization::generate_branch_graph,
};
use colored::*;

#[derive(Parser)]
#[command(name = "rtree")]
#[command(about = "Rustree - Git branch management and visualization", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all branches with their last commit date and author
    List {
        #[arg(short, long)]
        sort_by_date: bool,
    },
    /// Generate a visual representation of branch relationships
    Visualize {
        #[arg(short, long)]
        include_remote: bool,
    },
    /// Clean up stale branches
    Cleanup {
        #[arg(short, long)]
        dry_run: bool,
        #[arg(short, long, default_value = "30")]
        days: u32,
    },
}

fn show_welcome_message() {
    println!("\n{}", "Rustree".bright_green().bold());
    println!("{}", "================================".bright_green());
    println!("This tool helps you manage your Git branches easily.");
    println!("Make sure you're running this from within a Git repository.");
    println!();
}

fn run_cli() -> Result<()> {
    let config = Config::load()?;

    loop {
        let options = vec![
            "List all branches",
            "List branches (sorted by date)",
            "Visualize branch relationships",
            "Visualize branch relationships (including remote)",
            "Clean up old branches",
            "Exit",
        ];

        println!("\n{}", "What would you like to do?".cyan());
        let selection = Select::new()
            .items(&options)
            .default(0)
            .interact()?;

        match selection {
            0 => list_branches(&config, false)?,
            1 => list_branches(&config, true)?,
            2 => generate_branch_graph(false)?,
            3 => generate_branch_graph(true)?,
            4 => {
                println!("\n{}", "Branch Cleanup Options:".cyan());

                // Ask for days threshold
                let days: u32 = Input::new()
                    .with_prompt("How many days old should branches be before considering them stale? (default: 30)")
                    .default(30)
                    .interact()?;

                // Ask about dry run
                let dry_run = Confirm::new()
                    .with_prompt("Would you like to do a dry run first? (Recommended)")
                    .default(true)
                    .interact()?;

                cleanup_stale_branches(&config, days, dry_run)?;
            },
            5 => {
                println!("\nThank you for using Rustree!");
                break;
            }
            _ => unreachable!(),
        }

        println!("\nPress Enter to continue...");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
    }

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // If no command line arguments are provided, run in interactive mode
    match cli.command {
        None => {
            show_welcome_message();
            run_cli()
        }
        Some(command) => {
            let config = Config::load()?;
            match command {
                Commands::List { sort_by_date } => {
                    list_branches(&config, sort_by_date)
                }
                Commands::Visualize { include_remote } => {
                    generate_branch_graph(include_remote)
                }
                Commands::Cleanup { dry_run, days } => {
                    cleanup_stale_branches(&config, days, dry_run)
                }
            }
        }
    }
}