use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    /// Project file to operate on
    #[clap(short, long, value_parser, default_value = ".aplan.ap")]
    filename: String,

    #[clap(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    /// Manage tasks and the members allocated to them
    WSB {
    },
    /// Arrange tasks in a priority queue, where tasks have status of "not started", "in progress"
    /// or "done"
    Burndown {

    }
}

fn main() {
    let cli = Cli::parse();
}
