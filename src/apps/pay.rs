use egui::{containers::*, *};

use super::database::{delete::clear_records, select::select_all_pay};

pub struct Pay {
    pub pay: f32,
    pub hours: i32,
    pub info_label: String,
    // pub paydate: Option<Date<Utc>>,
    // pub paydate: Date<Utc>,
    pub paydate: String,
    pub payrate: String,
    pub withholding: String,
    pub roth_ira: String,
}

impl Default for Pay {
    fn default() -> Self {
        Self {
            pay: 0.0,
            hours: 0,
            info_label: "test".to_string(),
            // paydate: Some(Utc.ymd(2021, 1, 1)),
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

                let clearbtn = egui::Button::new("Clear All records");
                if ui.add(clearbtn).clicked() {
                    if let Err(e) = clear_records() {
                        self.info_label = e.to_string();
                        eprintln!("could not clear all records: {}", e)
                    } else {
                        self.info_label = "Employee List Cleared".to_string();
                    }
                }
                ui.label(&self.info_label);
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
                egui::Grid::new("pay_list").striped(true).show(_ui, |ui| {
                    ui.label("pay");
                    ui.label("hours");
                    ui.label("paydate");
                    ui.end_row();

                    for b in pay_list.iter() {
                        ui.label(b.pay.to_string());
                        ui.label(b.hours.to_string());
                        ui.label(b.paydate.to_string());
                        ui.end_row();
                    }
                });
            }
            Err(e) => eprintln!("could not fetch pay data: {}", e),
        };
    }
}
