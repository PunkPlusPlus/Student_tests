pub mod component {
    use eframe::{egui::{self, RichText, Sense}, epi};
    use crate::{app::User, exam::Ex::Answers, Exam};

    pub struct Profile;

    impl Profile {
        pub fn display_profile(user: &mut User, ctx: &egui::CtxRef, attempt: &mut u32, answers: &mut Answers, test_begin: &mut bool, test_passed: &mut bool) {
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
                        ui.add(egui::Label::new(RichText::new("Student group: ").text_style(egui::TextStyle::Monospace)));
                        ui.add(egui::Label::new(RichText::new("ИС-01").text_style(egui::TextStyle::Monospace)));
                    });
                    ui.add(egui::Separator::default().horizontal());
                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(RichText::new("Subject: ").text_style(egui::TextStyle::Monospace)));
                        ui.add(egui::Label::new(RichText::new("Geometry").text_style(egui::TextStyle::Monospace)));
                    });
                    ui.add(egui::Separator::default().horizontal());
                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(RichText::new("Attempt: ").text_style(egui::TextStyle::Monospace)));
                        ui.add(egui::Label::new(RichText::new(&attempt.to_string()).text_style(egui::TextStyle::Monospace)));
                    });
                    ui.add(egui::Separator::default().horizontal());
                    ui.horizontal(|ui| {
                        let mut btn = egui::Button::new(RichText::new("Begin test").text_style(egui::TextStyle::Button));
                        if *test_begin {
                            btn = btn.sense(Sense::hover());
                        }
                        if ui.add(btn).clicked() {                        
                            *test_begin = true;
                            *test_passed = false;                            
                        }
                    });
                    
                });
        
                
           });            
        }
    }

    
}