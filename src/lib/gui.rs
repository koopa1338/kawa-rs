use super::model::AppData;
use eframe::{egui, epi};
use rfd::FileDialog;

impl epi::App for AppData {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &epi::Frame) {
        egui::TopBottomPanel::top("Menu").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        let _files = FileDialog::new()
                            .add_filter("text", &["txt", "rs"])
                            .pick_file();
                    }
                });
                if ui.button("Quit").clicked() {
                    frame.quit();
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Download Manager");
        });

        egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
            egui::warn_if_debug_build(ui);
        });
    }

    fn name(&self) -> &str {
        "Kawa Download Manager"
    }

    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }
}
