use crate::db;
use chrono::{DateTime, Datelike, Duration, Local};

pub fn start() -> Result<(), String> {
    let conn = db::establish_connection().map_err(|e| e.to_string())?;
    let unfinished_sessions = db::count_unfinished_sessions(&conn).map_err(|e| e.to_string())?;
    if unfinished_sessions > 0 {
        return Err(
            "There is an unfinished session. End it before starting a new one.".to_string(),
        );
    }

    db::create_session(&conn, Local::now()).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn end(datetime: Option<&str>) -> Result<(), String> {
    let conn = db::establish_connection().map_err(|e| e.to_string())?;
    let end_time = match datetime {
        Some(dt) => DateTime::parse_from_rfc3339(dt)
            .map_err(|_| "Invalid datetime format".to_string())?
            .with_timezone(&Local),
        None => Local::now(),
    };

    db::end_session(&conn, end_time).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn time_worked_today() -> Result<Duration, String> {
    let conn = db::establish_connection().map_err(|e| e.to_string())?;
    let today = Local::now().naive_local();
    db::get_time_worked_since(&conn, today)
}

pub fn time_worked_week() -> Result<Duration, String> {
    let conn = db::establish_connection().map_err(|e| e.to_string())?;
    let today = Local::now().naive_local();
    let weekday = today.weekday().num_days_from_monday();
    let start_of_week = today - chrono::Duration::days(weekday as i64);
    db::get_time_worked_since(&conn, start_of_week)
}
