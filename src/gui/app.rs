use super::{font_settings::FontSettings, header::Header, preview_panel::PreviewPanel};
use crate::localisation::Localisation;
use eframe::egui;
use egui::{FontData, FontDefinitions, FontFamily};
use std::sync::Arc;

pub struct HammerFontApp {
    header: Header,
    font_settings: FontSettings,
    preview_panel: PreviewPanel,
    localisation: Localisation,
}

impl HammerFontApp {
    pub fn new(creation_context: &eframe::CreationContext, localisation: Localisation) -> Self {
        setup_chinese_font(&creation_context.egui_ctx);
        Self {
            header: Header::new(),
            font_settings: FontSettings::new(),
            preview_panel: PreviewPanel::new(),
            localisation,
        }
    }
}

impl eframe::App for HammerFontApp {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(new_language) = self.header.language_changed() {
            if let Err(e) = self.localisation.set_language(new_language) {
                eprintln!("Failed to change language: {}", e);
            } else {
                context.send_viewport_cmd(egui::ViewportCommand::Title(
                    self.localisation.title().to_string(),
                ));
            }
        }

        egui::TopBottomPanel::top("header").show(context, |ui| {
            self.header.ui(ui, &self.localisation);
        });

        egui::SidePanel::left("font_settings")
            .default_width(300.0)
            .show(context, |ui| {
                self.font_settings.ui(ui, &self.localisation);
            });

        egui::CentralPanel::default().show(context, |ui| {
            self.preview_panel.ui(ui, &self.localisation);
        });
    }
}

fn setup_chinese_font(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "NotoSansSC".to_owned(),
        Arc::new(FontData::from_owned(
            include_bytes!("../../assets/NotoSansSC-VariableFont_wght.ttf").to_vec(),
        )),
    );

    for family in [FontFamily::Proportional, FontFamily::Monospace] {
        let entry = fonts.families.entry(family).or_default();
        if !entry.contains(&"NotoSansSC".to_string()) {
            entry.push("NotoSansSC".to_string());
        }
    }

    ctx.set_fonts(fonts);
}
