use super::connect::get_db;
use crate::apps::{Employee, Pay};
use rusqlite::named_params;
use rusqlite::params;

use rusqlite::Result;

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

pub fn select_all_pay2() -> Result<Vec<String>> {
    let insert_str = "select pay, hours, paydate from pay;";
    let conn = get_db().unwrap();
    let mut stmt = conn.prepare(insert_str).unwrap();
    let mut rows = stmt.query([])?;
    let mut bruh = Vec::new();
    while let Some(row) = rows.next()? {
        bruh.push(row.get(0)?);
        bruh.push(row.get(1)?);
    }
    Ok(bruh)
}

pub fn select_all_pay() -> Result<Vec<Pay>> {
    let insert_str = "select pay, hours, paydate from pay;";
    let conn = get_db().unwrap();
    let mut stmt = conn.prepare(insert_str).unwrap();
    let selected_pay = stmt
        .query_map(params![], |row| {
            Ok(Pay {
                pay: 0.0,
                hours: 0,
                info_label: "".to_string(),
                paydate: "".to_string(),
                payrate: "".to_string(),
                withholding: "".to_string(),
                roth_ira: "".to_string(),
            })
        })
        .unwrap();

    //how to handle the row.get_unwrap for integers?
    selected_pay.collect()
}
