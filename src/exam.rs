pub mod Ex {
    use eframe::egui::{self, Button, Color32};
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

struct Question {
    title: String,
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
            "Question2".to_string(),
            "Question3".to_string(),
            "Question4".to_string(),
            "Question5".to_string()

        ];
        
        let it = (titles).map(|a| Question {
            title: a,
        });
        Exam {
            questions: Vec::from_iter(it),
        }
    }

    pub fn render(&self, ui: &mut eframe::egui::Ui, answers: &mut Answers) {
        egui::Grid::new("exam").show(ui, |ui| {
            for (mut i, question) in self.questions.iter().enumerate() {
                match i {
                    0 => render_controls(ui, &mut answers.first, question.title.to_string()),
                    1 => render_controls(ui, &mut answers.second, question.title.to_string()),
                    2 => render_controls(ui, &mut answers.third, question.title.to_string()),
                    3 => render_controls(ui, &mut answers.fourth, question.title.to_string()),
                    4 => render_controls(ui, &mut answers.five, question.title.to_string()),
                    _ => i = 0,
                }
            }
        });
    }
}

fn render_controls(ui: &mut eframe::egui::Ui, answers: &mut Variants, title: String) {
    ui.end_row();
    ui.colored_label(CYAN, title);
    ui.add_space(PADDING);
    ui.radio_value(answers, Variants::First, "One".to_string());
    ui.radio_value(answers, Variants::Second, "two".to_string());
    ui.end_row();
    ui.separator();
}

impl Default for Exam {
    fn default() -> Self {
        Self::new()
    }
}
}
