use std::{
    error::Error,
    fmt,
    io::{self, Write},
    ops::Neg,
    str,
};

#[cfg(test)]
mod test;

#[derive(Debug, PartialEq)]
enum CalculatorError {
    InvalidExpression,
    InvalidNumber(String),
    EmptyExpression,
    UnmatchedParentheses,
}

impl fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalculatorError::InvalidExpression => write!(f, "invalid expression"),
            CalculatorError::InvalidNumber(s) => write!(f, "invalid number: ({s})"),
            CalculatorError::EmptyExpression => write!(f, "empty expression"),
            CalculatorError::UnmatchedParentheses => write!(f, "unmatched parentheses"),
        }
    }
}

impl Error for CalculatorError {}

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

    is_op_or_paren(tokens.last().unwrap())
}

fn should_push_buffer(tokens: &[String], buffer: &str) -> bool {
    if is_unary_neg(tokens, buffer) {
        return false;
    }

    !buffer.is_empty()
}

fn tokenize(problem: &str) -> Vec<String> {
    let chars = problem.chars();
    let mut tokens: Vec<String> = Vec::new();
    let mut buffer = String::new();

    for char in chars {
        match char {
            '0'..='9' => {
                // Push and clear buffer if buffer is not empty (contains operators)
                // and not a unary neg.
                // Otherwise, if the buffer is empty or the last token is op or paren and buffer is a `-`. Skip this.
                if should_push_buffer(&tokens, &buffer) && !is_str_digit(&buffer) {
                    tokens.push(buffer.clone());
                    buffer.clear();
                }

                buffer.push(char);
            }
            '+' | '-' | '*' | '/' | '^' | '(' | ')' => {
                // 5(2+2) -> 5 * (2+2)
                if char == '(' && (is_str_digit(&buffer) || buffer == ")") {
                    tokens.push(buffer.clone());
                    buffer.clear();
                    buffer.push('*');
                }

                // Same
                if should_push_buffer(&tokens, &buffer) {
                    tokens.push(buffer.clone());
                    buffer.clear();
                }

                // Change buffer `-` to NEG
                // -(2+2) -> NEG(2+2) || --4 -> NEG(-4)
                if (char == '(' || char == '-') && buffer == "-" {
                    tokens.push("NEG".to_string());
                    buffer.clear();
                }

                buffer.push(char);
            }
            // For float number
            '.' => {
                buffer.push(char);
            }
            _ => {}
        }
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

    input.trim().to_string()
}

fn to_postfix(tokens: Vec<String>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    let mut op_stack: Vec<String> = Vec::new();

    for token in tokens {
        // Push the number to the output
        if is_str_digit(&token) {
            output.push(token);
            continue;
        }

        // Push the op from op_stack to the output if the current token has lower or equal precedence
        while let Some(op) = op_stack.pop() {
            if !is_lt_eq_precedence(&token, &op) {
                // Push back op
                op_stack.push(op);
                break;
            }

            output.push(op);
        }

        // Push the op from op_stack to the output until it found "(".
        if &token == ")" {
            while let Some(op) = op_stack.pop() {
                if op == "(" {
                    break;
                }

                output.push(op);
            }

            continue;
        }

        op_stack.push(token);
    }

    // Push remainings operator
    output.extend(op_stack.drain(..).rev());

    output
}

fn is_lt_eq_precedence(op: &str, other_op: &str) -> bool {
    if !is_op_or_paren(op) || !is_op_or_paren(other_op) {
        return false;
    }

    // ignore `(` or `)`. Only for op
    if op == "(" || op == ")" {
        return false;
    }

    // ignore `NEG`
    if op == "NEG" && other_op == "NEG" {
        return false;
    }

    // ignore `^`
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

fn evaluate(expr: &str) -> Result<f64, CalculatorError> {
    if expr.is_empty() {
        return Err(CalculatorError::EmptyExpression);
    }

    dbg!(&expr);
    let tokens = tokenize(expr);
    dbg!(&tokens);
    let postfix = to_postfix(tokens);
    let mut result: Vec<f64> = Vec::new();
    dbg!(&postfix);

    if postfix.iter().any(|t| t == "(") {
        return Err(CalculatorError::UnmatchedParentheses);
    }

    for unit in postfix {
        if unit == "NEG" {
            let num = result.pop().unwrap().neg();
            result.push(num);
            continue;
        }

        if !is_str_digit(&unit) {
            let rhs = result.pop().unwrap();
            let lhs = result.pop().unwrap();
            let res = apply_operator(lhs, rhs, unit);

            result.push(res);
            continue;
        }

        let val = unit
            .parse::<f64>()
            .map_err(|_| CalculatorError::InvalidNumber(unit))?;
        result.push(val);
    }

    if result.is_empty() {
        return Err(CalculatorError::InvalidExpression);
    }

    Ok(result[0])
}

fn main() {
    loop {
        let input = get_input("sybau-rs> ").to_lowercase();

        if input == "q" || input == "quit" {
            return;
        }

        let res = evaluate(&input);

        match res {
            Ok(res) => println!("Result: {res}"),
            Err(err) => println!("Error: {err}"),
        }
    }
}
