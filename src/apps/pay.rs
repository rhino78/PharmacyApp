use egui::{containers::*, *};

use crate::apps::db_conn;
pub struct Pay {
    name: String,
    name_checked: bool,
}

impl Default for Pay {
    fn default() -> Self {
        Self {
            name: "pooo".to_string(),
            name_checked: false,
        }
    }
}

impl epi::App for Pay {
    fn name(&self) -> &str {
        "Payments"
    }

    fn update(&mut self, _ctx: &CtxRef, _frame: &epi::Frame) {
        println!("connection test:{}", db_conn::test_conn());
        egui::CentralPanel::default().show(_ctx, |ui| {
            ScrollArea::both().auto_shrink([false; 2]).show(ui, |ui| {
                self.ui(_ctx, ui);
            });
        });
    }
}

impl Pay {
    pub fn ui(&mut self, _ctx: &CtxRef, ui: &mut Ui) {
        ui.set_max_width(680.0);
        ui.label(&self.name);
        ui.checkbox(&mut self.name_checked, "the name");
    }
}
