use clap::{Parser, Subcommand};
use aplan::{builder::project_execution::{ProjectExecution, Return}, task::task_id::TaskId};

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

#[derive(Subcommand)]
enum WSBCommands {

    /// Write WSB visualization in DOT language to a file
    Dot {
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
    /// Remove task from WSB
    Remove {
        /// Id to the task to remove
        #[clap(short, long, value_parser)]
        id: String,
    },
    /// Mark a task as done
    Done {
        /// Task id
        #[clap(short, long, value_parser)]
        id: String,
        /// Cost it took to complete task
        #[clap(value_parser)]
        cost: f64
    },
    /// Set value of a task
    Value {
        /// Task id
        #[clap(short, long, value_parser)]
        id: String,
        /// Value of the task
        #[clap(value_parser)]
        value: f64
    },
    /// Get info about a task
    GetTask {
        /// Task id
        #[clap(short, long, value_parser)]
        id: String
    }
}

fn process_args(cli: Cli) -> Option<()>  {

    match &cli.command {
        Commands::Burndown {  } => todo!(),
        Commands::WSB { command } => match command {
            WSBCommands::Dot { output } => {
                ProjectExecution::load(&cli.project).unwrap()
                    .wsb(|wsb| {
                        wsb.dot(output);
                    })
            },
            WSBCommands::Add { parent, name } => {
                ProjectExecution::load(&cli.project).unwrap()
                    .wsb(|wsb| {
                        wsb.add(&TaskId::parse(&parent).unwrap(), name);
                    })
            },
            WSBCommands::Remove { id } => {
                ProjectExecution::load(&cli.project).unwrap()
                    .wsb(|wsb| {
                        wsb.remove(&TaskId::parse(&id).unwrap());
                    })
            },
            WSBCommands::Done { id, cost } => {
                ProjectExecution::load(&cli.project).unwrap()
                    .wsb(|wsb| {
                        wsb.done(&TaskId::parse(&id).unwrap(), *cost);
                    })
            },
            WSBCommands::Value { id, value } => {
                ProjectExecution::load(&cli.project).unwrap()
                    .wsb(|wsb| {
                        wsb.value(&TaskId::parse(&id).unwrap(), *value);
                    })
            },
            WSBCommands::GetTask { id } => {
                ProjectExecution::load(&cli.project).unwrap()
                    .wsb(|wsb| {
                        wsb.get_task(&TaskId::parse(&id).unwrap());
                    })
            }
        },
        Commands::Init { .. } => {
            ProjectExecution::new(&cli.project)
        },
    }
    .save()
    .run()
    // get results from operation that have something to return (some info to display)
    .iter()
    .for_each(|res| {
        match res {
            Return::Task(task) => {
                println!("{}", task.to_string())
            },
            Return::Project(_) => {},
        }
    });
    Some(())
}

fn main() {
    let cli = Cli::parse();
    process_args(cli);
}
