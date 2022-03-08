use eframe::{
    egui::{self, Align, Context, Layout, TopBottomPanel, Window},
    epi::{App, Frame, Storage},
};

use super::models::app::AppState;

impl App for AppState {
    fn update(&mut self, ctx: &Context, _frame: &Frame) {
        self.ui_icon_bar(ctx);
        self.ui_download_grid(ctx);

        TopBottomPanel::bottom("footer").show(ctx, |ui| {
            let layout = Layout::top_down(Align::Center).with_main_justify(true);
            ui.allocate_ui_with_layout(ui.available_size(), layout, |ui| {
                egui::warn_if_debug_build(ui);
            })
        });
        Window::new("Paste URLs")
            .open(&mut self.url_window_open)
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

    fn setup(&mut self, ctx: &Context, _frame: &Frame, _storage: Option<&dyn Storage>) {
        // if let Some(storage) = _storage {
        //     *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        // }
        ctx.set_visuals(egui::Visuals::dark());
    }
}
