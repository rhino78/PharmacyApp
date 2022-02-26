use chrono::Date;
use chrono::Utc;
use mysql::prelude::*;
use mysql::*;

use super::Employee;
use super::Pay;
// use chrono::{TimeZone, Utc};

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
            println!("error clearing records: {}", e);
        }
        Err(e) => {
            eprintln!("got a general error: {}", e);
        }
    })
}

pub fn select_all_pay(
) -> std::result::Result<std::result::Result<std::vec::Vec<Pay>, mysql::Error>, mysql::Error> {
    let opts = get_opts();
    let pool = Pool::new_manual(1, 1, opts).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let selected_pay = conn.query_map(
        "select pay, hours, paydate from pay;",
        |(pay, hours, paydate)| -> Pay {
            Pay {
                pay,
                hours,
                paydate,
                info_label: "".to_string(),
                payrate: "".to_string(),
                withholding: "".to_string(),
                roth_ira: "".to_string(),
            }
        },
    );

    Ok(selected_pay)
}

pub fn select_all_emp(
) -> std::result::Result<std::result::Result<std::vec::Vec<Employee>, mysql::Error>, mysql::Error> {
    let opts = get_opts();
    let pool = Pool::new_manual(1, 1, opts).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let selected_emp = conn.query_map(
        "select id, first, last from employees;",
        |(id, first_name, last_name)| Employee {
            id,
            first_name,
            last_name,
        },
    );

    Ok(selected_emp)
}

// CREATE TABLE `testDB`.`pay` (
//   `id` INT NOT NULL AUTO_INCREMENT,
//   `pay` DECIMAL NOT NULL,
//   `hours` DECIMAL NOT NULL,
//   `paydate` DATE NOT NULL,
//   PRIMARY KEY (`id`));

pub fn insert_new_pay(
    pay: String,
    hours: String,
    date: String,
) -> std::result::Result<(), mysql::Error> {
    let opts = get_opts();
    let pool = Pool::new_manual(1, 1, opts).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let insert_str = format!(
        "insert into pay (pay, hours, paydate) values('{}','{}','{}');",
        pay, hours, date
    );

    Ok(match conn.query_drop(&insert_str) {
        Ok(_) => {}
        Err(mysql::Error::IoError(e)) => {
            eprintln!("{}", e);
        }
        Err(mysql::Error::MySqlError(e)) => {
            println!("error inserting pay: {}", e);
            println!("{}", insert_str);
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

pub fn get_emp_list() -> std::vec::Vec<std::string::String> {
    let opts = get_opts();
    let pool = Pool::new_manual(1, 1, opts).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let selected_emp = conn.query_map(
        "select id, first, last from employees;",
        |(id, first_name, last_name)| Employee {
            id,
            first_name,
            last_name,
        },
    );

    let bruh = selected_emp.unwrap();

    let mut results = vec![];
    for s in bruh {
        results.push(s.first_name.to_string());
    }
    results
}
