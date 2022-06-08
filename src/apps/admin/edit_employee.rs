use egui::*;

use crate::apps::db_conn;

pub struct EditEmployee {
    id: String,
    first_name: String,
    last_name: String,
    address: String,
    state: String,
    no_of_dependents: String,
    married: String,
    pay: String,
    selected_index: usize,
}

#[derive(Debug)]
pub struct Employee {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub address: String,
    pub state: String,
    pub married: String,
    pub dependents: String,
    pub pay: String,
}

impl Default for EditEmployee {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            first_name: "".to_string(),
            last_name: "".to_string(),
            address: "".to_string(),
            state: "".to_string(),
            no_of_dependents: "".to_string(),
            married: "".to_string(),
            pay: "".to_string(),
            selected_index: 0,
        }
    }
}

impl EditEmployee {
    pub fn ui_control(&mut self, ui: &mut egui::Ui) {
        let emp_list = db_conn::select_all_emp().unwrap();

        egui::ComboBox::from_label("Select Employee").show_index(
            ui,
            &mut self.selected_index,
            emp_list.len(),
            |i| emp_list[i].first_name.to_string(),
        );

        ui.label("Selected Id");
        ui.text_edit_singleline(&mut self.selected_index.to_string());
        ui.label("first name");
        ui.text_edit_singleline(&mut self.first_name.to_string());
        ui.separator();
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
        self.ui_control(ui);

        let edit_employee = egui::Button::new("Edit Employee in database");
        if ui.add(edit_employee).clicked() {
            if let Err(e) = db_conn::update_employee(
                self.id.to_string(),
                self.first_name.to_string(),
                self.last_name.to_string(),
                self.address.to_string(),
                self.state.to_string(),
                self.no_of_dependents.to_string(),
                self.married.to_string(),
                self.pay.to_string(),
            ) {
                eprintln!("could not edit employee: {}", e)
            }
        }
    }
}
