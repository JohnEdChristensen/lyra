use crate::string_wave::StringWave;
use egui::{containers::*, *};

use crate::style::nord_ui_visuals;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct LyraApp {
    #[serde(skip)]
    wave: StringWave,
    settings_open: bool,
    updates_per_frame: usize,
}

impl Default for LyraApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            wave: StringWave::new(500, 100, 500.0, 0.9),
            settings_open: false,
            updates_per_frame: 1,
        }
    }
}
enum GuiMode {
    Mobile,
    Desktop,
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

    fn add_settings_ui(ui: &mut Ui, wave: &mut StringWave, updates_per_frame: &mut usize) {
        ui.vertical(|ui| {
            ui.add(
                egui::Slider::new(&mut wave.c, 0.001..=1.0)
                    .text("Wave Speed")
                    .clamp_to_range(true),
            );
            ui.add(
                egui::Slider::new(&mut wave.initial_num_points, wave.starting_pos_index..=1000)
                    .text("Number of Points")
                    .clamp_to_range(true),
            );
            ui.add(
                egui::Slider::new(updates_per_frame, 1..=2000)
                    .text("Sim updates per frame")
                    .clamp_to_range(true),
            );
            //reset button
            if ui.button("Reset").clicked() {
                wave.set_initial_conditions();
            }
        });
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
        let Self {
            wave,
            settings_open,
            updates_per_frame,
        } = self;

        //catppuccin_egui::set_theme(&ctx, catppuccin_egui::FRAPPE);
        //Update the state
        for _ in 0..*updates_per_frame {
            wave.update_wave();
        }
        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            egui::trace!(ui);
            ui.horizontal_wrapped(|ui| {
                ui.visuals_mut().button_frame = false;
                ui.toggle_value(settings_open, "⚙ Settings");
            });
        });

        let gui_mode = if ctx.available_rect().width() < 800.0 {
            GuiMode::Mobile
        } else {
            GuiMode::Desktop
        };

        //add settings ui
        match gui_mode {
            GuiMode::Mobile => {
                egui::TopBottomPanel::bottom("settings_panel").show_animated(
                    ctx,
                    self.settings_open,
                    |ui| {
                        LyraApp::add_settings_ui(ui, wave,updates_per_frame);
                    },
                );
            }
            GuiMode::Desktop => {
                egui::SidePanel::left("settings_panel").show_animated(
                    ctx,
                    self.settings_open,
                    |ui| {
                        LyraApp::add_settings_ui(ui, wave,updates_per_frame);
                    },
                );
            }
        }
        //main window
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
