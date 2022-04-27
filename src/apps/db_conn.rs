use mysql::prelude::*;
use mysql::*;

use super::Employee;
use super::Pay;

#[derive(Debug)]
pub struct Database {
    database: String,
}
const CONN_STR: &str = "mysql://user1:password1@localhost:3306/testDB";

pub fn has_db(
) -> std::result::Result<std::result::Result<std::vec::Vec<Database>, mysql::Error>, mysql::Error> {
    let opts = get_opts();
    let pool = Pool::new_manual(1, 1, opts).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let select_db = conn.query_map("SHOW Databases like 'testDB%';", |database| Database {
        database,
    });
    Ok(select_db)
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
            println!("error clearing records: {}", e);
        }
        Err(e) => {
            eprintln!("got a general error: {}", e);
        }
    })
}

pub fn create_database() {
    // let emp = create_emp();
    // let pay = create_pay();
    print!("creating database");
}

pub fn create_emp() -> std::result::Result<(), mysql::Error> {
    let opts = get_opts();
    let pool = Pool::new_manual(1, 1, opts).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let insert_str = "CREATE TABLE 'testDB'.'employees' ('id' int(11 NOT NULL, 
'first' varchar(45) DEFAULT NULL, 
'last' varchar(45) DEFAULT NULL, 
'address' varchar(45) DEFAULT NULL, 
'state' varchar(45) DEFAULT NULL, 
'marital' varchar(45) DEFAULT NULL, 
'dependents' varchar(45) DEFAULT NULL, 
'pay' varchar(45) DEFAULT NULL, 
));";

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

pub fn create_pay() -> std::result::Result<(), mysql::Error> {
    let opts = get_opts();
    let pool = Pool::new_manual(1, 1, opts).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let insert_str = "CREATE TABLE `testDB`.`pay` (`id` INT NOT NULL AUTO_INCREMENT, `pay` DECIMAL NOT NULL, `hours` DECIMAL NOT NULL, `paydate` DATE NOT NULL, PRIMARY KEY (`id`));";

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
        "select id, first, last, address, state, marital, dependents, pay from employees;",
        |(id, first_name, last_name, address, state, married, dependents, pay)| Employee {
            id,
            first_name,
            last_name,
            address,
            state,
            married,
            dependents,
            pay,
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
) -> std::result::Result<(), mysql::Error> {
    let opts = get_opts();
    let pool = Pool::new_manual(1, 1, opts).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let _marital_adj = "0";

    let update_str = format!(
    "update employee set first = '{}', last = '{}', address = '{}', state = '{}', marital = '{}', dependents = '{}', pay = '{}' where id = '{}';", first_name, last_name, address, state, marital.to_string(), dependents, pay, id);

    Ok(match conn.query_drop(&update_str) {
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
    pay: String,
) -> std::result::Result<(), mysql::Error> {
    let opts = get_opts();
    let pool = Pool::new_manual(1, 1, opts).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let _marital_adj = "0";

    let insert_str = format!(
        "insert into employees (id, first, last, address, state, marital, dependents, pay) values('1','{}','{}','{}','{}', '{}', '{}', '{}');", first_name, last_name, address, state, marital.to_string(), dependents, pay);

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

pub fn get_emp_obj() -> Result<Vec<Employee>> {
    let opts = get_opts();
    let pool = Pool::new_manual(1, 1, opts).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let selected_emp = conn.query_map(
        "select id, first, last, address, state, marital, dependents, pay from employees;",
        |(id, first_name, last_name, address, state, married, dependents, pay)| Employee {
            id,
            first_name,
            last_name,
            address,
            state,
            married,
            dependents,
            pay,
        },
    );

    selected_emp
}

fn get_opts() -> Opts {
    let url = CONN_STR.to_string();
    Opts::from_url(&*url).unwrap()
}

