use rusqlite::params;
use rusqlite::{Connection, Result};

use super::Employee;
use super::Pay;

const CONN_STR: &str = "./my_db.db3";

pub fn get_db() -> Result<Connection> {
    Ok(Connection::open(&CONN_STR)?)
}

pub fn clear_records() -> Result<()> {
    let conn = get_db().unwrap();
    let insert_str = "set sql_safe_updates = 0; DELETE FROM testDB.employees WHERE id =1; set sql_safe_updates = 1;";
    let mut stmt = conn.prepare(insert_str)?;
    stmt.execute([])?;
    Ok(())
}

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

pub fn select_all_pay() -> Result<Vec<Pay>> {
    let insert_str = "select pay, hours, paydate from pay;";
    let conn = get_db().unwrap();
    let mut stmt = conn.prepare(insert_str).unwrap();
    let selected_pay = stmt
        .query_map(params![], |row| {
            Ok(Pay {
                pay: row.get_unwrap(0),
                hours: row.get_unwrap(1),
                info_label: row.get_unwrap(2),
                paydate: "".to_string(),
                payrate: "".to_string(),
                withholding: "".to_string(),
                roth_ira: "".to_string(),
            })
        })
        .unwrap();

    selected_pay.collect()
}

pub fn select_all_emp() -> Result<Vec<Employee>> {
    let select_string =
        "select id, first, last, address, state, marital, dependents, pay from employees;";
    let conn = get_db().unwrap();
    let mut stmt = conn.prepare(select_string).unwrap();

    let selected_emp = stmt
        .query_map(params![], |row| {
            Ok(Employee {
                id: "".to_string(),
                first_name: row.get_unwrap(1),
                last_name: row.get_unwrap(2),
                address: row.get_unwrap(3),
                state: row.get_unwrap(4),
                married: row.get_unwrap(5),
                dependents: row.get_unwrap(6),
                pay: row.get_unwrap(7),
            })
        })
        .unwrap();

    selected_emp.collect()
}

pub fn insert_new_pay(pay: String, hours: String, date: String) -> Result<()> {
    let insert_str = "insert into pay (pay, hours, paydate) values(?,?,?);";
    let conn = get_db().unwrap();
    let mut stmt = conn.prepare(insert_str).unwrap();
    stmt.execute([pay, hours, date])?;
    Ok(())
}

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

pub fn get_emp_obj() -> Result<Vec<Employee>> {
    let select_string =
        "select id, first, last, address, state, marital, dependents, pay from employees;";
    let conn = get_db().unwrap();
    let mut stmt = conn.prepare(select_string).unwrap();

    let selected_emp = stmt
        .query_map(params![], |row| {
            Ok(Employee {
                id: row.get_unwrap(0),
                first_name: row.get_unwrap(1),
                last_name: row.get_unwrap(2),
                address: row.get_unwrap(3),
                state: row.get_unwrap(4),
                married: row.get_unwrap(5),
                dependents: row.get_unwrap(6),
                pay: row.get_unwrap(7),
            })
        })
        .unwrap();
    selected_emp.collect()
}

pub(crate) fn has_db() -> Result<bool> {
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
