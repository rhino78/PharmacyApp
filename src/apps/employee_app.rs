use egui::{containers::*, *};

pub struct Employee {
    first_name: String,
    last_name: String,
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
                self.ui(ctx, ui);
                self.display_grid(ctx, ui);
            });
        });
        //self.
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
}
