pub mod Results {
    use crate::{Exam};
    use eframe::egui::{self, Color32};
    use crate::exam::Ex::{Variants, Answers, Question};
    pub struct res {
        general: bool,
        first: bool,
        second: bool,
        third: bool,
        fourth: bool,
        five: bool
    }

    const RED: Color32 = Color32::from_rgb(245, 87, 66);
    const GREEN: Color32 = Color32::from_rgb(66, 245, 105);

    impl res {
        pub fn get_instance(answers: &Answers) -> res {
            let mut res = res{
                general: false,
                first: false,
                second: false,
                third: false,
                fourth: false,
                five: false
            };
            res.general = Exam::check(answers);
            if answers.first == Variants::First {
                res.first = true;
            }
            if answers.second == Variants::Second {
                res.second = true;
            }
            if answers.third == Variants::Second {
                res.third = true;
            }
            if answers.fourth == Variants::First {
                res.fourth = true;
            }
            if answers.five == Variants::First {
                res.five = true;
            }
            res
        }

        pub fn render_controls(&self, ui: &mut eframe::egui::Ui, res: &bool, title: String) {
            let mut color = RED;
            if *res {
                color = GREEN;
            }
            ui.end_row();
            ui.colored_label(color, title);
            ui.end_row();
            ui.add(egui::Separator::default().horizontal());
        }

        pub fn new_render(&self, ui: &mut eframe::egui::Ui, res: &res, titles: Vec<Question>) {            
            for (i, title) in titles.into_iter().enumerate() {
                match i {
                    0 => self.render_controls(ui, &res.first, title.title),
                    1 => self.render_controls(ui, &res.second, title.title),
                    2 => self.render_controls(ui, &res.third, title.title),
                    3 => self.render_controls(ui, &res.fourth, title.title),
                    4 => self.render_controls(ui, &res.five, title.title),
                    _ => self.render_controls(ui, &false, "Error".to_string()),
                }
            }
        }        
    }
}