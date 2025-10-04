use crate::localisation::{Language, Localisation};
use eframe::egui;

pub struct Header {
    current_file: Option<String>,
    selected_language: Language,
    language_changed: bool,
}

impl Header {
    pub fn new() -> Self {
        Self {
            current_file: None,
            selected_language: Language::English,
            language_changed: false,
        }
    }

    pub fn language_changed(&mut self) -> Option<Language> {
        if self.language_changed {
            self.language_changed = false;
            Some(self.selected_language)
        } else {
            None
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, loc: &Localisation) {
        ui.horizontal(|ui| {
            ui.heading("🔨 HammerFont");
            ui.separator();

            if ui.button(loc.open_button()).clicked() {
                // Todo
            }
            if ui.button(loc.save_button()).clicked() {
                // Todo
            }
            if ui.button(loc.save_as_button()).clicked() {
                // Todo
            }

            ui.separator();

            egui::ComboBox::from_label(loc.language_label())
                .selected_text(loc.get_language_name(self.selected_language))
                .show_ui(ui, |ui| {
                    for lang in Language::all() {
                        if ui
                            .selectable_value(
                                &mut self.selected_language,
                                lang,
                                loc.get_language_name(lang),
                            )
                            .clicked()
                        {
                            self.language_changed = true;
                        }
                    }
                });

            ui.separator();

            if let Some(file) = &self.current_file {
                ui.label(format!("📄 {}", file));
            } else {
                ui.label(loc.no_file_label());
            }
        });
    }
}
