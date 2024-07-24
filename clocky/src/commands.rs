use crate::session;

pub fn start_session() {
    match session::start() {
        Ok(_) => println!("Session started."),
        Err(e) => eprintln!("Error starting session: {}", e),
    }
}

pub fn end_session(datetime: Option<&str>) {
    match session::end(datetime) {
        Ok(_) => println!("Session ended."),
        Err(e) => eprintln!("Error ending session: {}", e),
    }
}

pub fn show_today() {
    match session::time_worked_today() {
        Ok(duration) => println!("Time worked today: {}", utils::format_duration(duration)),
        Err(e) => eprintln!("Error calculating time worked today: {}", e),
    }
}

pub fn show_week() {
    match session::time_worked_week() {
        Ok(duration) => println!(
            "Time worked this week: {}",
            utils::format_duration(duration)
        ),
        Err(e) => eprintln!("Error calculating time worked this week: {}", e),
    }
}
