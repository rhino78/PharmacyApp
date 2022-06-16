use super::connect::get_db;
use rusqlite::Result;

pub fn create_emp() -> Result<()> {
    let insert_str = "CREATE TABLE IF NOT EXISTS employees (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        first TEXT NULL,
        last TEXT NULL,
        address TEXT NULL,
        state TEXT NULL,
        marital TEXT NULL,
        dependents TEXT NULL,
        pay TEXT NULL);";

    let conn = get_db().unwrap();
    let mut stmt = conn.prepare(insert_str)?;
    stmt.execute([])?;
    Ok(())
}

pub fn create_pay() -> Result<()> {
    let insert_str = "CREATE TABLE IF NOT EXISTS pay (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        pay DECIMAL NOT NULL, 
        hours DECIMAL NOT NULL, 
        paydate DATE NOT NULL);";
    let conn = get_db().unwrap();
    let mut stmt = conn.prepare(insert_str)?;
    stmt.execute([])?;
    Ok(())
}
