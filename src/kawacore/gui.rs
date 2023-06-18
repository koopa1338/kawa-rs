use eframe::{
    egui::{Context, Layout, TopBottomPanel, Window, warn_if_debug_build},
    emath::Align,
    App, Frame, Storage,
};

use super::models::app::AppState;

impl App for AppState {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.ui_icon_bar(ctx);
        self.ui_download_grid(ctx);

        TopBottomPanel::bottom("footer").show(ctx, |ui| {
            let layout = Layout::top_down(Align::Center).with_main_justify(true);
            ui.allocate_ui_with_layout(ui.available_size(), layout, |ui| {
                warn_if_debug_build(ui);
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

    fn save(&mut self, _storage: &mut dyn Storage) {}

    fn on_close_event(&mut self) -> bool {
        true
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {}

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn max_size_points(&self) -> eframe::egui::Vec2 {
        eframe::egui::Vec2::INFINITY
    }

    fn clear_color(&self, _visuals: &eframe::egui::Visuals) -> [f32; 4] {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        eframe::egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).to_normalized_gamma_f32()

        // _visuals.window_fill() would also be a natural choice
    }

    fn persist_native_window(&self) -> bool {
        true
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }

    fn warm_up_enabled(&self) -> bool {
        false
    }

    fn post_rendering(&mut self, _window_size_px: [u32; 2], _frame: &Frame) {}

    // NOTE: commented for usage of dummy data
    //
    // fn save(&mut self, storage: &mut dyn epi::Storage) {
    //     epi::set_value(storage, epi::APP_KEY, self);
    // }

    // fn setup(&mut self, ctx: &Context, _frame: &Frame, _storage: Option<&dyn Storage>) {
    //     // if let Some(storage) = _storage {
    //     //     *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
    //     // }
    //     ctx.set_visuals(egui::Visuals::dark());
    // }
}
