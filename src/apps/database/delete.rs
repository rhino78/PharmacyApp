use super::connect::get_db;
use rusqlite::Result;

pub fn clear_records() -> Result<()> {
    let conn = get_db().unwrap();
    let insert_str = "set sql_safe_updates = 0; DELETE FROM testDB.employees WHERE id =1; set sql_safe_updates = 1;";
    let mut stmt = conn.prepare(insert_str)?;
    stmt.execute([])?;
    Ok(())
}
