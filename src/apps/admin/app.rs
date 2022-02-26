#[derive(Default)]
pub struct AdminApp {
    admin_windows: super::AdminWindows,
}

impl epi::App for AdminApp {
    fn name(&self) -> &str {
        "Administration"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
        self.admin_windows.ui(ctx);
    }
}
