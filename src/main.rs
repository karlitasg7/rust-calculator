use regex::Regex;

fn main() {
    println!("Welcome!");

    // regex
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let re_mul = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();

    // get input from user
    println!("Please enter your expresion: ");
    let mut expression = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();

    // multiplication
    loop {
        // make operations
        let caps = re_mul.captures(expression.as_str());

        if caps.is_none() {
            break;
        }

        let caps = caps.unwrap();

        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        let multiply = left_value * right_value;

        expression = expression.replace(cap_expression, &multiply.to_string());
    }

    // add
    loop {
        // make operations
        let caps = re_add.captures(expression.as_str());

        if caps.is_none() {
            break;
        }

        let caps = caps.unwrap();

        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        let addition = left_value + right_value;

        expression = expression.replace(cap_expression, &addition.to_string());
    }

    // show result
    println!("Result: {}", expression);
}
