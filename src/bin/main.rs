use std::str::FromStr;

use clap::{Parser, Subcommand};
use aplan::{builder::project_execution::{ProjectExecution, Return}, task::task_id::TaskId, error::Error, util};

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
    /// Manage project members
    Member {
        #[clap(subcommand)]
        command: MemberCommands
    },
    /// Create project file
    Init { }
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
enum MemberCommands {
    /// List members in the project
    List { },
    /// Add member to project
    Add {
        /// Name of the member
        #[clap(value_parser)]
        name: String
    },
    /// Remove member from project
    Remove {
        /// Name of the member
        #[clap(value_parser)]
        name: String
    },
    /// Assign task to member
    Assign {
        /// Name of the member
        #[clap(value_parser)]
        name: String,
        /// Task id
        #[clap(value_parser = task_id_parser)]
        id: TaskId
    },
    /// Remove Member from task
    RemoveTask {
        /// Name of the member
        #[clap(value_parser)]
        name: String,
        /// Task id
        #[clap(value_parser = task_id_parser)]
        id: TaskId
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
        #[clap(value_parser)]
        name: String
    },
    /// Remove task from WSB
    Remove {
        /// Id to the task to remove
        #[clap(value_parser = task_id_parser)]
        id: TaskId,
    },
    /// Mark a task as done
    Done {
        /// Task id
        #[clap(value_parser = task_id_parser)]
        id: TaskId,
        /// Cost it took to complete task
        #[clap(value_parser)]
        cost: f64
    },
    /// Set value of a task
    PlannedValue {
        /// Task id
        #[clap(value_parser = task_id_parser)]
        id: TaskId,
        /// Value of the task
        #[clap(value_parser)]
        value: f64
    },
    /// Get info about a task
    GetTask {
        /// Task id
        #[clap(value_parser = task_id_parser)]
        id: TaskId
    },
}

fn task_id_parser(s: &str) -> Result<TaskId, String> {
    TaskId::parse(s).or_else(|_| Err(s.to_string()))
}

fn process_wsb(command: &WSBCommands, project_name: &str) -> Result<ProjectExecution, Error> {
    Ok(match command {
        WSBCommands::Show { format, output } => {
            ProjectExecution::load(&project_name)?
                .wsb(|wsb| {
                    match format {
                        ShowFormat::Dot => wsb.dot(output.as_deref()),
                        ShowFormat::Text => wsb.tree(output.as_deref()),
                    };
                })
        },
        WSBCommands::Add { parent, name } => {
            ProjectExecution::load(&project_name)?
                .wsb(|wsb| {
                    wsb.add(parent.as_ref().unwrap_or(&TaskId::get_root_id()), name);
                })
        },
        WSBCommands::Remove { id } => {
            ProjectExecution::load(&project_name)?
                .wsb(|wsb| {
                    wsb.remove(&id);
                })
        },
        WSBCommands::Done { id, cost } => {
            ProjectExecution::load(&project_name)?
                .wsb(|wsb| {
                    wsb.done(&id, *cost);
                })
        },
        WSBCommands::PlannedValue { id, value } => {
            ProjectExecution::load(&project_name)?
                .wsb(|wsb| {
                    wsb.planned_value(&id, *value);
                })
        },
        WSBCommands::GetTask { id } => {
            ProjectExecution::load(&project_name)?
                .wsb(|wsb| {
                    wsb.get_task(&id);
                })
        }
    })
}

fn process_member(command: &MemberCommands, project_name: &str) -> Result<ProjectExecution, Error> {
    Ok(match command {
        MemberCommands::List {  } => {
            ProjectExecution::load(&project_name)?
            .member(|member| {
                member.list_members();
            })
        },
        MemberCommands::Add { name } => {
            ProjectExecution::load(&project_name)?
            .member(|member| {
                member.add_member(name);
            })
        },
        MemberCommands::Remove { name } => {
            ProjectExecution::load(&project_name)?
            .member(|member| {
                member.remove_member(name);
            })
        },
        MemberCommands::Assign { name, id } => {
            ProjectExecution::load(&project_name)?
            .member(|member| {
                member.assign_task_to_member(id.clone(), name);
            })
        },
        MemberCommands::RemoveTask { name, id } => {
            ProjectExecution::load(&project_name)?
            .member(|member| {
                member.remove_member_from_task(id.clone(), name);
            })
        },
    })
}

fn process_args(cli: Cli) -> Result<(), Error>  {

    let project_name = &cli.project;
    match &cli.command {
        Commands::Burndown {  } => todo!(),
        Commands::Init { .. } => {
            ProjectExecution::new(&cli.project)
        },
        Commands::Member { command } => {
            process_member(command, project_name)?
        },
        Commands::WSB { command } => {
            process_wsb(command, project_name)?
        }
    }
    .save()
    .run()?
    .iter()
    .try_for_each(|res| -> Result<(), Error> {
        match res {
            Return::Task(task) => {
                println!("{}", task.to_string())
            }
            Return::VisualizationDot(filename, text) | Return::VisualizationTree(filename, text) => {
                util::to_file(filename.as_deref(), text.to_string())
            },
            Return::MembersList(members) => {
                members.iter().for_each(|member| println!("{}", member))
            },
        }
        Ok(())
    })
}

fn main() {
    let cli = Cli::parse();
    if let Err(e) = process_args(cli) {
        println!("Error: {}", e);
    }
}
