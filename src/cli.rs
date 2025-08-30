use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Optional name to operate on
    // pub name: Option<String>,

    // /// Sets a custom config file
    // #[arg(short, long, value_name = "FILE")]
    // config: Option<PathBuf>,

    /// Turn debugging information on
    // #[arg(short, long, action = clap::ArgAction::Count)]
    // pub debug: u8,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Begins a session of a given workflow
    Begin {
        name: String,
    },
    /// Pauses the current workflow
    Pause {},
    /// Ends the current workflow
    End {}, 
    /// Prints the workflow status
    Status {
        #[arg(short, long)]
        show_icon: Option<bool>,
    }
}

pub fn get_cli() -> Cli {
    Cli::parse()
}
