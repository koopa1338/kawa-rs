use super::model::AppData;
use eframe::{egui, epi};

impl epi::App for AppData {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &epi::Frame) {
        egui::TopBottomPanel::top("Menu").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                egui::widgets::global_dark_light_mode_switch(ui);

                if ui.add(egui::Button::new("Start")).clicked() {
                    println!("START");
                }
                ui.separator();
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    egui::Grid::new("some_unique_id")
                        .striped(true)
                        .num_columns(2)
                        .spacing((10.0, 20.0))
                        .show(ui, |ui| {
                            ui.heading("Name");
                            ui.heading("progress");
                            ui.end_row();

                            for i in 1..100 {
                                ui.label(format!("Item {}", i));
                                ui.add(egui::widgets::ProgressBar::new(0.8).show_percentage());
                                ui.end_row();
                            }
                        });
                });
        });

        egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
            let layout = egui::Layout::top_down(egui::Align::Center).with_main_justify(true);
            ui.allocate_ui_with_layout(ui.available_size(), layout, |ui| {
                egui::warn_if_debug_build(ui);
            })
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
