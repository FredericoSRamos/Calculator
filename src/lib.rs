pub mod calculator;

#[derive(Default)]
pub struct MyApp {
    input: String,
    clear: bool
}

impl MyApp {
    fn can_add_char(&self, last_char: Option<char>, new_char: char) -> bool {
        if new_char == '.' {
            let mut dot = true;

            for c in self.input.chars() {
                if c == '.' {
                    dot = false;
                } else if matches!(c, '+' | '-' | 'x' | '/' | '%') {
                    dot = true;
                }
            }

            return dot;
        }

        match last_char {
            Some(c) if c == ')' => {
                // After a closing parenthesis, only allow operators or another closing parenthesis
                matches!(new_char, '+' | '-' | 'x' | '/' | '%')
            }
            Some(c) if ['+', '-', 'x', '/', '%'].contains(&c) => {
                // After an operator, allow digits or opening parenthesis
                matches!(new_char, '0'..='9')
            }
            Some(c) if c == '.' => {
                // After a dot, only allow digits
                matches!(new_char, '0'..='9')
            }
            Some(c) if c.is_digit(10) => {
                // After a digit, allow digits, operators or a dot
                matches!(new_char, '0'..='9' | '+' | '-' | 'x' | '/' | '%')
            }
            Some(c) if c == '(' => {
                // After an opening parenthesis, allow numbers, negative sign or another opening parenthesis
                matches!(new_char, '0'..='9' | '.' | '-')
            }
            None => {
                // At the start, allow digits, an opening parenthesis, a dot or a negative sign
                matches!(new_char, '0'..='9' | '-')
            }
            _ => true
        }
    }

    fn add_char(&mut self, c: char) {
        if self.clear {
            self.input.clear();
            self.clear = false;
        }

        if self.input.len() < 20 {
            self.input.push(c);
        }
    }

    fn handle_button_click(&mut self, c: char) {
        if self.clear {
            self.input.clear();
            self.clear = false;
        }
        
        if self.can_add_char(self.input.chars().last(), c) {
            self.add_char(c);
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let original_style = ctx.style().clone();
    
        let mut text_styles = std::collections::BTreeMap::new();
    
        text_styles.insert(egui::TextStyle::Heading, egui::FontId::new(16.0, egui::FontFamily::Proportional));
        text_styles.insert(egui::TextStyle::Body, egui::FontId::new(26.0, egui::FontFamily::Proportional));
        text_styles.insert(egui::TextStyle::Button, egui::FontId::new(24.0, egui::FontFamily::Proportional));
    
        let new_style = egui::Style {
            text_styles,
            visuals: original_style.visuals.clone(),
            ..Default::default()
        };
        
        ctx.set_style(new_style);

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.add_space(20.0);

            ui.add(egui::TextEdit::singleline(&mut self.input)
                .min_size(egui::vec2(385.0, 60.0)) 
                .interactive(false));

            ui.add_space(90.0);

            let button_size = egui::vec2(90.0, 60.0);

            ui.horizontal(|ui| {

                if ui.add(egui::Button::new("C").min_size(button_size)).clicked() {
                    self.input.clear();
                }

                if ui.add(egui::Button::new("()").min_size(button_size)).clicked() {
                    if matches!(self.input.chars().last(), Some('+') | Some('/') | Some('x') | Some('-') | Some('%') | Some('(') | None) {
                        self.add_char('(');
                    } else {
                        if self.input.chars().filter(|&c| c == '(').count() > self.input.chars().filter(|&c| c == ')').count() {
                            self.add_char(')');
                        } else {
                            if !self.clear {
                                self.add_char('x');
                            }
                            self.add_char('(');
                        }
                    }
                }

                if ui.add(egui::Button::new("+").min_size(button_size)).clicked() {
                    self.handle_button_click('+');
                }

                if ui.add(egui::Button::new("-").min_size(button_size)).clicked() {
                    self.handle_button_click('-');
                }
            });

            let symbols = vec!['x', '/', '%'];
            let chars = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'];

            for row in 0..3 {
                ui.horizontal(|ui| {
                    for col in 0..3 {
                        let index = row * 3 + col;

                        if ui.add(egui::Button::new(chars[index].to_string()).min_size(button_size)).clicked() {
                            self.handle_button_click(chars[index]);
                        }
                    }
                    if ui.add(egui::Button::new(symbols[row].to_string()).min_size(button_size)).clicked() {
                        self.handle_button_click(symbols[row]);
                    }
                });
            }

            ui.horizontal(|ui| {
                if ui.add(egui::Button::new(".").min_size(button_size)).clicked() {
                    if self.can_add_char(self.input.chars().last(), '.') {
                        self.add_char('.');
                    }
                }

                if ui.add(egui::Button::new("0").min_size(button_size)).clicked() {
                    if self.can_add_char(self.input.chars().last(), '0') {
                        self.add_char('0');
                    }
                }

                if ui.add(egui::Button::new("<-").min_size(button_size)).clicked() {
                    if self.clear {
                        self.input.clear();
                    } else {
                        self.input.pop();
                    }
                }

                if ui.add(egui::Button::new("=").min_size(button_size)).clicked() {
                    self.input = match calculator::calculate(&self.input) {
                        Ok(result) => format!("{}", result),
                        Err(error) => format!("{}", error)
                    };

                    self.clear = true;
                }
            });
        });
    }
}