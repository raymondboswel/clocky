use chrono::{DateTime, Duration, Local, NaiveDate};
use rusqlite::{Connection, Result};

pub fn establish_connection() -> Result<Connection> {
    Connection::open("clocky.db")
}

pub fn create_session(conn: &Connection, start_time: DateTime<Local>) -> Result<()> {
    conn.execute(
        "INSERT INTO sessions (start_time) VALUES (?1)",
        &[&start_time.to_rfc3339()],
    )?;
    Ok(())
}

pub fn end_session(conn: &Connection, end_time: DateTime<Local>) -> Result<()> {
    conn.execute(
        "UPDATE sessions SET end_time = ?1 WHERE end_time IS NULL",
        &[&end_time.to_rfc3339()],
    )?;
    Ok(())
}

pub fn count_unfinished_sessions(conn: &Connection) -> Result<usize> {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM sessions WHERE end_time IS NULL")?;
    let count: usize = stmt.query_row([], |row| row.get(0))?;
    Ok(count)
}

pub fn get_time_worked_since(conn: &Connection, since: NaiveDate) -> Result<Duration, String> {
    let mut stmt = conn
        .prepare("SELECT start_time, end_time FROM sessions WHERE start_time >= ?1")
        .map_err(|e| e.to_string())?;

    let mut rows = stmt
        .query([&since.to_string()])
        .map_err(|e| e.to_string())?;
    let mut total_duration = Duration::zero();

    while let Some(row) = rows.next().map_err(|e| e.to_string())? {
        let start_time: String = row.get(0).map_err(|e| e.to_string())?;
        let end_time: Option<String> = row.get(1).map_err(|e| e.to_string())?;

        let start_time = DateTime::parse_from_rfc3339(&start_time).map_err(|e| e.to_string())?;
        let end_time = match end_time {
            Some(et) => DateTime::parse_from_rfc3339(&et).map_err(|e| e.to_string())?,
            None => Local::now().fixed_offset(),
        };

        total_duration = total_duration + (end_time - start_time);
    }

    Ok(total_duration)
}
