mod gui;
mod localisation;
use localisation::{Language, Localisation};

fn main() -> Result<(), eframe::Error> {
    let localisation = Localisation::new(Language::English).expect("Failed to load localisation");
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title(localisation.title()),
        ..Default::default()
    };
    eframe::run_native(
        "HammerFont",
        options,
        Box::new(|creation_context| {
            Ok(Box::new(gui::HammerFontApp::new(
                creation_context,
                localisation,
            )))
        }),
    )
}
