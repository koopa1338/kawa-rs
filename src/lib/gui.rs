use super::model::AppData;
use eframe::{
    egui::{self, widgets, Align, Button, CentralPanel, Grid, Layout, ScrollArea, TopBottomPanel},
    epi,
};

impl epi::App for AppData {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &epi::Frame) {
        self.ui_icon_bar(ctx);
        self.ui_download_grid(ctx);

        TopBottomPanel::bottom("footer").show(ctx, |ui| {
            let layout = Layout::top_down(Align::Center).with_main_justify(true);
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

impl AppData {
    fn ui_icon_bar(&self, ctx: &eframe::egui::CtxRef) {
        TopBottomPanel::top("Menu").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(5.0);
                ui.horizontal_wrapped(|ui| {
                    widgets::global_dark_light_mode_switch(ui);

                    ui.separator();

                    if ui
                        .add(Button::new("Start"))
                        .on_hover_text("Start all downloads")
                        .clicked()
                    {
                        // NOTE: this will start all downloads that are not finished yet
                        println!("START");
                    }

                    if ui
                        .add(Button::new("Stop"))
                        .on_hover_text("Stop all downloads")
                        .clicked()
                    {
                        // NOTE: stop all downloads
                        println!("STOP");
                    }

                    if ui
                        .add(Button::new("Clear"))
                        .on_hover_text("Delete all finished downloads")
                        .clicked()
                    {
                        // NOTE: remove finished downloads (scope packages)
                        println!("CLEAR");
                    }

                    ui.separator();
                });
                ui.add_space(5.0);
            });
        });
    }

    fn ui_download_grid(&self, ctx: &eframe::egui::CtxRef) {
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    Grid::new("some_unique_id")
                        .striped(true)
                        .num_columns(2)
                        .spacing((10.0, 20.0))
                        .show(ui, |ui| {
                            ui.heading("Name");
                            ui.heading("progress");
                            ui.end_row();

                            for i in 1..100 {
                                ui.label(format!("Item {}", i));
                                ui.add(widgets::ProgressBar::new(0.8).show_percentage());
                                ui.end_row();
                            }
                        });
                });
        });
    }
}
