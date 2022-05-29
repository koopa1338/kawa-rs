mod lib;

use lib::models::app::AppState;

#[macro_use]
extern crate lazy_static_include;

// for testing purposes only
lazy_static_include_str! {
    STATE => "demo.json",
}

#[tokio::main]
async fn main() {
    let app: AppState = serde_json::from_str(&STATE).unwrap();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
