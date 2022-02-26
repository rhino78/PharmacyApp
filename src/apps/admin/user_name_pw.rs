use egui::*;

pub struct UserNamePW {
    username: String,
    password: String,
}

impl Default for UserNamePW {
    fn default() -> Self {
        Self {
            username: "".to_string(),
            password: "".to_string(),
        }
    }
}

impl UserNamePW {
    pub fn ui_control(&mut self, ui: &mut egui::Ui) {
        ui.label("UserName");
        ui.text_edit_singleline(&mut self.username);
        ui.label("Password");
        ui.text_edit_singleline(&mut self.password);
        let clearbtn = egui::Button::new("Execute");
        if ui.add(clearbtn).clicked() {
            println!("{}", self.username);
        }
    }
}

impl super::Admin for UserNamePW {
    fn name(&self) -> &'static str {
        "User Name and PW"
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

impl super::View for UserNamePW {
    fn ui(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.label("Change User Name and PW");
        });
        self.ui_control(ui);
    }
}
