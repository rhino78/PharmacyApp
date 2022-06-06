use egui::*;

use crate::apps::db_conn;

pub struct InsertEmployee {
    first_name: String,
    last_name: String,
    address: String,
    state: String,
    state_selected: usize,
    no_of_dependents: String,
    married: bool,
    pay: f32,
}

impl Default for InsertEmployee {
    fn default() -> Self {
        Self {
            first_name: "".to_string(),
            last_name: "".to_string(),
            address: "".to_string(),
            state: "".to_string(),
            state_selected: 0,
            no_of_dependents: "".to_string(),
            married: false,
            pay: 0.0,
        }
    }
}

impl InsertEmployee {
    // pub fn update() {}

    pub fn ui_control(&mut self, ui: &mut egui::Ui) {
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

        let states: [&str; 2] = ["Texas", "Oklahoma"];

        egui::ComboBox::from_label("Select State").show_index(
            ui,
            &mut self.state_selected,
            states.len(),
            |i| states[i].to_string(),
        );

        ui.label("Number of Dependents");
        ui.text_edit_singleline(&mut self.no_of_dependents)
            .on_hover_text("type the state here");
        ui.label("Married?");
        ui.checkbox(&mut self.married, "Married");
        ui.label("Pay");
        ui.add(
            egui::Slider::new(&mut self.pay, 8.5..=100.0)
                .orientation(SliderOrientation::Vertical)
                .logarithmic(true)
                .clamp_to_range(true)
                .text("per hour"),
        )
        .on_hover_text("Pay");
    }
}

impl super::Admin for InsertEmployee {
    fn name(&self) -> &'static str {
        "Insert Employee"
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

impl super::View for InsertEmployee {
    fn ui(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.label("Insert Employee");
        });
        ui.separator();
        self.ui_control(ui);

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
                eprintln!("Could not add new Employee: {}", e)
            }
        }
    }
}
