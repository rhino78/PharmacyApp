use egui::{containers::*, *};

use crate::apps::db_conn;
pub struct Pay {
    pay: String,
    hours: String,
    info_label: String,
    date: String,
}

impl Default for Pay {
    fn default() -> Self {
        Self {
            pay: "".to_string(),
            hours: "".to_string(),
            info_label: "".to_string(),
            date: "".to_string(),
        }
    }
}

impl epi::App for Pay {
    fn name(&self) -> &str {
        "Payments"
    }

    fn update(&mut self, _ctx: &CtxRef, _frame: &epi::Frame) {

        let Self {
        pay: _,
        info_label: _,
        hours: _,
        date: _,
        } = self;

        egui::CentralPanel::default().show(_ctx, |ui| {
            ScrollArea::both().auto_shrink([false; 2]).show(ui, |ui| {
                ui.set_max_width(680.0);
                ui.text_edit_singleline(&mut self.pay);
                ui.separator();
                let addbtn = egui::Button::new("Add new Employee");

                self.ui(_ctx, ui);
                    if ui.add(addbtn).clicked() {
                        if let Err(e) = db_conn::insert_new_pay(
                            self.pay.to_string(),
                            self.hours.to_string(),
                            self.date.to_string(),
                        ) {
                            eprintln!("bruh: {}", e)
                        } else {
                            self.info_label = "db insert successful".to_string();
                        }
                    }
            });
        });
    }
}

impl Pay {
    pub fn ui(&mut self, _ctx: &CtxRef, _ui: &mut Ui) {
    }
}
