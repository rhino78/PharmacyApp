use egui::{CtxRef, Ui, Window};
use sysinfo::{ProcessExt, System, SystemExt};

use crate::apps::db_conn;

pub struct Database {
    service: String,
    is_initialized: bool,
}

impl Default for Database {
    fn default() -> Self {
        Self {
            service: "Not Ready".to_string(),
            is_initialized: false,
        }
    }
}

impl Database {
    pub fn ui_control(&mut self, _ui: &mut egui::Ui) {}
}

impl super::Admin for Database {
    fn name(&self) -> &'static str {
        "Database Administration"
    }

    fn show(&mut self, ctx: &CtxRef, open: &mut bool) {
        use super::View as _;
        Window::new(self.name())
            .open(open)
            .vscroll(false)
            .resizable(false)
            .default_size([300.0, 350.0])
            .show(ctx, |ui| self.ui(ui));
    }
}

impl super::View for Database {
    fn ui(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.label("Database");
        });
        ui.separator();
        if self.service == "Not Ready" {
            let s = System::new_all();
            for process in s.processes_by_exact_name("mariadbd") {
                println!("{}", process.name());
                self.service = "Ready".to_string();
            }
        }
        if !self.is_initialized {
            let foo = db_conn::has_db();
            match foo {
                Ok(_f) => self.is_initialized = true,
                Err(e) => println!("error getting databse: {:?}", e),
            }
        }

        ui.label(format!("the service is: {}", &self.service));
        ui.label(format!(
            "the database is: {}",
            &self.is_initialized.to_string()
        ));
        self.ui_control(ui);

        let createdatabase = egui::Button::new("Create Database");
        if ui.add(createdatabase).clicked() {
            // if let Err(e) = db_conn::create_database() {
            //     self.service = e.to_string();
            //     eprintln!("bruh: {}", e)
            // }
        }
    }
}
