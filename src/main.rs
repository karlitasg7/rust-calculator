use regex::Regex;

fn math_operation(expression: Regex, mut operation: String, operator: &str) -> String {
    loop {
        let caps = expression.captures(operation.as_str());

        if caps.is_none() {
            break;
        }

        let caps = caps.unwrap();

        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        let result = match operator {
            "+" => left_value + right_value,
            "-" => left_value - right_value,
            "*" => left_value * right_value,
            "/" => left_value / right_value,
            _ => 0,
        };

        operation = operation.replace(cap_expression, &result.to_string());
    }
    operation
}

fn main() {
    println!("Welcome!");

    // regex
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let re_mul = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();
    let re_sub = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();
    let re_div = Regex::new(r"(\d+)\s?\\/\s?(\d+)").unwrap();

    // get input from user
    println!("Please enter your expresion: ");
    let mut expression = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();

    expression = math_operation(re_div, expression, "/");
    expression = math_operation(re_mul, expression, "*");
    expression = math_operation(re_add, expression, "+");
    expression = math_operation(re_sub, expression, "-");

    // show result
    println!("Result: {}", expression);
}
