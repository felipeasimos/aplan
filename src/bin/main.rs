use std::{path::PathBuf, str::FromStr, fs::File, io::Write};

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

    },
    /// Create project file
    Init {
        /// Project name
        #[clap(short, long, value_parser, default_value = "aplan")]
        name: String
    }
}

#[derive(Subcommand)]
enum WSBCommands {

    /// Write WSB visualization in DOT language to a file
    Show {
        /// File to write the visualization to
        #[clap(short, long, value_parser)]
        output: String
    },
    /// Add new task to WSB
    Add {
        /// Parent id
        #[clap(short, long, value_parser, default_value = "")]
        parent: String,
        /// Name of the task
        #[clap(short, long, value_parser)]
        name: String
    },
    /// Mark a task as done
    Done {
        /// Task id
        #[clap(short, long, value_parser)]
        id: String,
        /// Cost it took to complete task
        #[clap(short, long, value_parser)]
        cost: f64
    }
}

fn save_project(project: Project, file: &str) -> Option<()> {
    write!(std::fs::File::create(file).ok()?, "{}", project.to_json().ok()?).ok()
}

fn project_name_from_filename(filename: &str) -> Option<String> {

    let filename = PathBuf::from_str(filename).ok()?;
    let name : String = filename.file_stem()?.to_str()?[1..].to_string();
    Some(name)
}

fn filename_from_project_name(name: &str) -> Option<String> {

    let filename = ".".to_string() + &name;
    let mut filename = PathBuf::from_str(&filename).ok()?;
    filename.set_extension("ap");
    Some(filename.to_str()?.to_string())
}

fn process_args(cli: Cli) -> Option<()>  {
    if let Commands::Init { name } = cli.command {
        let filename = filename_from_project_name(&name)?;
        let project = Project::new(&name);
        let json = project.to_json().unwrap();
        return write!(File::create(filename).ok()?, "{}", json).ok();
    }

    let file_contents = std::fs::read_to_string(&cli.filename).unwrap();
    let mut project = Project::from_json(&file_contents).ok()?;

    match &cli.command {
        Commands::Burndown {  } => todo!(),
        Commands::WSB { command } => match command {
            WSBCommands::Show { output } => {
                write!(File::create(&output).ok()?, "{}", project.wsb().to_dot_str()).unwrap()
            },
            WSBCommands::Add { parent, name } => {
                project.wsb().add_task(parent, name).unwrap();
            },
            WSBCommands::Done { id, cost } => {
                project.wsb().set_actual_cost(id, *cost).unwrap();
            },
        },
        Commands::Init { .. } => {},
    };
    save_project(project, &cli.filename);
    Some(())
}

fn main() {
    let cli = Cli::parse();
    process_args(cli);
}
