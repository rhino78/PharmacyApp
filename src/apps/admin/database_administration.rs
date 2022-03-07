use egui::{CtxRef, Ui, Window};

use crate::apps::db_conn;

pub struct Database {
    results: String,
}

impl Default for Database {
    fn default() -> Self {
        Self {
            results: "ready".to_string(),
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
        ui.label(&self.results);
        self.ui_control(ui);

        let createdatabase = egui::Button::new("Create Database");
        if ui.add(createdatabase).clicked() {
            if let Err(e) = db_conn::create_database() {
                self.results = e.to_string();
                eprintln!("bruh: {}", e)
            }
        }

    }
}
