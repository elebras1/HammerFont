use crate::localisation::Localisation;
use eframe::egui;
use std::path::PathBuf;

#[derive(Clone, PartialEq, Eq)]
struct FontConfig {
    path: PathBuf,
    size: u32,
}

pub struct FontSettings {
    fonts: Vec<FontConfig>,
    new_font_size: u32,
    file_dialog: egui_file::FileDialog,
}

impl FontSettings {
    pub fn new() -> Self {
        let file_dialog =
            egui_file::FileDialog::open_file(None).show_files_filter(Box::new(|path| {
                path.extension()
                    .and_then(|extension| extension.to_str())
                    .map(|extension| {
                        extension.eq_ignore_ascii_case("ttf")
                            || extension.eq_ignore_ascii_case("otf")
                    })
                    .unwrap_or(false)
            }));

        Self {
            fonts: Vec::new(),
            new_font_size: 18,
            file_dialog,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, loc: &Localisation) {
        ui.heading(loc.font_settings_title());
        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            self.draw_add_font(ui, loc);
            ui.add_space(10.0);
            self.draw_font_list(ui, loc);
            ui.add_space(10.0);
            self.draw_atlas_button(ui, loc);
        });

        self.manage_file_dialog(ui);
    }

    fn draw_add_font(&mut self, ui: &mut egui::Ui, loc: &Localisation) {
        ui.group(|ui| {
            ui.label(loc.add_font_label());
            ui.horizontal(|ui| {
                ui.label(loc.size_label());
                ui.add(
                    egui::Slider::new(&mut self.new_font_size, 8..=128).suffix(loc.size_suffix()),
                );
            });
            if ui.button(loc.select_font_button()).clicked() {
                self.file_dialog.open();
            }
        });
    }

    fn draw_font_list(&mut self, ui: &mut egui::Ui, loc: &Localisation) {
        ui.group(|ui| {
            ui.label(format!(
                "{} ({})",
                loc.configured_fonts_label(),
                self.fonts.len()
            ));
            ui.separator();

            let mut to_remove: Option<usize> = None;
            for (idx, font) in self.fonts.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    let filename = font
                        .path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("???");
                    ui.label(filename);
                    ui.add(
                        egui::DragValue::new(&mut font.size)
                            .speed(1.0)
                            .range(8..=128)
                            .suffix(loc.size_suffix()),
                    );
                    if ui.small_button(loc.delete_button()).clicked() {
                        to_remove = Some(idx);
                    }
                });
                ui.separator();
            }

            if let Some(idx) = to_remove {
                self.fonts.remove(idx);
            }

            if self.fonts.is_empty() {
                ui.label(loc.no_fonts_label());
            }
        });
    }

    fn draw_atlas_button(&self, ui: &mut egui::Ui, loc: &Localisation) {
        if ui.button(loc.generate_atlas_button()).clicked() {
            // Todo
        }
    }

    fn manage_file_dialog(&mut self, ui: &mut egui::Ui) {
        if self.file_dialog.show(ui.ctx()).selected() {
            if let Some(path) = self.file_dialog.path() {
                let font_config: FontConfig = FontConfig {
                    path: path.to_path_buf(),
                    size: self.new_font_size,
                };
                if !self.fonts.contains(&font_config) {
                    self.fonts.push(font_config);
                } else {
                    // Todo: afficher un message d'erreur
                }
            }
        }
    }
}
