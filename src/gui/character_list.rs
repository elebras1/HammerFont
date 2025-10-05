use crate::localisation::Localisation;
use eframe::egui;

pub struct CharacterList {
    text: String,
}

impl CharacterList {
    pub fn new() -> Self {
        let ascii_chars: String = (32u8..=126).map(|c| c as char).collect();
        Self { text: ascii_chars }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, loc: &Localisation) {
        ui.heading(loc.character_list_label());
        ui.separator();
        ui.add(egui::TextEdit::multiline(&mut self.text).desired_width(f32::INFINITY));
    }
}
