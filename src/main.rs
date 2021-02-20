mod lib;

use druid::{AppLauncher, WindowDesc};
use lib::{gui, model::AppData};

pub fn main() {
    let main_window = WindowDesc::new(gui::ui_builder);
    // TODO: Set our initial data, handle earlier sessions
    // Check the best way to persist data
    let data = AppData::new();
    AppLauncher::with_window(main_window)
        .launch(data)
        .expect("launch failed");
}
