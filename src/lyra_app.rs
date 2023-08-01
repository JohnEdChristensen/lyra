use crate::string_wave::StringWave;
use egui::{containers::*, *};

use crate::style::nord_ui_visuals;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct LyraApp {
    #[serde(skip)]
    wave: StringWave,
}

impl Default for LyraApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            wave: StringWave::new(500, 100, 500.0),
        }
    }
}

impl LyraApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        cc.egui_ctx.set_visuals(nord_ui_visuals());

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for LyraApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { wave } = self;
        //Update the state
        wave.update();
        //simple performance graph in the top left
        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        //#[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::CentralPanel::default().show(ctx, |ui| {
            Frame::canvas(ui.style()).show(ui, |ui| {
                ui.ctx().request_repaint();

                let desired_size = ui.available_width() * vec2(1.0, 0.35);
                let (_id, rect) = ui.allocate_space(desired_size);

                let to_screen = emath::RectTransform::from_to(self.wave.get_bounding_rect(), rect);

                let mut shapes = vec![];
                //get window width
                let points = self.wave.get_points(to_screen);
                shapes.push(epaint::Shape::line(
                    points,
                    Stroke::new(2.0, Color32::WHITE),
                ));

                ui.painter().extend(shapes);
            });
        });
    }
}
