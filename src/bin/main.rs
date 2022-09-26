use std::{path::PathBuf, str::FromStr, fs::File};

use clap::{Parser, Subcommand};
use aplan::{prelude::*, project::Project};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    /// Project file to operate on
    #[clap(short, long, value_parser, default_value = ".aplan.ap")]
    filename: String,

    #[clap(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    /// Manage tasks and the members allocated to them
    WSB {
        #[clap(subcommand)]
        command: WSBCommands
    },
    /// Arrange tasks in a priority queue, where tasks have status of "not started", "in progress"
    /// or "done"
    Burndown {

    }
}

#[derive(Subcommand)]
enum WSBCommands {

    /// Write WSB visualization in DOT language to a file
    Show {
        filename: String
    }
}

fn process_args(cli: Cli) -> Option<()> {
    let filename = cli.filename;

    match &cli.command {
        Commands::Burndown {  } => todo!(),
        Commands::WSB { command } => match command {
            WSBCommands::Show { filename } => {
                
            },
        },
    };
    Some(())
}

fn main() {
    let cli = Cli::parse();
    process_args(cli);
}
