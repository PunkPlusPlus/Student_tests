use eframe::{egui::{self, RichText}, epi};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:    
    user: User,
    test_begin: bool,
    // this how you opt-out of serialization of a member
    #[cfg_attr(feature = "persistence", serde(skip))]
    value: f32,
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct User {
    username: String,
    password: String,
    group: String
}

impl Default for User {
    fn default() -> Self {
        User { username: "".to_string(), password: "".to_string(), group: "".to_string() }
    }
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            user: User {
                username: "".to_string(),
                password: "".to_string(),
                group: "".to_string()
            },
            value: 2.7,
            test_begin: false,
        }
    }
}

fn display_profile(user: &mut User, ctx: &egui::CtxRef, frame: &epi::Frame) {
    egui::SidePanel::left("side_panel").resizable(false).show(ctx, |ui| {
        ui.spacing_mut().item_spacing.y = 10.0;
        ui.heading("Profile");
        ui.add(egui::Separator::default().horizontal());
        ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {
            ui.horizontal(|ui| {
                ui.add(egui::Label::new(RichText::new("Studend name: ").text_style(egui::TextStyle::Monospace)));
                ui.add(egui::Label::new(RichText::new(&user.username).text_style(egui::TextStyle::Monospace)));                        
            });
            ui.add(egui::Separator::default().horizontal());
            ui.horizontal(|ui| {
                ui.add(egui::Label::new(RichText::new("Student password: ").text_style(egui::TextStyle::Monospace)));
                ui.add(egui::Label::new(RichText::new(&user.password).text_style(egui::TextStyle::Monospace)));
            });
        });

        
   });            
}



impl epi::App for TemplateApp {
    
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        let Self { user, value, test_begin } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Menu", |ui| {
                    if ui.button("Back").clicked() {
                        if *test_begin {
                            *test_begin = false;
                        } else {
                            frame.quit();
                        }
                        
                    }
                });
            });
        });

        
         

        let style = egui::TextStyle::Heading;
        
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.spacing_mut().item_spacing.y = 10.0;
            ui.heading("Login form");
            ui.label("Username: ");
            ui.add(egui::TextEdit::singleline(&mut user.username).text_style(style));
            ui.label("Password: ");
            ui.add(egui::TextEdit::singleline(&mut user.password).password(true).text_style(style));
            let button = egui::Button::new("Login").text_style(egui::TextStyle::Button);
            if ui.add(button).clicked() {
                *test_begin = true;
            }
                        
            //egui::warn_if_debug_build(ui);
        });
        if *test_begin == true {
            display_profile(user, ctx, frame);
            egui::CentralPanel::default().show(ctx, |ui| {
                
            });        
        }
        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
    

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn name(&self) -> &str {
        "Lab 1"
    }

    fn warm_up_enabled(&self) -> bool {
        false
    }

    fn on_exit(&mut self) {}

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn max_size_points(&self) -> egui::Vec2 {
        // Some browsers get slow with huge WebGL canvases, so we limit the size:
        egui::Vec2::new(1024.0, 2048.0)
    }

    fn clear_color(&self) -> egui::Rgba {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).into()
    }

    fn persist_native_window(&self) -> bool {
        true
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }
}
