use eframe::egui::{widgets, Button, CentralPanel, Context, Grid, ScrollArea, TopBottomPanel};
use serde::{Deserialize, Serialize};

use super::{account::Account, package::Package};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AppSettings {
    pub accounts: Vec<Account>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AppData {
    pub packages: Vec<Package>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AppState {
    pub data: AppData,
    pub settings: AppSettings,
    pub url_window_open: bool,
}


impl AppState {
    pub fn set_creation_context(self, _cc: &eframe::CreationContext<'_>) -> Self {
        // TODO: customize cc
        self
    }

    pub(crate) fn ui_icon_bar(&mut self, ctx: &Context) {
        TopBottomPanel::top("Menu").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(5.0);
                ui.horizontal_wrapped(|ui| {
                    widgets::global_dark_light_mode_switch(ui);

                    ui.separator();

                    if ui
                        .add(Button::new("▶"))
                        .on_hover_text("Start all downloads")
                        .clicked()
                    {
                        // NOTE: this will start all downloads that are not finished yet
                        println!("START");
                    }

                    if ui
                        .add(Button::new("⏹"))
                        .on_hover_text("Stop all downloads")
                        .clicked()
                    {
                        // NOTE: stop all downloads
                        println!("STOP");
                    }

                    if ui
                        .add(Button::new("🗑"))
                        .on_hover_text("Delete all finished downloads")
                        .clicked()
                    {
                        // NOTE: remove finished downloads (scope packages)
                        self.clear();
                    }

                    ui.separator();

                    if ui.add(Button::new("⊞")).on_hover_text("Add urls").clicked() {
                        self.url_window_open = !self.url_window_open;
                    }
                });
                ui.add_space(5.0);
            });
        });
    }

    pub(crate) fn ui_download_grid(&self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    Grid::new("some_unique_id")
                        .striped(true)
                        .spacing((10.0, 20.0))
                        .num_columns(2)
                        .show(ui, |ui| {
                            ui.heading("Name");
                            ui.horizontal(|ui| {
                                ui.expand_to_include_rect(ui.available_rect_before_wrap());
                                ui.heading("Progress");
                            });
                            ui.end_row();

                            for package in self.data.packages.iter() {
                                let res = package.render(ui);

                                ui.vertical(|ui| {
                                    let progress = package.get_progress();
                                    ui.add(
                                        widgets::ProgressBar::new(progress)
                                            .show_percentage()
                                            .animate(progress < 1.0),
                                    );
                                    if res.body_response.is_some() {
                                        for part in package.parts.iter() {
                                            ui.add(
                                                widgets::ProgressBar::new(part.progress)
                                                    .show_percentage()
                                                    .animate(part.progress < 1.0),
                                            );
                                        }
                                    }
                                });
                                ui.end_row();
                            }
                        });
                });
        });
    }

    fn clear(&mut self) {
        self.data.packages = self
            .data
            .packages
            .clone()
            .into_iter()
            .filter(|pack| pack.get_progress() < 1.0)
            .collect();
    }
}
