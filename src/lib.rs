#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod apps;
pub use app::PharmacyApp;
pub use apps::AdminApp;

#[cfg(test)]
mod tests {
    use crate::apps::database::{
        connect::has_db,
        insert::{insert_new_employee, insert_new_pay},
        select::select_all_emp,
        select::select_all_pay2,
        update::update_employee,
    };

    #[test]
    fn test_select() {
        print!("begining select test\n");
        let _init = has_db();
        let select = select_all_pay2();
        for s in select {
            println!("{:?}", s);
        }

        print!("select complete\n");
    }

    #[test]
    fn test_update() {
        print!("begining udpate test\n");
        let _init = has_db();
        let update = update_employee(
            "1".to_string(),
            "ryan".to_string(),
            "shave".to_string(),
            "123 easy".to_string(),
            "Tx".to_string(),
            "1".to_string(),
            "1".to_string(),
            "2.3".to_string(),
        );

        match update {
            Ok(res) => println!("insert success: {:?}", res),
            Err(err) => println!("we made a boo-bool: {}", err),
        }
    }
    #[test]
    fn test_pay_insert() {
        print!("begining insert test\n");
        let _init = has_db();
        let insert = insert_new_pay(
            "pay".to_string(),
            "hours".to_string(),
            "paydate".to_string(),
        );
        match insert {
            Ok(res) => println!("insert success: {:?}", res),
            Err(err) => println!("we made a boo-bool: {}", err),
        }
    }

    #[test]
    fn test_insert() {
        print!("begining insert test\n");
        let _init = has_db();
        let _insert = insert_new_employee(
            "ryan".to_string(),
            "shave".to_string(),
            "123 easy".to_string(),
            "tx".to_string(),
            "5".to_string(),
            "6".to_string(),
            "7".to_string(),
        );
        let foo = select_all_emp();

        match foo {
            Ok(farts) => {
                for f in farts {
                    println!("first name: {}", f.first_name)
                }
            }
            Err(err) => println!("{}", err),
        }
    }
}
