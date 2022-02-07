use rusqlite::{params, Connection, Result};
const CONN_STR: &str = "employee.db";

pub fn test_conn() -> Result<()> {
    let conn = Connection::open(CONN_STR)?;

    conn.execute(
        "create table if not exists cat_colors (
             id integer primary key,
             name text not null unique
         )",
        params![],
    )?;
    conn.execute(
        "create table if not exists cats (
             id integer primary key,
             name text not null,
             color_id integer not null references cat_colors(id)
         )",
        params![],
    )?;

    Ok(())
}

pub fn insert_new_employee(first_name: String, last_name: String) -> Result<()> {
    let conn = Connection::open(CONN_STR);

    println!("inserting new emp: {}: {}", first_name, last_name);
    //TODO: clean this up a bit
    //add some new fields to the database
    //add some error checks to the form to not allow blanks in there
    //TODO: why is this always showing false?
    //is there a way to unpack this a bit and see more info?
    // conn.execute("insert into employees(id, first, last) values(?1, ?2)", params![first_name, last_name])?;
    Ok(())
}
