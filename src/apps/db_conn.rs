use mysql::prelude::*;
use mysql::*;

use super::Employee;

const CONN_STR: &str = "mysql://user1:password1@localhost:3306/testDB";

pub fn clear_records() -> std::result::Result<(), mysql::Error> {
    let opts = get_opts();
    let pool = Pool::new_manual(1, 1, opts).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let insert_str = "set sql_safe_updates = 0; DELETE FROM testDB.employees WHERE id =1; set sql_safe_updates = 1;";
    Ok(match conn.query_drop(&insert_str) {
        Ok(_) => {}
        Err(mysql::Error::IoError(e)) => {
            eprintln!("{}", e);
        }
        Err(mysql::Error::MySqlError(e)) => {
            print!("got a mysql error");
            println!("{}", e);
        }
        Err(e) => {
            print!("got a general error");
            eprintln!("{}", e);
        }
    })
}

pub fn select_all_emp() -> std::result::Result<std::result::Result<std::vec::Vec<Employee>, mysql::Error>, mysql::Error> {
    let opts = get_opts();
    let pool = Pool::new_manual(1, 1, opts).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let selected_emp = conn.query_map("select id, first, last from employees;", |(id, first_name, last_name)| {
        Employee { id, first_name, last_name }
        },
        );

    Ok(selected_emp)
}

pub fn insert_new_pay(pay: String, hours: String, date: String) -> std::result::Result<(), mysql::Error> {
    let opts = get_opts();
    let pool = Pool::new_manual(1, 1, opts).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let insert_str = format!(
        "insert into pay (id, pay, hours, date) values('1','{}','{}','{}');", pay, hours, date);

    Ok(match conn.query_drop(&insert_str) {
        Ok(_) => {}
        Err(mysql::Error::IoError(e)) => {
            eprintln!("{}", e);
        }
        Err(mysql::Error::MySqlError(e)) => {
            print!("got a mysql error");
            println!("{}", e);
        }
        Err(e) => {
            print!("got a general error");
            eprintln!("{}", e);
        }
    })

}

pub fn insert_new_employee(
    first_name: String,
    last_name: String,
    address: String,
    state: String,
    dependents: String,
    marital: String,
    _pay: String,
) -> std::result::Result<(), mysql::Error> {
    let opts = get_opts();
    let pool = Pool::new_manual(1, 1, opts).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let _marital_adj = "0";

    let insert_str = format!(
        "insert into employees (id, first, last, address, state, marital, dependents) values('1','{}','{}','{}','{}', '{}', '{}');", first_name, last_name, address, state, marital.to_string(), dependents);

    Ok(match conn.query_drop(&insert_str) {
        Ok(_) => {}
        Err(mysql::Error::IoError(e)) => {
            eprintln!("{}", e);
        }
        Err(mysql::Error::MySqlError(e)) => {
            print!("got a mysql error");
            println!("{}", e);
        }
        Err(e) => {
            print!("got a general error");
            eprintln!("{}", e);
        }
    })
}

fn get_opts() -> Opts {
    let url = CONN_STR.to_string();
    Opts::from_url(&*url).unwrap()
}
