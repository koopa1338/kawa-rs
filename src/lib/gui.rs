use super::model::{AppData, Package, Part};
use eframe::{
    egui::{
        self, widgets, Align, Button, CentralPanel, CollapsingResponse, Context, Grid, Layout,
        ScrollArea, TopBottomPanel, Ui, Window,
    },
    epi::{self, App},
};

impl App for AppData {
    fn update(&mut self, ctx: &Context, _frame: &epi::Frame) {
        self.ui_icon_bar(ctx);
        self.ui_download_grid(ctx);

        TopBottomPanel::bottom("footer").show(ctx, |ui| {
            let layout = Layout::top_down(Align::Center).with_main_justify(true);
            ui.allocate_ui_with_layout(ui.available_size(), layout, |ui| {
                egui::warn_if_debug_build(ui);
            })
        });
        Window::new("Paste URLs")
            .open(&mut self.settings.url_window_open)
            .resizable(true)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.label("test");
            });
    }

    fn name(&self) -> &str {
        "Kawa Download Manager"
    }

    // NOTE: commented for usage of dummy data
    //
    // fn save(&mut self, storage: &mut dyn epi::Storage) {
    //     epi::set_value(storage, epi::APP_KEY, self);
    // }

    // fn setup(
    //     &mut self,
    //     _ctx: &egui::CtxRef,
    //     _frame: &epi::Frame,
    //     _storage: Option<&dyn epi::Storage>,
    // ) {
    //     if let Some(storage) = _storage {
    //         *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
    //     }
    // }
}

impl AppData {
    fn ui_icon_bar(&mut self, ctx: &Context) {
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

                    if ui
                        .add(Button::new("URLs"))
                        .on_hover_text("Add urls")
                        .clicked()
                    {
                        self.settings.url_window_open = !self.settings.url_window_open;
                    }
                });
                ui.add_space(5.0);
            });
        });
    }

    fn ui_download_grid(&self, ctx: &Context) {
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

                            for package in self.packages.iter() {
                                let res = package.render(ui);

                                ui.vertical(|ui| {
                                    ui.add(
                                        widgets::ProgressBar::new(package.progress)
                                            .show_percentage(),
                                    );
                                    if res.body_response.is_some() {
                                        for part in package.parts.iter() {
                                            ui.add(
                                                widgets::ProgressBar::new(part.progress)
                                                    .show_percentage(),
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
}

impl Part {
    fn render(&self, ui: &mut Ui) {
        ui.horizontal_wrapped(|ui| {
            ui.horizontal(|ui| {
                ui.expand_to_include_rect(ui.available_rect_before_wrap());
                ui.label(&self.name);
            });
        });
    }
}

impl Package {
    fn render(&self, ui: &mut Ui) -> CollapsingResponse<()> {
        ui.collapsing(&self.name, |ui| {
            for part in self.parts.iter() {
                part.render(ui);
            }
        })
    }
}
