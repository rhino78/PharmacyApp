use egui::*;

pub struct PayEntry {
    employee: String,
    pay: String,
}

impl Default for PayEntry {
    fn default() -> Self {
        Self {
            employee: "".to_string(),
            pay: "".to_string(),
        }
    }
}

impl PayEntry {
    pub fn ui_control(&mut self, ui: &mut egui::Ui) {
        ui.label("Employee");
        ui.text_edit_singleline(&mut self.employee);
        ui.label("Pay");
        ui.text_edit_singleline(&mut self.pay);
        let clearbtn = egui::Button::new("Execute");
        if ui.add(clearbtn).clicked() {
            println!("{}", self.employee);
        }
    }
}

impl super::Admin for PayEntry {
    fn name(&self) -> &'static str {
        "Enter Pay"
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

impl super::View for PayEntry {
    fn ui(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.label("Pay Entry");
        });
        self.ui_control(ui);
    }
}
