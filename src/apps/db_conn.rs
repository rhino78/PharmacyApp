use mysql::*;
use mysql::prelude::*;


use super::Employee;

const CONN_STR: &str ="mysql://user1:password1@localhost:3306/testDB";

pub fn test_conn() -> bool { 
    let success = false;

    println!("attempting connection: {}", success);
    let opts = get_opts();
    let pool = Pool::new_manual(1, 1, opts).unwrap();
    let mut conn = pool.get_conn().unwrap();

    println!("connection: {}", success);
    success
}

pub fn insert_new_employee(first_name:String, last_name:String) -> std::result::Result<bool, mysql::Error> {
    let success = false;

    println!("inserting new emp: {}: {}", first_name, last_name);
    let opts = get_opts();
    //TODO: clean this up a bit
    //add some new fields to the database
    //add some error checks to the form to not allow blanks in there
    let pool = Pool::new_manual(1, 1, opts).unwrap();
    let mut conn = pool.get_conn().unwrap();
    
    let insert_str = format!("insert into employees (id, first, last) values('1', '{}','{}');", first_name, last_name);
    println!("{}", insert_str); 
     conn.query_drop(insert_str).unwrap();
    //TODO: why is this always showing false? 
    //is there a way to unpack this a bit and see more info?
    println!("the results of the query are: {}", success);
    Ok(success)
}


fn get_opts() -> Opts {

    let url = CONN_STR.to_string();
    Opts::from_url(&*url).unwrap()

}
