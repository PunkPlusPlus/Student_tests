pub mod Ex {
    use std::env::VarError;

    use eframe::egui::{self, Button, Color32, RichText};
    //use crate::app::Answers;

    pub const PADDING: f32 = 5.0;
    //const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
    const CYAN: Color32 = Color32::from_rgb(0, 255, 255);
    pub struct Exam {
        questions: Vec<Question>,
    }

    pub struct Answers {
        pub first: Variants,
        pub second: Variants,
        pub third: Variants,
        pub fourth: Variants,
        pub five: Variants
    }

    pub struct Question {
        pub title: String,
        //variants: Vec<Variant>,
    }

    #[derive(PartialEq)]
    pub enum Variants {
        First,
        Second
    }

    impl Exam {
        pub fn new() -> Exam {
            let titles : [String; 5] = [
                "Если две окружности касаются одной прямой \n и лежат по одну сторону \n от этой прямой, то такая прямая является:".to_string(),
                "Если две окружности касаются одной прямой \n и лежат по разные стороны от этой прямой, \n то такая прямая является:".to_string(),
                "Сколько общих касательных можно провести к \n двум окружностям радиусами 5 см и 3 см, \n расстояние между центрами которых равно 12 см?".to_string(),
                "Сколько общих касательных можно провести к \n двум окружностям радиусами 5 см и 3 см, \n расстояние между центрами которых равно 5 см?".to_string(),
                "Катеты прямоугольного треугольника равны 6 и 8. \n Чему равна гипотенуза треугольника?".to_string()
            ];
            
            let it = (titles).map(|a| Question {
                title: a,
            });
            Exam {
                questions: Vec::from_iter(it),
            }
        }

        pub fn check(an: &Answers) -> bool {
            if an.first == Variants::First && an.second == Variants::Second && an.third == Variants::Second && an.fourth == Variants::First && an.five == Variants::First {
                return true;    
            } else {
                return false;
            }
        }

        pub fn getQuestions(ex: Exam) -> Vec<Question> {
            return ex.questions;
        }

        /*
        win combination:
        first-second-second-first-first
        */
        pub fn render(&self, ui: &mut eframe::egui::Ui, answers: &mut Answers, attempt: &mut u32, test_begin: &mut bool, test_passed: &mut bool) {
            egui::Grid::new("exam").show(ui, |ui| {
                for (mut i, question) in self.questions.iter().enumerate() {
                    match i {
                        0 => render_controls(ui, &mut answers.first, question.title.to_string(), &["Общей внешней \n касательной".to_string(), "Общей прямой".to_string()]),
                        1 => render_controls(ui, &mut answers.second, question.title.to_string(), &["Общей внешней \n касательной".to_string(), "Общей внутренней \n касательной".to_string()]),
                        2 => render_controls(ui, &mut answers.third, question.title.to_string(), &["Одну".to_string(), "Четыре".to_string()]),
                        3 => render_controls(ui, &mut answers.fourth, question.title.to_string(), &["Две".to_string(), "Три".to_string()]),
                        4 => render_controls(ui, &mut answers.five, question.title.to_string(), &["10".to_string(), "46".to_string()]),
                        _ => i = 0,
                    }
                }
                ui.end_row();
                let btn = egui::Button::new(RichText::new("Pass").text_style(egui::TextStyle::Button));                    
                    if ui.add(btn).clicked() {                        
                         *attempt += 1;
                         *test_begin = false;
                         *test_passed = true;
                    }
            });
        }
    }

    fn render_controls(ui: &mut eframe::egui::Ui, answers: &mut Variants, title: String, variants: &[String]) {
        ui.end_row();
        ui.colored_label(CYAN, title);
        ui.add_space(PADDING);
        ui.radio_value(answers, Variants::First, &variants[0]);
        ui.radio_value(answers, Variants::Second, &variants[1]);
        ui.end_row();
        ui.separator();
    }

    impl Default for Exam {
        fn default() -> Self {
            Self::new()
        }
    }
}
