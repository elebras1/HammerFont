use eframe::egui;

pub struct Header {
    current_file: Option<String>,
    current_language: String,
}

impl Header {
    pub fn new() -> Self {
        Self {
            current_file: None,
            current_language: "English".to_string(),
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("🔨 HammerFont");

            ui.separator();

            if ui.button("📂 Open").clicked() {
                // Todo
            }

            if ui.button("💾 Save").clicked() {
                // Todo
            }

            if ui.button("💾 Save As...").clicked() {
                // Todo
            }

            ui.separator();

            egui::ComboBox::from_label("Language")
                .selected_text(&self.current_language)
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.current_language,
                        "English".to_string(),
                        "English",
                    );
                    ui.selectable_value(&mut self.current_language, "French".to_string(), "French");
                    ui.selectable_value(&mut self.current_language, "German".to_string(), "German");
                });

            ui.separator();

            if let Some(file) = &self.current_file {
                ui.label(format!("📄 {}", file));
            } else {
                ui.label("📄 No file");
            }
        });
    }
}
