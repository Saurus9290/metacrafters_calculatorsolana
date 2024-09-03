use std::io;

pub fn adder(x: i32, y: i32) -> i32 {
    x + y
}

pub fn subber(x: i32, y: i32) -> i32 {
    x - y
}

pub fn multer(x: i32, y: i32) -> i32 {
    x * y
}

pub fn divider(x: i32, y: i32) -> Result<i32, String> {
    if y == 0 {
        Err(String::from("Error: Division by zero"))
    } else {
        Ok(x / y)
    }
}

fn main() {
    println!("Welcome to the calculator!");
    println!("Enter a binary expression without spaces (e.g., 3+2, 4*5): ");

    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("Failed to read line");

    // Trim and validate input length
    let inp = inp.trim();
    if inp.len() != 3 {
        println!("Error: Invalid input length. Please enter a binary expression in the form 'a+b'.");
        return;
    }

    let my_vec: Vec<char> = inp.chars().collect();

    // Validate characters and parse numbers
    let a = match my_vec[0].to_digit(10) {
        Some(num) => num as i32,
        None => {
            println!("Error: Invalid first operand. Please enter a number.");
            return;
        }
    };

    let b = match my_vec[2].to_digit(10) {
        Some(num) => num as i32,
        None => {
            println!("Error: Invalid second operand. Please enter a number.");
            return;
        }
    };

    let operator = my_vec[1];

    // Perform calculation based on the operator
    let result = match operator {
        '+' => Ok(adder(a, b)),
        '-' => Ok(subber(a, b)),
        '*' => Ok(multer(a, b)),
        '/' => divider(a, b),
        _ => Err(String::from("Error: Invalid operator. Please use one of +, -, *, /.")),
    };

    // Print the result
    match result {
        Ok(value) => println!("The result of the expression is: {}", value),
        Err(e) => println!("{}", e),
    }
}
