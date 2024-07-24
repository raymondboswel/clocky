mod commands;
mod db;
mod session;
mod utils;

use clap::{command, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Clocky")]
#[command(version = "0.1.0")]
#[command(author = "Your Name <your.email@example.com>")]
#[command(about = "Tracks time worked")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Start,
    End {
        #[arg(short, long)]
        datetime: Option<String>,
    },
    Today,
    Week,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Start => commands::start_session(),
        Commands::End { datetime } => commands::end_session(datetime.as_deref()),
        Commands::Today => commands::show_today(),
        Commands::Week => commands::show_week(),
    }
}
