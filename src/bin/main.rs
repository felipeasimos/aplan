use std::str::FromStr;

use clap::{Parser, Subcommand};
use aplan::prelude::*;

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
        /// Name of the member to insert
        #[clap(value_parser)]
        name: String
    },
    /// Remove member from project
    Remove {
        /// Name of the member to remove
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
    /// Get list of task todo
    Todo {
        /// Show only this member's incomplete tasks
        name: Option<String>
    }
}

fn task_id_parser(s: &str) -> Result<TaskId, String> {
    TaskId::parse(s).or_else(|_| Err(s.to_string()))
}

fn process_wsb(command: &WSBCommands, project_name: &str) -> Result<Aplan, Error> {
    let mut project = Aplan::load(&project_name)?;
    Ok(match command {
        WSBCommands::Show { format, output } => {
            project
                .wsb(|wsb| {
                    match format {
                        ShowFormat::Dot => util::to_file(output.as_deref(), wsb.dot()),
                        ShowFormat::Text => util::to_file(output.as_deref(), wsb.tree()),
                    };
                    Ok(())
                })
        },
        WSBCommands::Add { parent, name } => {
            project
                .wsb(|wsb| {
                    wsb.add(parent.clone().unwrap_or(TaskId::get_root_id()), name)?;
                    Ok(())
                })
        },
        WSBCommands::Remove { id } => {
            project
                .wsb(|wsb| {
                    wsb.remove(&id)?;
                    Ok(())
                })
        },
        WSBCommands::Done { id, cost } => {
            project
                .wsb(|wsb| {
                    wsb.done(&id, *cost)?;
                    Ok(())
                })
        },
        WSBCommands::PlannedValue { id, value } => {
            project
                .wsb(|wsb| {
                    wsb.planned_value(&id, *value)?;
                    Ok(())
                })
        },
        WSBCommands::GetTask { id } => {
            project
                .wsb(|wsb| {
                    println!("{}", wsb.get_task(&id)?);
                    Ok(())
                })
        }
        WSBCommands::Todo { name } => {
            project
                .wsb(|wsb| {
                    match name {
                        Some(name) => wsb.get_todo_tasks().filter(|t| t.has_member(name)).for_each(|t| println!("{}", t)),
                        None => wsb.get_todo_tasks().for_each(|t| println!("{}", t)),
                    }
                    Ok(())
                })
        },
    }?.clone())
}

fn process_member(command: &MemberCommands, project_name: &str) -> Result<Aplan, Error> {
    let mut project = Aplan::load(&project_name)?;
    Ok(match command {
        MemberCommands::List {  } => {
            project
            .members(|member| {
                member.list_members().for_each(|m| println!("{}", m));
                Ok(())
            })
        },
        MemberCommands::Add { name } => {
            project
            .members(|member| {
                member.add_member(name)?;
                Ok(())
            })
        },
        MemberCommands::Remove { name } => {
            project
            .members(|member| {
                member.remove_member(name)?;
                Ok(())
            })
        },
        MemberCommands::Assign { name, id } => {
            project
            .members(|member| {
                member.assign_task_to_member(id.clone(), name)?;
                Ok(())
            })
        },
        MemberCommands::RemoveTask { name, id } => {
            project
            .members(|member| {
                member.remove_member_from_task(id.clone(), name)?;
                Ok(())
            })
        },
    }?
    .clone())
}

fn process_args(cli: Cli) -> Result<(), Error>  {

    let project_name = &cli.project;
    match &cli.command {
        Commands::Burndown {  } => todo!(),
        Commands::Init { .. } => {
            Aplan::new(&cli.project)
        },
        Commands::Member { command } => {
            process_member(command, project_name)?
        },
        Commands::WSB { command } => {
            process_wsb(command, project_name)?
        }
    }
    .save()?;
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    if let Err(e) = process_args(cli) {
        println!("Error: {}", e);
    }
}
