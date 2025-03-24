#[derive(Debug)]
enum Token {
    Number(f64),
    Operator(char),
}

fn parse_expression(expression: &str) -> Vec<Token> {
    let mut parsed_expression = Vec::new();
    let mut is_num = false;
    let mut num_start = 0;

    for (i, item) in expression.chars().enumerate() {
        if item.is_digit(10) || item == '.' {
            if !is_num {
                is_num = true;
                num_start = i;
            }
        } else {
            if is_num {
                let number: f64 = expression[num_start..i].parse().unwrap();
                parsed_expression.push(Token::Number(number));
                is_num = false;
            }

            if item == '-' && (i == 0 || expression.chars().nth(i - 1) == Some('(')) {
                is_num = true;
                num_start = i;
            } else if "+-x/%()".contains(item) {
                parsed_expression.push(Token::Operator(item));
            }
        }
    }

    if is_num {
        let number: f64 = expression[num_start..].parse().unwrap();
        parsed_expression.push(Token::Number(number));
    }

    parsed_expression
}

pub fn calculate(expression: &str) -> Result<f64, String> {

    let parsed_expression = parse_expression(expression);

    let mut queue = Vec::new();
    let mut stack = Vec::new();

    for token in parsed_expression {
        match token {
            Token::Number(value) => queue.push(value),
            Token::Operator(op) => {
                if op == ')' {
                    let mut found_opening = false;
                    while let Some(&last) = stack.last() {
                        if last == '(' {
                            found_opening = true;
                            stack.pop();
                            break;
                        }

                        apply_operator(&mut stack, &mut queue)?;
                    }
                    if !found_opening {
                        return Err("Mismatched parenthesis".into());
                    }
                } else if op == '(' {
                    stack.push(op);
                } else {
                    while let Some(&last) = stack.last() {
                        if last == '(' || precedence(op) > precedence(last) {
                            break;
                        }

                        apply_operator(&mut stack, &mut queue)?;
                    }
                    stack.push(op);
                }
            }
        }
    }
    
    while let Some(_) = stack.last() {
        apply_operator(&mut stack, &mut queue)?;
    }

    queue.pop().ok_or::<String>("Invalid expression".into())
}

fn precedence(op: char) -> u8 {
    match op {
        '+' | '-' => 1,
        'x' | '/' | '%' => 2,
        _ => 0
    }
}

fn apply_operator(stack: &mut Vec<char>, queue: &mut Vec<f64>) -> Result<(), String> {
    let y = queue.pop().ok_or("Invalid expression")?;
    let x = queue.pop().ok_or("Invalid expression")?;

    let op = stack.pop().ok_or("Invalid expression")?;

    let result: Result<f64, String>  = match op {
        '+' => Ok(x + y),
        '-' => Ok(x - y),
        'x' => Ok(x * y),
        '/' => {
            if y == 0.0 {
                return Err("Division by zero".into());
            }
            Ok(x / y)
        }
        '%' => {
            if y == 0.0 {
                return Err("Division by zero".into());
            }
            Ok(x % y)
        }
        _ => return Err("Mismatched parenthesis".into()),
    };

    let value = result?;

    if value.is_infinite() {
        return Err("Overflow occurred".into());
    }

    if value.is_nan() {
        return Err("Invalid operation resulted in NaN".into());
    }

    queue.push(value);
    
    Ok(())
}