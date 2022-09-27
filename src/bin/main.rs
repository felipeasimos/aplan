use clap::{Parser, Subcommand};
use aplan::{project::Project, builder::project_execution::ProjectExecution, task::task_id::TaskId};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    /// Name of the project to operate on
    #[clap(short, long, value_parser, default_value = "aplan")]
    name: String,

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

fn process_args(cli: Cli) -> Option<()>  {
    if let Commands::Init { name } = cli.command {
        let project = Project::new(&name);
        project.save();
        return Some(());
    }
    let mut project_execution = ProjectExecution::load(&cli.name).unwrap();

    project_execution = match &cli.command {
        Commands::Burndown {  } => todo!(),
        Commands::WSB { command } => match command {
            WSBCommands::Show { output } => {
                project_execution
                    .wsb(|wsb| {
                        wsb.show(output);
                    })
            },
            WSBCommands::Add { parent, name } => {
                project_execution
                    .wsb(|wsb| {
                        wsb.add(&TaskId::parse(&parent).unwrap(), name);
                    })
            },
            WSBCommands::Done { id, cost } => {
                project_execution
                    .wsb(|wsb| {
                        wsb.done(&TaskId::parse(&id).unwrap(), *cost);
                    })
            },
        },
        Commands::Init { .. } => project_execution,
    };
    project_execution
        .save()
        .run();
    Some(())
}

fn main() {
    let cli = Cli::parse();
    process_args(cli);
}
