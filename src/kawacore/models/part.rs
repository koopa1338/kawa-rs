use eframe::egui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Part {
    pub name: String,
    pub url: String,
    pub progress: f32,
    pub size: u64,
}

impl Part {
    pub(crate) fn render(&self, ui: &mut Ui) {
        ui.horizontal_wrapped(|ui| {
            ui.horizontal(|ui| {
                ui.expand_to_include_rect(ui.available_rect_before_wrap());
                ui.label(&self.name);
            });
        });
    }
}
