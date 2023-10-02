use num_bigfloat::BigFloat;
use pipe_trait::*;
use std::io::stdin;

struct UserInput {
    contents: String,
}

impl UserInput {
    fn new(contents: String) -> UserInput {
        UserInput {
            contents: contents.clone(),
        }
    }

    fn try_new(contents: String) -> Option<UserInput> {
        contents
            .trim_end_matches("\r\n")
            .to_string()
            .pipe(|val| match val.chars().count() {
                0 => None,
                _ => Some(val),
            })
            .map(UserInput::new)
    }
}

fn get_valid_operators() -> Vec<String> {
    ["+", "-", "*", "/", "(", ")"]
        .iter()
        .map(|op| op.to_string())
        .collect::<Vec<String>>()
}

fn get_valid_operands() -> Vec<String> {
    ('0'..='9')
        .chain(['.'])
        .map(|ch| ch.to_string())
        .collect::<Vec<String>>()
}

fn tokenize(input: String) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut i: usize = 0;
    let chars: Vec<String> = input
        .chars()
        .map(|ch| ch.to_string())
        .collect::<Vec<String>>();
    while i < chars.len() {
        if i == 0 && chars[i] == "-" {
            tokens.push("!".to_string());
            i += 1;
        } else if i > 0 && chars[i] == "-" && get_valid_operators().contains(tokens.last().unwrap())
        {
            tokens.push("!".to_string());
            i += 1;
        } else if get_valid_operands().contains(&chars[i]) {
            let mut j: usize = i + 1;
            let mut number_string: String = chars[i].clone();
            while j < chars.len() {
                if get_valid_operands().contains(&chars[j]) {
                    number_string.push_str(&chars[j]);
                    j += 1;
                    continue;
                }
                break;
            }
            if number_string.chars().count() > 0 {
                tokens.push(number_string);
            }
            i = j;
        } else if get_valid_operators().contains(&chars[i]) {
            tokens.push(chars[i].clone());
            i += 1;
        } else {
            i += 1;
        }
    }

    tokens
}

fn negate_tokens(tokens: Vec<String>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    let mut i: usize = 0;
    while i < tokens.len() {
        if tokens[i] == "!" && i < tokens.len() {
            output.push("-".to_string() + &tokens[i + 1]);
            i += 2;
        } else {
            output.push(tokens[i].clone());
            i += 1;
        }
    }
    output
}

fn get_precedence(operator: &str, from_stack: bool) -> i32 {
    match from_stack {
        true => match operator {
            "(" => 0,
            ")" => -1,
            "+" | "-" => 2,
            "*" | "/" => 4,
            _ => -2,
        },
        false => match operator {
            "(" => 5,
            ")" => 0,
            "+" | "-" => 1,
            "*" | "/" => 3,
            _ => -2,
        },
    }
}

fn to_infix_notation(tokens: Vec<String>) -> Vec<String> {
    let mut stack: Vec<String> = Vec::from(["(".to_string()]);
    let mut output: Vec<String> = Vec::new();
    let mut tokens = tokens.clone();
    tokens.push(")".to_string());
    for token in tokens {
        if get_valid_operators().contains(&token) {
            let input_precedence = get_precedence(&token, false);
            loop {
                if stack.len() == 0 {
                    break;
                }
                let stack_precedence = get_precedence(stack.last().unwrap(), true);
                if stack_precedence >= input_precedence {
                    if stack.last().unwrap() == "(" {
                        stack.pop();
                    } else {
                        output.push(stack.pop().unwrap());
                    }
                } else {
                    break;
                }
            }
            stack.push(token);
        } else {
            output.push(token);
        }
    }
    output
}

fn solve_infix(infix_notation: Vec<String>) -> Option<BigFloat> {
    let mut stack: Vec<String> = Vec::new();
    for token in infix_notation {
        if get_valid_operators().contains(&token) {
            // TODO: popping an empty stack causes panic with input "+1" for expression.
            let val2: BigFloat = stack.pop().pipe(try_parse).unwrap();
            let val1: BigFloat = stack.pop().pipe(try_parse).unwrap();
            match token.as_str() {
                "+" => stack.push((val1 + val2).to_string()),
                "-" => stack.push((val1 - val2).to_string()),
                "*" => stack.push((val1 * val2).to_string()),
                "/" if val2 != 0.into() => stack.push((val1 / val2).to_string()),
                _ => continue,
            }
        } else {
            stack.push(token);
        }
    }

    if stack.len() > 0 {
        return stack.pop().pipe(try_parse);
    } else {
        return None;
    }
}

fn try_parse(input: Option<String>) -> Option<BigFloat> {
    match input {
        Some(user_string) => BigFloat::parse(&user_string),
        None => None,
    }
}

fn run_calculations(user_input: &UserInput) -> Option<BigFloat> {
    tokenize(user_input.contents.clone())
        .pipe(negate_tokens)
        .pipe(to_infix_notation)
        .pipe(solve_infix)
}
fn try_get_input() -> Option<UserInput> {
    String::new().pipe(|mut str| match stdin().read_line(&mut str) {
        Ok(_) => UserInput::try_new(str.clone()),
        Err(_) => None,
    })
}

fn should_exit(input: &UserInput) -> bool {
    input.contents == "exit".to_string()
}

fn require_input() {
    println!("No input was provided. Please try again.");
}

fn main() {
    loop {
        println!("Please enter an expression to calculate: ");

        match try_get_input() {
            Some(input) => match should_exit(&input) {
                true => break,
                false => match run_calculations(&input) {
                    Some(result) => match result.clone().to_f64() > f64::MAX {
                        true => println!("{} = {:.2}", &input.contents, result),
                        false => println!("{} = {:.2}", &input.contents, result.to_f64()),
                    },
                    None => println!("Input was invalid."),
                },
            },
            None => {
                require_input();
                continue;
            }
        }
    }
}