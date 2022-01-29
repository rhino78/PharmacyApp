#[derive(Default)]
pub struct Apps {
    emp: crate::apps::Employee,
    pay: crate::apps::Pay,
}

impl Apps {
    fn iter_mut(&mut self) -> impl Iterator<Item = (&str, &mut dyn epi::App)> {
        vec![
            ("emp", &mut self.emp as &mut dyn epi::App),
            ("pay", &mut self.pay as &mut dyn epi::App),
        ]
        .into_iter()
    }
}

#[derive(Default)]
pub struct PharmacyApp {
    selected_anchor: String,
    apps: Apps,
}

impl epi::App for PharmacyApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        if self.selected_anchor.is_empty() {
            println!("{} ", self.selected_anchor);
            self.selected_anchor = self.apps.iter_mut().next().unwrap().0.to_owned();
        }
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::trace!(ui);
            self.bar_contents(ui, frame);
        });

        for (anchor, app) in self.apps.iter_mut() {
            if anchor == self.selected_anchor || ctx.memory().everything_is_visible() {
                app.update(ctx, frame);
            }
        }
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    fn warm_up_enabled(&self) -> bool {
        false
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn on_exit(&mut self) {}

    fn name(&self) -> &str {
        "Pharmacy App"
    }
}

impl PharmacyApp {
    fn bar_contents(&mut self, ui: &mut egui::Ui, frame: &epi::Frame) {
        ui.horizontal_wrapped(|ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
                ui.separator();

                for (anchor, app) in self.apps.iter_mut() {
                    if ui
                        .selectable_label(self.selected_anchor == anchor, app.name())
                        .clicked()
                    {
                        self.selected_anchor = anchor.to_owned();
                    }
                }
            });
        });
    }
}
