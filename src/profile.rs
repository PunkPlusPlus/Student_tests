pub mod component {
    use eframe::{egui::{self, RichText}, epi};
    use crate::app::User;

    pub struct Profile;

    impl Profile {
        pub fn display_profile(user: &mut User, ctx: &egui::CtxRef, frame: &epi::Frame) {
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
    }

    
}