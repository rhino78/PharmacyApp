use mysql::*;
use mysql::prelude::*;


use super::Employee;

const CONN_STR: &str ="mysql://user1:password1@localhost:3306/testDB";

pub fn test_conn() -> bool { 
    let success = false;

    let opts = get_opts();
    let pool = Pool::new_manual(1, 1, opts).unwrap();
    let connected = pool.get_conn().unwrap();
    println!("{}", connected.info_str());

    println!("connection: {}", success);
    success
}

pub fn insert_new_employee(first_name:String, last_name:String) -> std::result::Result<(), mysql::Error> {
    let opts = get_opts();
    //TODO: clean this up a bit
    //add some new fields to the database
    //add some error checks to the form to not allow blanks in there
    let pool = Pool::new_manual(1, 1, opts).unwrap();
    let mut conn = pool.get_conn().unwrap();
    
    let insert_str = format!("insert into employees (id, first, last) values('1', '{}','{}');", first_name, last_name);
     //TODO: why is this always showing false? 
    //is there a way to unpack this a bit and see more info?
    Ok(match conn.query_drop(insert_str) {
        Ok(_) => {
        },
            Err(mysql::Error::IoError(e)) => {
            eprintln!("{}", e);
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    })
}


fn get_opts() -> Opts {
    let url = CONN_STR.to_string();
    Opts::from_url(&*url).unwrap()
}
