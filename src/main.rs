mod lib;

use lib::model::AppData;

pub fn main() {
    let app = AppData::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
