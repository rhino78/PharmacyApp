use super::connect::get_db;

use rusqlite::Result;

pub fn insert_new_employee(
    first_name: String,
    last_name: String,
    address: String,
    state: String,
    dependents: String,
    marital: String,
    pay: String,
) -> Result<()> {
    let _marital_adj = "0";
    println!("inserting new employee: {}", first_name);
    let insert_str = "insert into employees (first, last, address, state, marital, dependents, pay) values(?,?,?,?,?,?,?);";
    let conn = get_db().unwrap();
    let mut stmt = conn.prepare(insert_str).unwrap();
    stmt.execute([
        first_name, last_name, address, state, dependents, marital, pay,
    ])?;
    print!("Inserting new employee complete \n");
    Ok(())
}

pub fn insert_new_pay(pay: String, hours: String, date: String) -> Result<()> {
    let insert_str = "insert into pay (pay, hours, paydate) values(?,?,?);";
    let conn = get_db().unwrap();
    let mut stmt = conn.prepare(insert_str).unwrap();
    stmt.execute([pay, hours, date])?;
    Ok(())
}
