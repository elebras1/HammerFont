use super::{font_settings::FontSettings, header::Header};
use eframe::egui;

pub struct HammerFontApp {
    header: Header,
    font_settings: FontSettings,
}

impl HammerFontApp {
    pub fn new(_creation_context: &eframe::CreationContext<'_>) -> Self {
        Self {
            header: Header::new(),
            font_settings: FontSettings::new(),
        }
    }
}

impl eframe::App for HammerFontApp {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("header").show(context, |ui| {
            self.header.ui(ui);
        });

        egui::SidePanel::left("font_settings")
            .default_width(300.0)
            .show(context, |ui| {
                self.font_settings.ui(ui);
            });
    }
}
