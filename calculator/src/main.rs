use std::io;
use std::f64::consts::PI;

fn main() {
    println!("Advanced Calculator");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        if input.trim() == "exit" {
            println!("Goodbye!");
            break;
        }

        match evaluate_expression(input.trim()) {
            Ok(result) => println!("Result: {}", result),
            Err(err) => println!("Error: {}", err),
        }
    }
}

fn evaluate_expression(expression: &str) -> Result<f64, String> {
    // Tokenize and evaluate the expression
    let tokens: Vec<&str> = expression.split_whitespace().collect();
    let mut stack: Vec<f64> = Vec::new();
    let mut operators: Vec<&str> = Vec::new();

    for token in tokens {
        if let Ok(num) = token.parse::<f64>() {
            stack.push(num);
        } else {
            match token {
                "+" | "-" | "*" | "/" => {
                    while !operators.is_empty()
                        && precedence(operators.last().unwrap()) >= precedence(token)
                    {
                        apply_operator(&mut stack, operators.pop().unwrap());
                    }
                    operators.push(token);
                }
                "(" => operators.push(token),
                ")" => {
                    while let Some(op) = operators.pop() {
                        if op == "(" {
                            break;
                        }
                        apply_operator(&mut stack, op);
                    }
                }
                "sqrt" => {
                    if let Some(val) = stack.pop() {
                        stack.push(val.sqrt());
                    } else {
                        return Err("Invalid expression".to_string());
                    }
                }
                "sin" => {
                    if let Some(val) = stack.pop() {
                        stack.push(val.sin());
                    } else {
                        return Err("Invalid expression".to_string());
                    }
                }
                "cos" => {
                    if let Some(val) = stack.pop() {
                        stack.push(val.cos());
                    } else {
                        return Err("Invalid expression".to_string());
                    }
                }
                _ => return Err("Invalid expression".to_string()),
            }
        }
    }

    while let Some(op) = operators.pop() {
        apply_operator(&mut stack, op);
    }

    if let Some(result) = stack.pop() {
        if stack.is_empty() {
            Ok(result)
        } else {
            Err("Invalid expression".to_string())
        }
    } else {
        Err("Invalid expression".to_string())
    }
}

fn precedence(operator: &str) -> i32 {
    match operator {
        "+" | "-" => 1,
        "*" | "/" => 2,
        _ => 0,
    }
}

fn apply_operator(stack: &mut Vec<f64>, operator: &str) {
    if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
        match operator {
            "+" => stack.push(a + b),
            "-" => stack.push(a - b),
            "*" => stack.push(a * b),
            "/" => stack.push(a / b),
            _ => panic!("Invalid operator"),
        }
    }
}