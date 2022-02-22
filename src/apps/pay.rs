use crate::apps::db_conn;
use chrono::{Date, TimeZone, Utc};
use egui::{containers::*, *};
use egui_datepicker::DatePicker;

use super::db_conn::select_all_pay;

pub struct Pay {
    pub pay: String,
    pub hours: String,
    pub info_label: String,
    // pub paydate: Date<Utc>,
    pub paydate: String,
    pub payrate: String,
    pub withholding: String,
    pub roth_ira: String,
}

impl Default for Pay {
    fn default() -> Self {
        Self {
            pay: "".to_string(),
            hours: "".to_string(),
            info_label: "test".to_string(),
            // paydate: Utc.ymd(2021, 1, 1),
            paydate: "".to_string(),
            payrate: "".to_string(),
            withholding: "".to_string(),
            roth_ira: "".to_string(),
        }
    }
}

impl epi::App for Pay {
    fn name(&self) -> &str {
        "Payments"
    }

    fn update(&mut self, ctx: &CtxRef, _frame: &epi::Frame) {
        let Self {
            pay: _,
            info_label: _,
            hours: _,
            paydate: _,
            payrate: _,
            withholding: _,
            roth_ira: _,
        } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
            ScrollArea::both().auto_shrink([false; 2]).show(ui, |ui| {
                ui.set_max_width(200.0);
                self.init_labels(ctx, ui);

                let addbtn = egui::Button::new("Add new Pay");
                if ui.add(addbtn).clicked() {
                    if let Err(e) = db_conn::insert_new_pay(
                        self.pay.to_string(),
                        self.hours.to_string(),
                        // self.paydate.format("%F").to_string(),
                        self.paydate.to_string(),
                    ) {
                        self.info_label = format!("db insert pay failed {}", e).to_string();
                    } else {
                        self.info_label = "success".to_string();
                    }
                }
                ui.label(self.info_label.to_string());
                ui.separator();
                self.display_grid(ctx, ui);
            });
        });
    }

    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
    }
}

impl Pay {
    // pub fn ui(&mut self, _ctx: &CtxRef, _ui: &mut Ui) {}
    fn display_grid(&mut self, _ctx: &CtxRef, _ui: &mut Ui) {
        let pay_list = select_all_pay();
        match pay_list {
            Ok(pay_list) => {
                let bruh = pay_list.unwrap();
                egui::Grid::new("pay_list").striped(true).show(_ui, |ui| {
                    ui.label("pay");
                    ui.label("hours");
                    ui.label("paydate");
                    ui.end_row();

                    for b in bruh.iter() {
                        ui.label(b.pay.to_string());
                        ui.label(b.hours.to_string());
                        ui.label(b.paydate.to_string());
                        ui.end_row();
                    }
                });
            }
            Err(e) => eprintln!("{}", e),
        };
    }
    fn init_labels(&mut self, _ctx: &CtxRef, ui: &mut Ui) {
        ui.label("pay");
        ui.text_edit_singleline(&mut self.pay);
        ui.label("hours");
        ui.text_edit_singleline(&mut self.hours);
        ui.label("paydate");
        // ui.add(DatePicker::new("datepicker-unique-id", &mut self.paydate));
        ui.text_edit_singleline(&mut self.paydate);
        ui.label("payrate");
        ui.label(self.payrate.to_string());
        ui.label("withholding");
        ui.label(self.withholding.to_string());
        ui.label("roth");
        ui.label(self.roth_ira.to_string());
        ui.separator();
    }
}
