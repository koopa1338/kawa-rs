mod lib;

use lib::model::AppData;
use std::fs::read_to_string;

pub fn main() {
    let app: AppData = serde_json::from_str(&read_to_string("./demo.json").unwrap()).unwrap();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
