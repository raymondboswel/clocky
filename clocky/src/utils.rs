use chrono::Duration;

pub fn format_duration(duration: Duration) -> String {
    let hours = duration.num_hours();
    let minutes = duration.num_minutes() % 60;
    format!("{} hours, {} minutes", hours, minutes)
}
