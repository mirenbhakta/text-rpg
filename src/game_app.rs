use egui::*;

pub struct GameApp {
    monitor: String,
}

impl eframe::App for GameApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.top_panel(ctx);
        self.game_panel(ctx);
    }
}

impl GameApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {

        

        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(_storage) = cc.storage {
            //return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        GameApp {
            monitor: String::new()
        }
    }

    fn top_panel(&mut self, ctx: &Context) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.add_space(10.0);
                // File menu
                ui.menu_button("File", |ui| {
                    const IS_WEB: bool = cfg!(target_arch = "wasm32");
                    let button = Button::new("Quit");
                    if ui.add_enabled(!IS_WEB, button).clicked() {
                        ctx.send_viewport_cmd(ViewportCommand::Close);
                    }
                });

                // right hand side theme pref
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    widgets::global_theme_preference_buttons(ui);
                });
            })
        });
    }

    fn game_panel(&mut self, ctx: &Context) {
        let top_bar_height = ctx.style().spacing.interact_size.y + 10.0;
        Window::new("Monitor")
            .anchor(Align2::CENTER_TOP, [0.0, top_bar_height])
            .collapsible(false)
            .movable(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.set_min_size([800.0, 600.0].into());
                ui.label("1. example with 10 lines\n2.\n3.\n4.\n5.\n6.\n7.\n8.\n9.\n10.");
        });
    }
}

pub const INCONSOLATA: &'_ str = "Inconsolata";

fn load_inconsolata(ctx: &Context) {
    let data = include_bytes!("../Inconsolata/Inconsolata-VariableFont_wdth,wght.ttf");
    
}
