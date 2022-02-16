use crate::apps::db_conn;
use egui::{containers::*, *};

pub struct EditEmployee {
    first_name: String,
    last_name: String,
    address: String,
    state: String,
    no_of_dependents: String,
    married: bool,
    info_label: String,
    pay: f64,
}

impl Default for EditEmployee {
    fn default() -> Self {
        Self {
            first_name: "".to_string(),
            last_name: "".to_string(),
            address: "".to_string(),
            state: "".to_string(),
            no_of_dependents: "".to_string(),
            married: false,
            info_label: "".to_string(),
            pay: 0.0,
        }
    }
}

impl epi::App for EditEmployee {
    fn name(&self) -> &str {
        "Add/Edit Employee"
    }

    fn update(&mut self, ctx: &CtxRef, _frame: &epi::Frame) {
        let Self {
            first_name: _,
            last_name: _,
            address: _,
            state: _,
            no_of_dependents: _,
            married: _,
            info_label: _,
            pay: _,
        } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
            ScrollArea::both().auto_shrink([false; 2]).show(ui, |ui| {
                self.ui(ctx, ui);
                ui.set_max_width(600.0);
                ui.vertical(|ui| {
                    ui.label("First Name");
                    ui.text_edit_singleline(&mut self.first_name)
                        .on_hover_text("type the first name here");
                    ui.label("Last Name");
                    ui.text_edit_singleline(&mut self.last_name)
                        .on_hover_text("type the last name here");
                    ui.label("Address");
                    ui.text_edit_singleline(&mut self.address)
                        .on_hover_text("type the address here");
                    ui.label("State");
                    ui.text_edit_singleline(&mut self.state)
                        .on_hover_text("type the state here");
                    ui.label("Number of Dependents");
                    ui.text_edit_singleline(&mut self.no_of_dependents)
                        .on_hover_text("type the state here");
                    ui.label("Married?");
                    ui.checkbox(&mut self.married, "Married");
                    ui.label("Pay");
                    ui.text_edit_singleline(&mut self.pay.to_string())
                        .on_hover_text("type the pay here");
                    let addbtn = egui::Button::new("Add new Employee");

                    if ui.add(addbtn).clicked() {
                        if let Err(e) = db_conn::insert_new_employee(
                            self.first_name.to_string(),
                            self.last_name.to_string(),
                            self.address.to_string(),
                            self.state.to_string(),
                            self.no_of_dependents.to_string(),
                            self.married.to_string(),
                            self.pay.to_string(),
                        ) {
                            eprintln!("bruh: {}", e)
                        } else {
                            self.info_label = "db insert successful".to_string();
                        }
                    }

                    let clearbtn = egui::Button::new("Clear Employees");
                    if ui.add(clearbtn).clicked() {
                        if let Err(e) = db_conn::clear_records() {
                            self.info_label = format!("clear db fail: {}", e);
                        } else {
                            self.info_label = "clear db success".to_string();
                        }
                    }

                    ui.label(self.info_label.to_string());
                });
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

    fn warm_up_enabled(&self) -> bool {
        false
    }

    fn save(&mut self, _storage: &mut dyn epi::Storage) {}

    fn on_exit(&mut self) {}

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn max_size_points(&self) -> egui::Vec2 {
        // Some browsers get slow with huge WebGL canvases, so we limit the size:
        egui::Vec2::new(1024.0, 2048.0)
    }

    fn clear_color(&self) -> egui::Rgba {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).into()
    }

    fn persist_native_window(&self) -> bool {
        true
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }
}

impl EditEmployee {
    pub fn ui(&mut self, _ctx: &CtxRef, _ui: &mut Ui) {}
}
