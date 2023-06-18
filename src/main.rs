mod kawacore;
use kawacore::models::app::AppState;

#[macro_use]
extern crate lazy_static_include;

// for testing purposes only
lazy_static_include_str! {
    STATE => "demo.json",
}

#[tokio::main]
async fn main() -> eframe::Result<()> {
    let app: AppState = serde_json::from_str(&STATE).unwrap();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Kawa Downloadmanager",
        native_options,
        Box::new(|cc| Box::new(app.set_creation_context(cc))),
    )
}
