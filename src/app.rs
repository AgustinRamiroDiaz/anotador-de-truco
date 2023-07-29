use crate::brain::*;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct MyApp {
    label_a: String,
    label_b: String,

    // #[serde(skip)]
    brain: Brain,
    // // this how you opt-out of serialization of a member
    // #[serde(skip)]
    // value: f32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            label_a: "Nosotres".to_owned(),
            label_b: "Elles".to_owned(),
            brain: Brain::new(),
        }
    }
}

impl MyApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for MyApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            self.brain
                .state_history
                .iter()
                .enumerate()
                .rev()
                .for_each(|(i, state)| {
                    let i = i + 1;
                    ui.label(format!(
                        "Ronda {}: {} {}-{} {}",
                        i, self.label_a, state.counterA, state.counterB, self.label_b
                    ));
                });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("some_unique_id").show(ui, |ui| {
                ui.text_edit_singleline(&mut self.label_a);

                if self.brain.canDecrementA() {
                    if ui.button("-").clicked() {
                        self.brain.update(Event::DecrementA);
                    }
                } else {
                    ui.label("");
                }

                ui.heading(self.brain.state.counterA.to_string());
                if ui.button("+").clicked() {
                    self.brain.update(Event::IncrementA);
                }
                ui.end_row();

                ui.text_edit_singleline(&mut self.label_b);

                if self.brain.canDecrementB() {
                    if ui.button("-").clicked() {
                        self.brain.update(Event::DecrementB);
                    }
                } else {
                    ui.label("");
                }
                ui.heading(self.brain.state.counterB.to_string());
                if ui.button("+").clicked() {
                    self.brain.update(Event::IncrementB);
                }
                ui.end_row();
            });

            ui.horizontal(|ui| {});

            if ui.button("Guardar ronda").clicked() {
                self.brain.update(Event::Commit);
            }

            if ui.button("Cargar ultima ronda").clicked() {
                self.brain.update(Event::Rollback);
            }

            if ui.button("Limpiar").clicked() {
                self.brain.update(Event::Clear);
            }

            egui::warn_if_debug_build(ui);
        });
    }
}
