use std::env::{args, Args};

fn main() {
    let mut args: Args = args();
    
    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();
    
    let first_n: f32 = first.parse().unwrap();
    let second_n = second.parse::<f32>().unwrap();
    
    let result = operate(operator, first_n, second_n);
    
    println!("{:?}", output(first_n, operator, second_n, result));
}

fn operate(operator: char, first_n: f32, second_n: f32) -> f32 {
    if operator == '+'{
        first_n + second_n
    }
    else if operator == '-'{
        first_n - second_n
    }
    else if operator == '*'{
        first_n * second_n
    }
    else if operator == '/'{
        first_n / second_n
    }
    else {
        0.0
    }
}

fn output(first_n: f32, operator: char, second_n: f32, result: f32)-> String{
    format!("{} {} {} = {}", first_n, operator, second_n, result)
}

fn operation(operator: char, first_n: f32, second_n: f32) -> f32 {
    match operator {
        '+' => first_n + second_n,
        '-' => first_n - second_n,
        '*' | 'x' | 'X' => first_n * second_n,
        '/' => first_n / second_n,
        _ => panic!("Invalid operator used!")
    }
}
