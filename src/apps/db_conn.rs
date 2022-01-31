use mysql::*;
use mysql::prelude::*;

use std::default::Default;

use mysql::conn::MyOpts;
use mysql::conn::pool::MyPool;
use mysql::value::from_value;

use super::Employee;

// #[derive(Debug, PartialEq, Eq)]
// struct Payment {
//     customer_id: i32,
//     amount: i32,
//     account_name: Option<String>,
// }

pub fn insert_new_employee() -> bool {

    let success = false;
    //let pool = mysql::Pool::new("mysql://root:a1128f69-e6f7-4e93-a2df-3d4db6030abc@localhost:3306/turretadb").unwrap();

    let opts = MyOpts {
          user: Some("root".to_string()),
          pass: Some("password".to_string()),
          ..Default::default()
    };
    let pool = MyPool::new(opts).unwrap();
    let employee_inserts = vec![
        Employee { first_name: "first".to_string(), last_name: "last".to_string() },
        Employee { first_name: "first".to_string(), last_name: "last".to_string() },
    ];

    // Let's insert payments to the database
    // We will use into_iter() because we do not need to map Stmt to anything else.
    // Also we assume that no error happened in `prepare`.
    for mut stmt in pool.prepare(r"INSERT INTO tmp.employee
                                       (customer_id, amount, account_name)
                                   VALUES
                                       (?, ?, ?)").into_iter() {
        for p in employee_inserts.iter() {
            // `execute` takes ownership of `params` so we pass account name by reference.
            // Unwrap each result just to make sure no errors happended.
            stmt.execute((1, p.first_name, p.last_name)).unwrap();
        }
    }
        success
}
// let url = "mysql://root:password@localhost:3307/db_name";

// let pool = Pool::new(url)?;

// let mut conn = pool.get_conn()?;

// Let's create a table for payments.
// conn.query_drop(
//     r"CREATE TEMPORARY TABLE payment (
//         customer_id int not null,
//         amount int not null,
//         account_name text
//     )")?;

// let payments = vec![
//     Payment { customer_id: 1, amount: 2, account_name: None },
//     Payment { customer_id: 3, amount: 4, account_name: Some("foo".into()) },
//     Payment { customer_id: 5, amount: 6, account_name: None },
//     Payment { customer_id: 7, amount: 8, account_name: None },
//     Payment { customer_id: 9, amount: 10, account_name: Some("bar".into()) },
// ];

// Now let's insert payments to the database
// conn.exec_batch(
//     r"INSERT INTO payment (customer_id, amount, account_name)
//       VALUES (:customer_id, :amount, :account_name)",
//     payments.iter().map(|p| params! {
//         "customer_id" => p.customer_id,
//         "amount" => p.amount,
//         "account_name" => &p.account_name,
//     })
// )?;

// Let's select payments from database. Type inference should do the trick here.
// let selected_payments = conn
//     .query_map(
//         "SELECT customer_id, amount, account_name from payment",
//         |(customer_id, amount, account_name)| {
//             Payment { customer_id, amount, account_name }
//         },
//     )?;

// Let's make sure, that `payments` equals to `selected_payments`.
// Mysql gives no guaranties on order of returned rows
// without `ORDER BY`, so assume we are lucky.
// assert_eq!(payments, selected_payments);
// println!("Yay!");
