use std::str::FromStr;

use clap::{Parser, Subcommand};
use aplan::{builder::project_execution::{ProjectExecution, Return}, task::task_id::TaskId, error::Error};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    /// Name of the project to operate on
    #[clap(short, long, value_parser, default_value = "aplan")]
    project: String,

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
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
enum ShowFormat {
    Dot,
    Text
}

impl FromStr for ShowFormat {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "dot" => Ok(ShowFormat::Dot),
            "text" | "txt" | "tree" => Ok(ShowFormat::Text),
            _ => Err(Error::ParseCliArgument(s.to_string()))
        }
    }
}

#[derive(Subcommand)]
enum WSBCommands {

    /// Visualize the WSB
    Show {
        /// Get tree in visualization string
        #[clap(short, long, default_value = "text")]
        format: ShowFormat,

        /// File to write the visualization in DOT language
        #[clap(short, long, value_parser)]
        output: Option<String>,
    },
    /// Add new task to WSB
    Add {
        /// Parent id
        #[clap(short, long, value_parser = task_id_parser)]
        parent: Option<TaskId>,
        /// Name of the task
        #[clap(short, long, value_parser)]
        name: String
    },
    /// Remove task from WSB
    Remove {
        /// Id to the task to remove
        #[clap(short, long, value_parser = task_id_parser)]
        id: TaskId,
    },
    /// Mark a task as done
    Done {
        /// Task id
        #[clap(short, long, value_parser = task_id_parser)]
        id: TaskId,
        /// Cost it took to complete task
        #[clap(value_parser)]
        cost: f64
    },
    /// Set value of a task
    PlannedValue {
        /// Task id
        #[clap(short, long, value_parser = task_id_parser)]
        id: TaskId,
        /// Value of the task
        #[clap(value_parser)]
        value: f64
    },
    /// Get info about a task
    GetTask {
        /// Task id
        #[clap(short, long, value_parser = task_id_parser)]
        id: TaskId
    },
}

fn task_id_parser(s: &str) -> Result<TaskId, String> {
    TaskId::parse(s).or_else(|_| Err(s.to_string()))
}

fn process_args(cli: Cli) -> Result<(), Error>  {

    match &cli.command {
        Commands::Burndown {  } => todo!(),
        Commands::Init { .. } => {
            ProjectExecution::new(&cli.project)
        },
        Commands::WSB { command } => match command {
            WSBCommands::Show { format, output } => {
                ProjectExecution::load(&cli.project)?
                    .wsb(|wsb| {
                        match format {
                            ShowFormat::Dot => wsb.dot(output.as_deref()),
                            ShowFormat::Text => wsb.tree(output.as_deref()),
                        };
                    })
            },
            WSBCommands::Add { parent, name } => {
                ProjectExecution::load(&cli.project)?
                    .wsb(|wsb| {
                        wsb.add(parent.as_ref().unwrap_or(&TaskId::get_root_id()), name);
                    })
            },
            WSBCommands::Remove { id } => {
                ProjectExecution::load(&cli.project)?
                    .wsb(|wsb| {
                        wsb.remove(&id);
                    })
            },
            WSBCommands::Done { id, cost } => {
                ProjectExecution::load(&cli.project)?
                    .wsb(|wsb| {
                        wsb.done(&id, *cost);
                    })
            },
            WSBCommands::PlannedValue { id, value } => {
                ProjectExecution::load(&cli.project)?
                    .wsb(|wsb| {
                        wsb.planned_value(&id, *value);
                    })
            },
            WSBCommands::GetTask { id } => {
                ProjectExecution::load(&cli.project)?
                    .wsb(|wsb| {
                        wsb.get_task(&id);
                    })
            }
        },
    }
    .save()
    .run()?
    // get results from operation that have something to return (some info to display)
    .iter()
    .try_for_each(|res| -> Result<(), Error> {
        match res {
            Return::Task(task) => {
                println!("{}", task.to_string())
            }
            Return::Text(text_str) => {
                println!("{}", text_str.to_string())
            },
        }
        Ok(())
    });
    Ok(())
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    process_args(cli)
}
