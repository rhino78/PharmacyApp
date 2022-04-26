use egui::{containers::*, *};

use super::db_conn::select_all_emp;
/// a struct to make employees
/// first_name: String
/// last_name: String
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

impl Default for Employee {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            first_name: "".to_string(),
            last_name: "".to_string(),
            address: "".to_string(),
            state: "".to_string(),
            married: "".to_string(),
            dependents: "".to_string(),
            pay: "".to_string(),
        }
    }
}

impl epi::App for Employee {
    fn name(&self) -> &str {
        "Employee List"
    }

    fn update(&mut self, ctx: &CtxRef, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ScrollArea::both().auto_shrink([false; 2]).show(ui, |ui| {
                self.ui(ctx, ui);
                self.display_grid(ctx, ui);
                self.ui(ctx, ui);
            });
        });
    }
}

impl Employee {
    pub fn ui(&mut self, _ctx: &CtxRef, _ui: &mut Ui) {}
    fn display_grid(&mut self, _ctx: &CtxRef, _ui: &mut Ui) {
        let emps = select_all_emp();
        match emps {
            Ok(emp) => {
                let bruh = emp.unwrap();
                egui::Grid::new("bruh").striped(true).show(_ui, |ui| {
                    ui.label("ID");
                    ui.label("First Name");
                    ui.label("Last Name");
                    ui.end_row();

                    for b in bruh.iter() {
                        ui.label(b.id.to_string());
                        ui.label(b.first_name.to_string());
                        ui.label(b.last_name.to_string());
                        ui.end_row();
                    }
                });
            }
            Err(e) => eprintln!("{}", e),
        };
    }
}
