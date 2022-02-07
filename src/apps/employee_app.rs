use egui::{containers::*, *};

// a struct to make employees
// first_name: String
// last_name: String
pub struct Employee {
    pub first_name: String,
    pub last_name: String,
}

impl Default for Employee {
    fn default() -> Self {
        Self {
            first_name: "Pooo".to_string(),
            last_name: "mcGee".to_string(),
        }
    }
}

impl epi::App for Employee {
    fn name(&self) -> &str {
        "Employee"
    }

    fn update(&mut self, ctx: &CtxRef, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ScrollArea::both().auto_shrink([false; 2]).show(ui, |ui| {
                ui.text_edit_singleline(&mut self.first_name);
                self.ui(ctx, ui);
                self.make_button(ctx, ui);
                self.display_grid(ctx, ui);
            });
        });
    }
}

impl Employee {
    pub fn ui(&mut self, _ctx: &CtxRef, _ui: &mut Ui) {}
    fn display_grid(&mut self, _ctx: &CtxRef, _ui: &mut Ui) {
        egui::Grid::new("myGrid").striped(true).show(_ui, |ui| {
            ui.label("id");
            ui.label("First Name");
            ui.label("Last Name");
            ui.end_row();

            ui.label("ID");
            ui.label(&self.first_name);
            ui.label(&self.last_name);
            ui.end_row();
            ui.label("ID");
            ui.label(&self.first_name);
            ui.label(&self.last_name);
            ui.end_row();
            ui.label("ID");
            ui.label(&self.first_name);
            ui.label(&self.last_name);
            ui.end_row();
            ui.label("ID");
            ui.label(&self.first_name);
            ui.label(&self.last_name);
            ui.end_row();
        });
    }

    fn make_button(&self, _ctx: &CtxRef, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("first name");

            let mut foo = "";
            let _response = ui.add(
                egui::TextEdit::singleline(&mut foo)
                    .lock_focus(false)
                    .hint_text("Poop"),
            );

           let button2 = egui::Button::new("click me");
            if ui.add(button2).clicked() {
                let foo = "poop";
                println!("{}", foo);
            }
       });
    }
}
