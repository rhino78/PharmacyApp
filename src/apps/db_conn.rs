use std::ptr::addr_of;

use mysql::prelude::*;
use mysql::*;

use super::Employee;

const CONN_STR: &str = "mysql://user1:password1@localhost:3306/testDB";

pub fn test_conn() -> bool {
    let success = false;

    let opts = get_opts();
    let pool = Pool::new_manual(1, 1, opts).unwrap();
    let connected = pool.get_conn().unwrap();
    println!("{}", connected.info_str());

    success
}

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

pub fn insert_new_employee(
    first_name: String,
    last_name: String,
    address: String,
    state: String,
    dependents: String,
    marital: String,
) -> std::result::Result<(), mysql::Error> {
    let opts = get_opts();
    let pool = Pool::new_manual(1, 1, opts).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let _marital_adj = "0";

    let insert_str = format!(
        "insert into employees (id, first, last, address, state, marital, dependents) values('1','{}','{}','{}','{}', '{}', '{}');", first_name, last_name, address, state, marital.to_string(), dependents);

    println!("{}", insert_str);

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
