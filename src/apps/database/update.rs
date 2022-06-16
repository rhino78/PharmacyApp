use super::connect::get_db;
use rusqlite::Result;

///a method to update the employee record
pub fn update_employee(
    id: String,
    first_name: String,
    last_name: String,
    address: String,
    state: String,
    dependents: String,
    marital: String,
    pay: String,
) -> Result<()> {
    let _marital_adj = "0";
    let update_str = "update employees set first = ?, last = ?, address = ?, state = ?, marital = ?, dependents = ?, pay = ? where id = ?;";
    let conn = get_db().unwrap();
    let mut stmt = conn.prepare(update_str).unwrap();
    stmt.execute([
        id, first_name, last_name, address, state, dependents, marital, pay,
    ])?;
    Ok(())
}
