use egui::*;

use crate::apps::database::insert::insert_new_pay;

pub struct PayEntry {
    employee: String,
    pay: String,
    hours: String,
    // pub paydate: Option<Date<Utc>>,
    // pub paydate: Date<Utc>,
    paydate: String,
    payrate: String,
    withholding: String,
    roth_ira: String,
}

impl Default for PayEntry {
    fn default() -> Self {
        Self {
            employee: "".to_string(),
            pay: "".to_string(),
            hours: "".to_string(),
            paydate: "".to_string(),
            payrate: "".to_string(),
            withholding: "".to_string(),
            roth_ira: "".to_string(),
        }
    }
}

impl PayEntry {
    pub fn ui_control(&mut self, ui: &mut egui::Ui) {
        ui.label("Employee");
        ui.text_edit_singleline(&mut self.employee);
        ui.label("pay");
        ui.text_edit_singleline(&mut self.pay);
        ui.label("hours");
        ui.text_edit_singleline(&mut self.hours);
        ui.label("paydate");
        // ui.add(DatePicker::new("datepicker-unique-id", &mut self.paydate));
        ui.text_edit_singleline(&mut self.paydate);
        ui.label("payrate");
        ui.label(self.payrate.to_string());
        ui.label("withholding");
        ui.label(self.withholding.to_string());
        ui.label("roth");
        ui.label(self.roth_ira.to_string());
        ui.separator();

        let new_pay_btn = egui::Button::new("Insert New Pay Record");
        if ui.add(new_pay_btn).clicked() {
            if let Err(e) = insert_new_pay(
                self.pay.to_string(),
                self.hours.to_string(),
                self.paydate.to_string(),
            ) {
                eprintln!("Could not insert new pay record: {}", e)
            }
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
