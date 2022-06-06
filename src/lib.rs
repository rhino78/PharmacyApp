#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod apps;
pub use app::PharmacyApp;
pub use apps::AdminApp;


#[cfg(test)]
mod tests {
    use crate::apps::db_conn::{self, insert_new_employee};
    #[test]
    fn test_insert(){
        let _init = db_conn::has_db();
        let _insert = insert_new_employee("ryan".to_string(), "shave".to_string(), "123 easy".to_string(), "tx".to_string(), "5".to_string(), "6".to_string(), "7".to_string());
        let foo = db_conn::select_all_emp();

        match foo {
            Ok(farts) => for f in farts{
                println!("first name: {}", f.first_name)
            },
            Err(err) => println!("{}", err ),
        }




    }
}
