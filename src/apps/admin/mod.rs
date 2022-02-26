pub mod admin_windows;
pub mod app;
pub mod user_name_pw;
pub mod widget_gallery;

pub use {admin_windows::AdminWindows, app::AdminApp, widget_gallery::WidgetGallery};

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub trait Admin {
    fn name(&self) -> &'static str;
    fn show(&mut self, ctx: &egui::CtxRef, open: &mut bool);
}
