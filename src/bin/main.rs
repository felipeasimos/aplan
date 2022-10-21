use std::str::FromStr;

use clap::{Parser, Subcommand};
use aplan::prelude::*;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    /// File from which we will load and save the project
    #[clap(short, long, value_parser, default_value = util::DEFAULT_FILENAME)]
    filename: String,

    #[clap(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    /// Manage tasks and the members allocated to them
    Task {
        #[clap(subcommand)]
        command: TaskCommands
    },
    /// Manage project members
    Member {
        #[clap(subcommand)]
        command: MemberCommands
    },
    /// Create project file
    Init {
        /// Name of the project to create
        #[clap(short, long, value_parser, default_value = "aplan")]
        project: String,
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
enum TaskCommands {

    /// Visualize the Task
    Show {
        /// Get tree in visualization string
        #[clap(short, long, default_value = "text")]
        format: ShowFormat,

        /// File to write the visualization in DOT language
        #[clap(short, long, value_parser)]
        output: Option<String>,
    },
    /// Add new task to Task
    Add {
        /// Parent id
        #[clap(short, long, value_parser = task_id_parser)]
        parent: Option<TaskId>,
        /// Name of the task
        #[clap(value_parser)]
        name: String
    },
    /// Remove task from Task
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

fn process_tasks(command: &TaskCommands, project_filename: &str) -> Result<Project, Error> {
    let mut project = Project::load(project_filename)?;
    match command {
        TaskCommands::Show { format, output } => {
            match format {
                ShowFormat::Dot => util::to_file(output.as_deref(), project.tasks().to_dot_str())?,
                ShowFormat::Text => util::to_file(output.as_deref(), project.tasks().to_tree_str())?,
            }
        },
        TaskCommands::Add { parent, name } => {
            project.tasks_mut(|tasks| {
                tasks.add(parent.clone().unwrap_or(TaskId::get_root_id()), name)?;
                Ok(())
            })?;
        },
        TaskCommands::Remove { id } => {
            project.tasks_mut(|tasks| {
                tasks.remove(id)?;
                Ok(())
            })?;
        },
        TaskCommands::Done { id, cost } => {
            project.tasks_mut(|tasks| {
                tasks.done(&id, *cost)?;
                Ok(())
            })?;
        },
        TaskCommands::PlannedValue { id, value } => {
            project.tasks_mut(|tasks| {
                tasks.planned_value(id, *value)?;
                Ok(())
            })?;
        },
        TaskCommands::GetTask { id } => {
            println!("{}", project.tasks().get(id)?);
        },
        TaskCommands::Todo { name } => {
            if let Some(name) = name {
                let member = project.members().get(name)?.clone();
                project.tasks().get_todo_tasks().filter(|t| member.is_assigned_to(t.id())).for_each(|t| println!("{}" ,t));
            } else {
                project.tasks().get_todo_tasks().for_each(|t| println!("{}" ,t));
            }
        },
    }
    Ok(project)
}

fn process_member(command: &MemberCommands, project_filename: &str) -> Result<Project, Error> {
    let mut project = Project::load(project_filename)?;
    Ok(match command {
        MemberCommands::List {  } => {
            project
            .members_mut(|member| {
                member.list_members().for_each(|m| println!("{}", m));
                Ok(())
            })
        },
        MemberCommands::Add { name } => {
            project
            .members_mut(|member| {
                member.add_member(name)?;
                Ok(())
            })
        },
        MemberCommands::Remove { name } => {
            project
            .members_mut(|member| {
                member.remove_member(name)?;
                Ok(())
            })
        },
        MemberCommands::Assign { name, id } => {
            project
            .members_mut(|member| {
                member.assign_task_to_member(id.clone(), name)?;
                Ok(())
            })
        },
        MemberCommands::RemoveTask { name, id } => {
            project
            .members_mut(|member| {
                member.remove_member_from_task(id.clone(), name)?;
                Ok(())
            })
        },
    }?
    .clone())
}

fn process_args(cli: Cli) -> Result<(), Error>  {

    match &cli.command {
        Commands::Init { project } => {
            Project::new(project)
        },
        Commands::Member { command } => {
            process_member(command, &cli.filename)?
        },
        Commands::Task { command } => {
            process_tasks(command, &cli.filename)?
        }
    }
    .save_to(&cli.filename)?;
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    if let Err(e) = process_args(cli) {
        println!("Error: {}", e);
    }
}
