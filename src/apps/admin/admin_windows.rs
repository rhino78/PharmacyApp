use super::Admin;
use egui::{Context, CtxRef, ScrollArea, Ui};
use std::collections::BTreeSet;

struct Admins {
    admins: Vec<Box<dyn Admin>>,
    open: BTreeSet<String>,
}

impl Default for Admins {
    fn default() -> Self {
        Self::from_admins(vec![
            Box::new(super::user_name_pw::UserNamePW::default()),
            Box::new(super::pay_entry::PayEntry::default()),
            Box::new(super::edit_employee::EditEmployee::default()),
        ])
    }
}

impl Admins {
    pub fn from_admins(admins: Vec<Box<dyn Admin>>) -> Self {
        let mut open = BTreeSet::new();
        open.insert(
            super::widget_gallery::WidgetGallery::default()
                .name()
                .to_owned(),
        );

        Self { admins, open }
    }

    pub fn checkboxes(&mut self, ui: &mut Ui) {
        let Self { admins, open } = self;
        for admin in admins {
            let mut is_open = open.contains(admin.name());
            ui.checkbox(&mut is_open, admin.name());
            set_open(open, admin.name(), is_open);
        }
    }

    pub fn windows(&mut self, ctx: &CtxRef) {
        let Self { admins, open } = self;
        for admin in admins {
            let mut is_open = open.contains(admin.name());
            admin.show(ctx, &mut is_open);
            set_open(open, admin.name(), is_open);
        }
    }
}

fn set_open(open: &mut BTreeSet<String>, key: &'static str, is_open: bool) {
    if is_open {
        if !open.contains(key) {
            open.insert(key.to_owned());
        }
    } else {
        open.remove(key);
    }
}

#[derive(Default)]
pub struct AdminWindows {
    admins: Admins,
}

impl AdminWindows {
    pub fn ui(&mut self, ctx: &CtxRef) {
        let Self { admins } = self;

        egui::SidePanel::right("administration")
            .min_width(150.0)
            .default_width(180.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Admin");
                });
                ui.separator();

                ScrollArea::vertical().show(ui, |ui| {
                    ui.separator();
                    admins.checkboxes(ui);
                });
            });
        self.windows(ctx);
    }
    fn windows(&mut self, ctx: &CtxRef) {
        let Self { admins } = self;
        admins.windows(ctx);
    }
}
