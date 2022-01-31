use egui::{containers::*, *};


pub struct EditEmployee {
    first_name: String,
    last_name: String,
    address: String,
    state: String,
}

impl Default for EditEmployee {
    fn default() -> Self {
        Self {
            first_name: "Pooo".to_string(),
            last_name: "mcGee".to_string(),
            address: "".to_string(),
            state: "".to_string(),
        }
    }
}

impl epi::App for EditEmployee {
    fn name(&self) -> &str {
        "Add/Edit Employee"
    }

    fn update(&mut self, ctx: &CtxRef, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ScrollArea::both().auto_shrink([false; 2]).show(ui, |ui| {
                self.ui(ctx, ui);
            });
        });
    }
}

impl EditEmployee {
    pub fn ui(&mut self, _ctx: &CtxRef, ui: &mut Ui) {

        ui.set_max_width(600.0);
        ui.vertical(|ui| {

            ui.label("First Name");
            ui.text_edit_singleline(&mut self.first_name).on_hover_text("poop");
            ui.label("Last Name");
            ui.text_edit_singleline(&mut self.last_name);

        });
        ui.vertical(|ui| {
            ui.label("Address");
            ui.text_edit_singleline(&mut self.address);
            ui.label("State");
            ui.text_edit_singleline(&mut self.state);

        });
        ui.vertical(|ui| {

            let button1 = egui::Button::new("Add new Employee");
            if ui.add(button1).clicked(){
                    println!("{},{},{},{}", self.first_name, self.last_name, self.address, self.state);
            }
        });
    }
}
