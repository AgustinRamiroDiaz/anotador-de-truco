use std::borrow::BorrowMut;

use egui::FontId;

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

        // let mut style: egui::Style = cc.egui_ctx.style();
        // style.spacing.text_edit_width = 10.0;
        // cc.egui_ctx.set_style(style);

        let font_id = FontId {
            size: 50.0,
            family: egui::FontFamily::Proportional,
        };

        let mut style = egui::Style::default();

        [
            egui::TextStyle::Body,
            egui::TextStyle::Button,
            egui::TextStyle::Heading,
        ]
        .into_iter()
        .for_each(|text_style| {
            style.text_styles.insert(text_style, font_id.clone());
        });

        cc.egui_ctx.set_style(style);

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

        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(true)
            .show(ctx, |ui| {
                let num_rows = self.brain.state_history.len();
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show_rows(
                        ui,
                        ui.text_style_height(&egui::TextStyle::Body),
                        num_rows,
                        |ui, row_range| {
                            egui::Grid::new("some_unique_id").show(ui, |ui| {
                                ui.label("Rondas");
                                ui.end_row();

                                ui.label(&self.label_a);
                                ui.label(&self.label_b);
                                ui.end_row();

                                self.brain
                                    .state_history
                                    .iter()
                                    .enumerate()
                                    .rev()
                                    .skip(row_range.start)
                                    .take(row_range.count())
                                    .for_each(|(_, state)| {
                                        ui.label(state.counterA.to_string());
                                        ui.label(state.counterB.to_string());
                                        ui.end_row()
                                    });
                            });
                        },
                    );
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.columns(2, |columns| {
                (|ui: &mut egui::Ui| {
                    ui.label("First column");
                    ui.text_edit_singleline(&mut self.label_a);
                    ui.horizontal(|ui| {
                        ui.add_enabled_ui(self.brain.can_decrement_a(), |ui| {
                            if ui.button("-").clicked() {
                                self.brain.update(Event::DecrementA);
                            }
                        });

                        ui.heading(self.brain.state.counterA.to_string());
                        if ui.button("+").clicked() {
                            self.brain.update(Event::IncrementA);
                        }
                    });
                })(&mut columns[0]);

                (|ui: &mut egui::Ui| {
                    ui.label("Second column");

                    ui.text_edit_singleline(&mut self.label_b);

                    ui.horizontal(|ui| {
                        ui.add_enabled_ui(self.brain.can_decrement_b(), |ui| {
                            if ui.button("-").clicked() {
                                self.brain.update(Event::DecrementB);
                            }
                        });
                        ui.heading(self.brain.state.counterB.to_string());
                        if ui.button("+").clicked() {
                            self.brain.update(Event::IncrementB);
                        }
                    });
                })(&mut columns[1]);
            });

            ui.vertical_centered(|ui| {
                ui.add_enabled_ui(self.brain.can_commit(), |ui| {
                    if ui.button("Guardar ronda").clicked() {
                        self.brain.update(Event::Commit);
                    }
                });

                ui.menu_button("Operaciones riesgosas", |ui| {
                    ui.add_enabled_ui(!self.brain.state_history.is_empty(), |ui| {
                        ui.horizontal(|ui| {
                            if ui.button("Cargar ultima ronda").clicked() {
                                self.brain.update(Event::Rollback);
                            }
                        });
                    });

                    if ui.button("Limpiar").clicked() {
                        self.brain.update(Event::Clear);
                    }
                });
            });

            egui::warn_if_debug_build(ui);
        });
    }
}
