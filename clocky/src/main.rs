mod commands;
mod db;
mod session;
mod utils;

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("Clocky")
        .version("0.1.0")
        .author("Your Name <your.email@example.com>")
        .about("Tracks time worked")
        .subcommand(SubCommand::with_name("start").about("Starts a work session"))
        .subcommand(
            SubCommand::with_name("end")
                .about("Ends a work session")
                .arg(
                    Arg::with_name("datetime")
                        .help("The end time of the session")
                        .required(false),
                ),
        )
        .subcommand(SubCommand::with_name("today").about("Shows the time worked today"))
        .subcommand(SubCommand::with_name("week").about("Shows the time worked this week"))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("start") {
        commands::start_session();
    } else if let Some(matches) = matches.subcommand_matches("end") {
        if let Some(datetime) = matches.value_of("datetime") {
            commands::end_session(Some(datetime));
        } else {
            commands::end_session(None);
        }
    } else if let Some(_) = matches.subcommand_matches("today") {
        commands::show_today();
    } else if let Some(_) = matches.subcommand_matches("week") {
        commands::show_week();
    }
}
