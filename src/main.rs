#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod afc;
mod airlift;
mod airtraffic;
mod app;
mod apple;
mod device;
mod flasher;
mod image_skin;
mod i18n;
mod passthm;
mod scanner;
mod wallet_backup;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([960.0, 620.0])
            .with_min_inner_size([850.0, 560.0])
            .with_title("AirCard - Transit Scan Fix"),
        ..Default::default()
    };

    eframe::run_native(
        "AirCard - Transit Scan Fix",
        options,
        Box::new(|cc| Ok(Box::new(app::AirCardApp::new(cc)))),
    )
}
