pub mod Results {
    use crate::{Exam};
    use eframe::egui::{self, Button, Color32, RichText};
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
            return res;
        }

        pub fn render(&self, ui: &mut eframe::egui::Ui, res: &res) {
            let titles : [String; 5] = [
                "Если две окружности касаются одной прямой \n и лежат по одну сторону \n от этой прямой, то такая прямая является:".to_string(),
                "Если две окружности касаются одной прямой \n и лежат по разные стороны от этой прямой, \n то такая прямая является:".to_string(),
                "Сколько общих касательных можно провести к \n двум окружностям радиусами 5 см и 3 см, \n расстояние между центрами которых равно 12 см?".to_string(),
                "Сколько общих касательных можно провести к \n двум окружностям радиусами 5 см и 3 см, \n расстояние между центрами которых равно 5 см?".to_string(),
                "Катеты прямоугольного треугольника равны 6 и 8. \n Чему равна гипотенуза треугольника?".to_string()
            ];
            if res.first {
                ui.end_row();
                ui.colored_label(GREEN, &titles[0]);
                ui.end_row();
            } else {
                ui.end_row();
                ui.colored_label(RED, &titles[0]);
                ui.end_row();
            }
            ui.add(egui::Separator::default().horizontal());
            if res.second {
                ui.end_row();
                ui.colored_label(GREEN, &titles[1]);
                ui.end_row();
            } else {
                ui.end_row();
                ui.colored_label(RED, &titles[1]);
                ui.end_row();
            }
            ui.add(egui::Separator::default().horizontal());
            if res.third {
                ui.end_row();
                ui.colored_label(GREEN, &titles[2]);
                ui.end_row();
            } else {
                ui.end_row();
                ui.colored_label(RED, &titles[2]);
                ui.end_row();
            }
            ui.add(egui::Separator::default().horizontal());
            if res.fourth {
                ui.end_row();
                ui.colored_label(GREEN, &titles[3]);
                ui.end_row();
            } else {
                ui.end_row();
                ui.colored_label(RED, &titles[3]);
                ui.end_row();
            }
            ui.add(egui::Separator::default().horizontal());
            if res.five {
                ui.end_row();
                ui.colored_label(GREEN, &titles[4]);
                ui.end_row();
            } else {
                ui.end_row();
                ui.colored_label(RED, &titles[4]);
                ui.end_row();
            }
    
        }
    }
}