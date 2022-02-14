pub mod Ex {
    use eframe::egui::{self, Button, Color32, CtxRef, FontDefinitions, FontFamily, Hyperlink, Label, Layout, Separator, TopBottomPanel};
    use crate::app::Answers;
    use crate::app::Question1;
    use crate::app::Question2;


pub const PADDING: f32 = 5.0;
//const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);
pub struct Exam {
    questions: Vec<Question>,
}

struct Variant {
    question: String,
    answer: String,
}

struct Question {
    title: String,
    variants: Vec<Variant>,
}




impl Question {
    pub fn new(title: &String) -> Question {
        let iter = (0..2).map(|a| Variant {
            question: format!("question{}", a),
            answer: format!("answer{}", a),
        });
        Question {
            title: title.to_string(),
            variants: Vec::from_iter(iter),
        }
    }
}

impl Exam {
    pub fn new() -> Exam {
        let it = (0..4).map(|a| Question {
            title: format!("title{}", a),
            variants: Vec::from_iter((0..2).map(|a| Variant {
            question: format!("question{}", a),
            answer: format!("answer{}", a),
        })),
        });
        Exam {
            questions: Vec::from_iter(it),
        }
    }

    pub fn render(&self, ui: &mut eframe::egui::Ui, answers: &mut Answers) {         
        egui::Grid::new("exam").show(ui, |ui| {
            for question in &self.questions {
                let title = question.title.to_string();
                ui.colored_label(CYAN, title);
                ui.add_space(PADDING);
                ui.radio_value(&mut answers.first, Question1::First, "One".to_string());
                ui.radio_value(&mut answers.first, Question1::Second, "two".to_string());
                // for var in &question.variants {
                //     ui.checkbox(&mut checked, var.question.to_string());
                // }
                ui.end_row();
            }
        });
    }
}

impl Default for Exam {
    fn default() -> Self {
        Self::new()
    }
}
}
