use rusqlite::{Connection, Result};

use super::schema::{create_emp, create_pay};
const CONN_STR: &str = "./my_db.db3";

pub fn drop_tables() -> Result<bool> {
    if let Err(err) = exec_drop_tables() {
        println!("could not drop tables: {}", err)
    } else {
        print!("pay table init success\n")
    }

    if let Err(err) = create_pay() {
        println!("could not init pay table: {}", err)
    } else {
        print!("pay table init success\n")
    }

    if let Err(err) = create_emp() {
        println!("could not init employee table: {}", err)
    } else {
        print!("employee table init success\n")
    }
    Ok(true)
}

fn exec_drop_tables() -> Result<bool> {
    let conn = get_db().unwrap();
    let drop_emp = "DROP TABLE employees;";
    let mut stmt = conn.prepare(drop_emp)?;
    stmt.execute([])?;

    let drop_pay = "DROP TABLE pay;";
    let mut stmt = conn.prepare(drop_pay)?;
    stmt.execute([])?;
    Ok(true)
}
pub fn has_db() -> Result<bool> {
    if let Err(err) = create_pay() {
        println!("could not init pay table: {}", err)
    } else {
        print!("pay table init success\n")
    }
    if let Err(err) = create_emp() {
        println!("could not init employee table: {}", err)
    } else {
        print!("employee table init success\n")
    }
    Ok(true)
}

pub fn get_db() -> Result<Connection> {
    Ok(Connection::open(&CONN_STR)?)
}
