pub mod admin_windows;
pub mod app;
pub mod edit_employee;
pub mod insert_employee;
pub mod pay_entry;
pub mod user_name_pw;

pub use {admin_windows::AdminWindows, app::AdminApp};

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub trait Admin {
    fn name(&self) -> &'static str;
    fn show(&mut self, ctx: &egui::CtxRef, open: &mut bool);
}
