use crate::Rand;
use crate::game::stats::*;
use rand::prelude::*;
use egui_extras::{TableBuilder, Column};

pub struct TemplateApp {
    rand: Rand,
    player: crate::game::Player,
    attack_log: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            rand: Rand::seed_from_u64(0),
            player: crate::game::Player::new(),
            attack_log: String::new(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(_storage) = cc.storage {
            //return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        //eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: File->Quit exists on web pages but it wont do anything
                let is_web = cfg!(target_arch = "wasm32");
                ui.menu_button("File", |ui| {
                    ui.add_enabled_ui(!is_web, |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });
                ui.add_space(16.0);

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("RPG Stat Playground");

            TableBuilder::new(ui)
                .max_scroll_height(400.0)
                .column(Column::initial(150.0).at_least(100.0))
                .column(Column::initial(100.0).at_least(100.0))
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.heading("Stat Name");
                    });
                    header.col(|ui| {
                        ui.heading("Stat Value");
                    });
                })
                .body(|body| {
                    body.rows(20.0, ALL_STATS.len(), |mut row| {
                        let index = row.index();
                        let stat = ALL_STATS.get(index).unwrap();
                        row.col(|ui| {
                            ui.label(stat.name());
                        });
                        row.col(|ui| {
                            let val = self.player.stats.debug_get_mut(*stat);
                            let drag_value = egui::DragValue::new(val)
                                .update_while_editing(false);
                            ui.add(drag_value);
                        });
                    })
                });
            
            ui.separator();

            if ui.button("Simulate Attack").clicked() {
                self.attack_log = self.player.default_attack_test(&mut self.rand);
            }

            ui.label(&self.attack_log);

            ui.separator();

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
