use std::{
    io::{self, Write},
    ops::Neg,
    str,
};

#[cfg(test)]
mod test;

enum CalculatorError {} 

fn is_str_digit(string: &str) -> bool {
    string.chars().any(|c| c.is_ascii_digit())
}

fn is_op_or_paren(string: &str) -> bool {
    let op_paren = "+-*/^()NEG";
    op_paren.contains(string)
}

fn is_unary_neg(tokens: &[String], buffer: &str) -> bool {
    if buffer != "-" {
        return false;
    }

    let latest_token = match tokens.last() {
        Some(val) => val,
        None => return true,
    };

    if latest_token == ")" {
        return false;
    }

    if latest_token == "NEG" {
        return true;
    }

    return is_op_or_paren(tokens.last().unwrap());
}

fn should_push_buffer(tokens: &[String], buffer: &str) -> bool {
    if is_unary_neg(tokens, buffer) {
        return false;
    }

    return !buffer.is_empty();
}

fn tokenize(problem: &str) -> Vec<String> {
    let chars = problem.chars();
    let mut tokens: Vec<String> = Vec::new();
    let mut buffer = String::new();

    for char in chars {
        match char {
            '0'..='9' => {
                if should_push_buffer(&tokens, &buffer) && !is_str_digit(&buffer) {
                    tokens.push(buffer.clone());
                    buffer.clear();
                }

                buffer.push(char);
            }
            '+' | '-' | '*' | '/' | '^' | '(' | ')' => {
                if char == '(' && (is_str_digit(&buffer) || buffer == ")") {
                    tokens.push(buffer.clone());
                    buffer.clear();
                    buffer.push('*');
                }

                if should_push_buffer(&tokens, &buffer) {
                    tokens.push(buffer.clone());
                    buffer.clear();
                }

                if (char == '(' || char == '-') && buffer == "-" {
                    tokens.push("NEG".to_string());
                    buffer.clear();
                }

                buffer.push(char);
            }
            '.' => {
                buffer.push(char);
            }
            _ => {}
        }
        // dbg!(&tokens);
        // dbg!(&buffer);
        // dbg!(&char);
    }

    // Push a remaining buffer
    if !buffer.is_empty() {
        tokens.push(buffer);
    }

    tokens
}

fn get_input(query: &str) -> String {
    print!("{query}");
    let _ = io::stdout().flush();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input.");

    let input = input.trim().to_string();

    input
}

fn to_postfix(tokens: Vec<String>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    let mut op_stack: Vec<String> = Vec::new();

    for token in tokens {
        if !is_str_digit(&token) {
            let mut op = op_stack.pop().unwrap_or("null".to_string());

            while compare_precedence(&token, &op) {
                output.push(op);

                op = op_stack.pop().unwrap_or("null".to_string());
            }

            if &op != "null" {
                op_stack.push(op);
            }

            if &token == ")" {
                while let Some(op) = op_stack.pop() {
                    ////dbg!(&op);
                    if op == "(" {
                        break;
                    }

                    output.push(op);
                }
            } else {
                op_stack.push(token);
            }
        } else {
            output.push(token);
        }
    }

    // Push remainings operator
    output.extend(op_stack.drain(..).rev());

    output
}

fn compare_precedence(op: &str, other_op: &str) -> bool {
    if !is_op_or_paren(op) || !is_op_or_paren(other_op) {
        return false;
    }

    if op == "(" || op == ")" {
        return false;
    }

    if op == "^" && other_op == "^" {
        return false;
    }

    precedence(op) == precedence(other_op) || precedence(op) < precedence(other_op)
}

fn precedence(op: &str) -> u8 {
    match op {
        "+" | "-" => 1,
        "*" | "/" => 2,
        "^" => 3,
        "NEG" => 4,
        _ => 0,
    }
}

fn apply_operator(lhs: f64, rhs: f64, op: String) -> f64 {
    match op.as_str() {
        "+" => lhs + rhs,
        "-" => lhs - rhs,
        "*" => lhs * rhs,
        "/" => lhs / rhs,
        "^" => lhs.powf(rhs),
        _ => 0.0,
    }
}

fn evaluate(expr: &str) -> Result<f64, String> {
    if expr.is_empty() {
        return Err("Empty expression".to_string());
    }

    dbg!(&expr);
    let tokens = tokenize(expr);
    dbg!(&tokens);
    let postfix = to_postfix(tokens);
    let mut result: Vec<f64> = Vec::new();
    dbg!(&postfix);

    for unit in postfix {
        if &unit == "NEG" {
            let num = result.pop().unwrap().neg();
            result.push(num);
        } else if !is_str_digit(&unit) {
            let rhs = result.pop().unwrap();
            let lhs = result.pop().unwrap();
            let res = apply_operator(lhs, rhs, unit);

            result.push(res);
        } else {
            let parse = match unit.parse::<f64>() {
                Ok(val) => val,
                Err(err) => return Err(err.to_string()),
            };
            result.push(parse);
        }
    }

    if result.is_empty() {
        return Err("Invalid infix".to_string());
    }

    Ok(result[0])
}

fn main() -> Result<(), String> {
    loop {
        let input = get_input("sybau-rs> ").to_lowercase();

        if input == "q" || input == "quit" {
            return Ok(());
        }

        let res = evaluate(&input);

        match res {
            Ok(res) => println!("Result: {res}"),
            Err(err) => println!("Error: {err}"),
        }
    }

    // let problem1 = "2+4";
    // let problem2 = "3*4+2";
    // let problem3 = "(4+3)*5-1";
    // let problem4 = "3*4+2*6/2+2*9/2";
}
