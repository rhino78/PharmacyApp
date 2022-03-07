use egui::*;

use crate::apps::db_conn;

pub struct EditEmployee {
    id: String,
    first_name: String,
    last_name: String,
    address: String,
    state: String,
    no_of_dependents: String,
    married: bool,
    pay: f64,
    selected_index: usize,
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
            married: false,
            pay: 0.0,
            selected_index: 0,
        }
    }
}

impl EditEmployee {
    // pub fn update() {}

    pub fn ui_control(&mut self, ui: &mut egui::Ui) {
        let emp_list = db_conn::get_emp_obj();
        let bruh = emp_list.unwrap();

        egui::ComboBox::from_label("Select Employee").show_index(
            ui,
            &mut self.selected_index,
            bruh.len(),
            |i| bruh[i].first_name.to_string(),
        );

        let mut fname = bruh[self.selected_index].first_name.to_string();
        let mut lname = bruh[self.selected_index].last_name.to_string();
        let mut address = bruh[self.selected_index].address.to_string();
        let mut state = bruh[self.selected_index].state.to_string();
        let mut married = bruh[self.selected_index].married.to_string();
        let mut dependents = bruh[self.selected_index].dependents.to_string();
        let mut id = bruh[self.selected_index].id.to_string();
        let mut pay = bruh[self.selected_index].pay.to_string();

        ui.label("Selected Id");
        ui.text_edit_singleline(&mut id);
        ui.label("Selected First Name");
        ui.text_edit_singleline(&mut fname);
        ui.label("Selected Last Name");
        ui.text_edit_singleline(&mut lname);
        ui.label("Selected address");
        ui.text_edit_singleline(&mut address);
        ui.label("Selected state");
        ui.text_edit_singleline(&mut state);
        ui.label("Selected dependents");
        ui.text_edit_singleline(&mut dependents);
        ui.label("Selected married");
        ui.text_edit_singleline(&mut married);
        ui.label("Selected pay");
        ui.text_edit_singleline(&mut pay);
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
        ui.separator();
        self.ui_control(ui);

        //TODO: add update fn to db_conn
        let addbtn = egui::Button::new("Edit Employee");
        if ui.add(addbtn).clicked() {
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
                eprintln!("bruh: {}", e)
            }
        }
    }
}
