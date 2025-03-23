#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_resizable(false).with_inner_size([400.0, 700.0]),
        centered: true,
        ..Default::default()
    };
    eframe::run_native(
        "Calculator",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Debug)]
enum Token {
    Number(f64),
    Operator(char),
}

fn calculate(expression: &String) -> Result<f64, String> {
    let mut parsed_expression = Vec::new();

    let numbers = ['.', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let operators = ['+', '-', 'x', '/', '%'];
    
    let mut is_num = false;
    let mut num_start = 0;

    for (i, item) in expression.chars().enumerate() {

        if numbers.contains(&item) {
            if !is_num {
                is_num = true;
                num_start = i;
            }
        } else {
            if is_num {
                let number: f64 = expression[num_start..i].parse().map_err(|_| "Invalid number".to_string())?;
                parsed_expression.push(Token::Number(number));
                is_num = false;
            }
  
            if (item == '-') && (i == 0 || (i > 0 && expression.chars().nth(i - 1) == Some('('))) {
                is_num = true;
                num_start = i;
            } else {
                parsed_expression.push(Token::Operator(item));
            }
        }
    }

    if is_num {
        let number: f64 = expression[num_start..expression.len()].parse().map_err(|_| "Invalid number".to_string())?;
        parsed_expression.push(Token::Number(number));
    }

    let mut queue = Vec::new();
    let mut stack = Vec::<char>::new();

    for token in parsed_expression {
        match token {
            Token::Number(value) => {
                queue.push(value);
            }
            Token::Operator(op) => {
                if operators.contains(&op) {
                    while let Some(last) = stack.last() {
                        if op == 'x' || op == '/' || op == '%' {
                            if last == &'+' || last == &'-' || last == &'(' {
                                break;
                            }
                        }
                        if last == &'+' || last == &'-' {
                            let y = queue.pop().unwrap();
                            let x = queue.pop().unwrap();

                            match stack.pop().unwrap() {
                                'x' => queue.push(x * y),
                                '/' => queue.push(x / y),
                                '%' => queue.push(x % y),
                                '+' => queue.push(x + y),
                                '-' => queue.push(x - y),
                                _ => return Err("An error occurred at ops".into())
                            }
                            println!("{:?}", queue);
                        } else {
                            break;
                        }
                    }

                    stack.push(op);
                }
                else if op == ')' {
                    loop {
                        match stack.pop() {
                            Some(last) => {
                                if last == '(' {
                                    break;
                                }
        
                                let y = queue.pop().unwrap();
                                let x = queue.pop().unwrap();
        
                                match last {
                                    'x' => queue.push(x * y),
                                    '/' => queue.push(x / y),
                                    '%' => queue.push(x % y),
                                    '+' => queue.push(x + y),
                                    '-' => queue.push(x - y),
                                    _ => return Err("An error occurred here".into())
                                }
                            },
                            None => return Err("Missmatched parentheses".into())
                        }

                    }
                }
                else {
                    stack.push(op);
                }
            }
        }
    }

    while let Some(last) = stack.pop() {


            let y = queue.pop().ok_or("Insufficient values for operation")?;
            let x = queue.pop().ok_or("Insufficient values for operation")?;

            let result = match last {
                '+' => x + y,
                '-' => x - y,
                'x' => x * y,
                '/' => {
                    if y == 0.0 {
                        return Err("Division by 0".into());
                    }
                    x / y
                }
                '%' => x % y,
                '(' => return Err("Missmatched parentheses".into()),
                _ => return Err("Invalid operator".into()),
            };
            queue.push(result);        
    }

    if let Some(result) = queue.pop() {
        Ok(result)
    } else {
        Err("".into())
    }
}

#[derive(Default)]
struct MyApp {
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
                matches!(new_char, '1'..='9' | '(')
            }
            Some(c) if c == '.' => {
                // After a dot, only allow digits
                matches!(new_char, '1'..='9')
            }
            Some(c) if c.is_digit(10) => {
                // After a digit, allow digits, operators or a dot
                matches!(new_char, '1'..='9' | '+' | '-' | 'x' | '/' | '%')
            }
            Some(c) if c == '(' => {
                // After an opening parenthesis, allow numbers, negative sign or another opening parenthesis
                matches!(new_char, '1'..='9' | '.' | '-' | '(')
            }
            None => {
                // At the start, allow digits, an opening parenthesis, a dot or a negative sign
                matches!(new_char, '1'..='9' | '(' | '-')
            }
            _ => true
        }
    }

    fn add_char(&mut self, c: char) {
        if self.clear {
            self.input.clear();
            self.clear = false;
        }

        self.input.push(c);
    }

    fn handle_button_click(&mut self, c: char) {
        let last_char = self.input.chars().last();

        if self.can_add_char(last_char, c) {
            self.add_char(c);
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.text_edit_singleline(&mut self.input);

            let symbols = vec!['x', '/', '%'];
            let chars = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'];

            ui.horizontal(|ui| {

                if ui.button("C").clicked() {
                    self.input.clear();
                }

                if ui.button("()").clicked() {
                    let last_char = self.input.chars().last();

                    if matches!(last_char, Some('+') | Some('/') | Some('x') | Some('-') | Some('%') | Some('(') | None) {
                        self.add_char('(');
                    } else {
                        if self.input.chars().filter(|&c| c == '(').count() > self.input.chars().filter(|&c| c == ')').count() {
                            self.add_char(')');
                        } else {
                            self.add_char('x');
                            self.add_char('(');
                        }
                    }
                }

                if ui.button("+").clicked() {
                    self.handle_button_click('+');
                }

                if ui.button("-").clicked() {
                    self.handle_button_click('-');
                }

            });

            let mut counter = 1;

            for _ in 0..3 {
                ui.horizontal(|ui| {
                    for j in 0..3 {
                        if ui.button(chars[counter + j - 1].to_string()).clicked() {
                            self.handle_button_click(chars[counter + j - 1]);
                        }
                    }

                    if ui.button(symbols[counter / 3].to_string()).clicked() {
                        self.handle_button_click(symbols[counter / 3]);
                    }

                    counter += 3;
                });
            }

            ui.horizontal(|ui| {
                if ui.button(".").clicked() {
                    let last_char = self.input.chars().last();

                    if self.can_add_char(last_char, '.') {
                        self.add_char('.');
                    }

                }

                if ui.button("0").clicked() {
                    if self.input.chars().last() != Some(')') {
                        self.add_char('0');
                    }
                }

                if ui.button("Ret").clicked() {
                    self.input.pop();
                }

                if ui.button("=").clicked() {
                    self.input = match calculate(&self.input) {
                        Ok(result) => format!("{}", result),
                        Err(error) => error
                    };
                    self.clear = true;
                }
            });
        });
    }
}