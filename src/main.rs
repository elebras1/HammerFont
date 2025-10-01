mod gui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("HammerFont - Bitmap Font Packing Tool"),
        ..Default::default()
    };

    eframe::run_native(
        "HammerFont",
        options,
        Box::new(|creation_context| Ok(Box::new(gui::HammerFontApp::new(creation_context)))),
    )
}
