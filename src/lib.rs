#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod apps;
pub use app::PharmacyApp;
pub use apps::AdminApp;


#[cfg(test)]
mod tests {
    use rusqlite::ffi::SQLITE_ERROR;

    use crate::apps::{db_conn::{self, insert_new_employee}, Employee};
    #[test]
    fn test_insert(){
        let _init = db_conn::has_db();
        let _insert = insert_new_employee("ryan".to_string(), "shave".to_string(), "123 easy".to_string(), "tx".to_string(), "5".to_string(), "6".to_string(), "7".to_string());
        let foo = db_conn::select_all_emp();
        print!("entering for loop\n");
        for f in foo {
            println!("the count of the item is: {}", f.len());
            for bruh in f {
                println!("{}", bruh.first_name);
            }
        }

        match foo{
            Vec<Employee>(emp) => println!("{}", emp),
            SQLITE_ERROR(bruh) => println!("{}", bruh),
        };
    }
}
