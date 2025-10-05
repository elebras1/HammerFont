use crate::localisation::Localisation;
use eframe::egui;
use std::collections::HashSet;

enum DialogType {
    File,
    Folder,
}

pub struct CharacterList {
    text: String,
    language_id: String,
    file_dialog: egui_file::FileDialog,
    folder_dialog: egui_file::FileDialog,
    active_dialog: Option<DialogType>,
}

impl CharacterList {
    pub fn new() -> Self {
        let ascii_chars: String = (32u8..=126).map(|c| c as char).collect();

        let file_dialog =
            egui_file::FileDialog::open_file(None).show_files_filter(Box::new(|path| {
                path.extension()
                    .and_then(|extension| extension.to_str())
                    .map(|extension| extension.eq_ignore_ascii_case("csv"))
                    .unwrap_or(false)
            }));

        let folder_dialog = egui_file::FileDialog::select_folder(None);

        Self {
            text: ascii_chars,
            language_id: String::new(),
            file_dialog,
            folder_dialog,
            active_dialog: None,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, loc: &Localisation) {
        ui.heading(loc.character_list_label());
        ui.separator();
        self.import_characters(ui, loc);
        self.text_field(ui);
        self.manage_file_dialog(ui);
    }

    fn import_characters(&mut self, ui: &mut egui::Ui, loc: &Localisation) {
        ui.horizontal(|ui| {
            ui.label(loc.column_number_id_label());
            ui.add(egui::TextEdit::singleline(&mut self.language_id).desired_width(100.0));

            if ui.button(loc.import_from_file_button()).clicked() {
                self.active_dialog = Some(DialogType::File);
                self.file_dialog.open();
            }

            if ui.button(loc.import_from_folder_button()).clicked() {
                self.active_dialog = Some(DialogType::Folder);
                self.folder_dialog.open();
            }
        });
    }

    fn text_field(&mut self, ui: &mut egui::Ui) {
        let response =
            ui.add(egui::TextEdit::multiline(&mut self.text).desired_width(f32::INFINITY));
        if response.changed() {
            Self::make_unique(&mut self.text);
        }
    }

    fn make_unique(text: &mut String) {
        let mut seen = HashSet::new();
        *text = text.chars().filter(|c| seen.insert(*c)).collect();
    }

    fn manage_file_dialog(&mut self, ui: &mut egui::Ui) {
        if self.language_id.is_empty() {
            if self.active_dialog.is_some() {
                self.active_dialog = None;
            }
            return;
        }

        if matches!(self.active_dialog, Some(DialogType::File)) {
            if self.file_dialog.show(ui.ctx()).selected() {
                if let Some(path) = self.file_dialog.path() {
                    // TODO
                }
                self.active_dialog = None;
            }
        }

        if matches!(self.active_dialog, Some(DialogType::Folder)) {
            if self.folder_dialog.show(ui.ctx()).selected() {
                if let Some(path) = self.folder_dialog.path() {
                    // TODO
                }
                self.active_dialog = None;
            }
        }
    }
}
