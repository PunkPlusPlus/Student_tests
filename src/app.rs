use eframe::{egui::{self, RichText, Sense, Visuals}, epi};

use crate::Profile;
use crate::{Exam};
use crate::exam::Ex::{Variants, Answers};
use crate::{res};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    user: User,
    login: bool,
    test_passed: bool,
    test_begin: bool,
    answers: Answers,
    #[cfg_attr(feature = "persistence", serde(skip))]
    attempt: u32,
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct User {
    pub username: String,
    pub password: String,
    pub group: String
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
            attempt: 0,
            login: false,
            test_passed: false,
            test_begin: false,
            answers: Answers {
                first: Variants::First,
                second: Variants::First,
                third: Variants::First,
                fourth: Variants::First,
                five: Variants::First
            }
        }
    }
}

impl epi::App for TemplateApp {
    
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        let Self { user, attempt, test_begin , answers, login, test_passed} = self;
        ctx.set_visuals(Visuals::dark());
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Menu", |ui| {
                    if ui.button("Back").clicked() {
                        if *login {
                            *login = false;
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
            ui.heading(RichText::new("Login form"));
            ui.label(RichText::new("Username: "));
            ui.add(egui::TextEdit::singleline(&mut user.username).text_style(style));
            ui.label("Password: ");
            ui.add(egui::TextEdit::singleline(&mut user.password).password(true).text_style(style));
            let mut button = egui::Button::new(RichText::new("Login").text_style(egui::TextStyle::Button));
            if user.username.chars().count() == 0 || user.password.chars().count() == 0 {
                button = button.sense(Sense::hover());
            }            
            if ui.add(button).clicked() {
                *login = true;
            }
            if *login {
                Profile::display_profile(user, ctx, attempt, answers, test_begin, test_passed);
                egui::SidePanel::left("exam_panel").resizable(false).show(ctx, |ui| {
                    let exam = Exam::new();
                    if *test_begin {                        
                        exam.render(ui, answers, attempt, test_begin, test_passed);
                    }
                    if *test_passed {
                        let win = Exam::check(answers);
                        let res = res::get_instance(answers);
                        res.new_render(ui, &res, exam.questions);
                        if win {
                            ui.label("Test passed!");
                        } else {
                            ui.label("Try again!");
                        }
                    }
                                    
                });
            }            
            //egui::warn_if_debug_build(ui);
        });
        
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
        egui::Color32::from_rgba_unmultiplied(1, 1, 120, 180).into()
    }

    fn persist_native_window(&self) -> bool {
        true
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }
}
