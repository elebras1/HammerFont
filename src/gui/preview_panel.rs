use crate::localisation::Localisation;
use eframe::egui;

pub struct PreviewPanel {
    zoom: f32,
    show_grid: bool,
}

impl PreviewPanel {
    pub fn new() -> Self {
        Self {
            zoom: 1.0,
            show_grid: true,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, loc: &Localisation) {
        ui.vertical(|ui| {
            self.top_bar(ui, loc);
            ui.separator();
            self.preview_area(ui);
            ui.separator();
        });
    }

    fn top_bar(&mut self, ui: &mut egui::Ui, loc: &Localisation) {
        ui.horizontal(|ui| {
            ui.heading(loc.preview_panel_title());
            ui.separator();
            self.zoom_controls(ui);
            ui.separator();
            ui.checkbox(&mut self.show_grid, "Grid");
        });
    }

    fn zoom_controls(&mut self, ui: &mut egui::Ui) {
        ui.label("Zoom:");
        if ui.button("➖").clicked() {
            self.zoom = (self.zoom - 0.25).max(0.25);
        }
        ui.label(format!("{:.0}%", self.zoom * 100.0));
        if ui.button("➕").clicked() {
            self.zoom = (self.zoom + 0.25).min(4.0);
        }
    }

    fn preview_area(&self, ui: &mut egui::Ui) {
        egui::ScrollArea::both().show(ui, |ui| {
            ui.group(|ui| {
                let available_size = ui.available_size();
                let (response, painter) = ui.allocate_painter(
                    egui::vec2(available_size.x, available_size.y.max(400.0)),
                    egui::Sense::hover(),
                );

                let rect = response.rect;
                painter.rect_filled(rect, 0.0, egui::Color32::from_gray(40));

                if self.show_grid {
                    self.draw_grid(&painter, rect);
                }
            });
        });
    }

    fn draw_grid(&self, painter: &egui::Painter, rect: egui::Rect) {
        let grid_spacing = 32.0 * self.zoom;

        let mut x = rect.left();
        while x < rect.right() {
            painter.line_segment(
                [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
                egui::Stroke::new(1.0, egui::Color32::from_gray(60)),
            );
            x += grid_spacing;
        }

        let y = rect.top();
        while y < rect.bottom() {
            painter.line_segment(
                [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
                egui::Stroke::new(1.0, egui::Color32::from_gray(60)),
            );
        }
    }
}
