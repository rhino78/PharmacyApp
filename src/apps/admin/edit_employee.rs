use egui::*;

use crate::apps::db_conn;

pub struct EditEmployee {
    first_name: String,
    last_name: String,
    address: String,
    state: String,
    no_of_dependents: String,
    married: bool,
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
            pay: 0.0,
        }
    }
}

impl EditEmployee {
    pub fn ui_control(&mut self, ui: &mut egui::Ui) {

        let mut selected = 0;
        let alternatives = db_conn::get_emp_list();
        egui::ComboBox::from_label("Select one!").show_index(
            ui,
            &mut selected,
            alternatives.len(),
            |i| alternatives[i].to_owned(),
        );

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
            }
        }
    }
}


impl super::Admin for EditEmployee {
    fn name(&self) -> &'static str {
        "Edit Employee"
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

impl super::View for EditEmployee {
    fn ui(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.label("Edit Employee");
        });
        self.ui_control(ui);
    }
}