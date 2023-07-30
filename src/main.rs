use std::io;

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn main() {
    let mut user_input: String;
    let mut operation_instance: Operation;
    let mut user_input_num1: f64;
    let mut user_input_num2: f64;
    let mut user_input_oper: u8;

    loop {
        user_input = String::new();
        println!("First Number?");
        io::stdin()
            .read_line(&mut user_input)
            .expect("Did not enter correct value");
        user_input_num1 = user_input.trim().parse().unwrap();

        user_input = String::new();
        println!("Second Number?");
        io::stdin()
            .read_line(&mut user_input)
            .expect("Did not enter correct value");
        user_input_num2 = user_input.trim().parse().unwrap();

        user_input = String::new();
        println!("Operator? 1-Add 2-Substract 3-Multiply 4-Divide");
        io::stdin()
            .read_line(&mut user_input)
            .expect("Did not enter correct value");
        user_input_oper = user_input.trim().parse().unwrap();

        match user_input_oper {
            1 => operation_instance = Operation::Add(user_input_num1, user_input_num2),
            2 => operation_instance = Operation::Subtract(user_input_num1, user_input_num2),
            3 => operation_instance = Operation::Multiply(user_input_num1, user_input_num2),
            4 => operation_instance = Operation::Divide(user_input_num1, user_input_num2),
            _ => {
                println!("Unvalid operation");
                println!("----------------------------------------------------------");
                continue;
            }
        }

        let result = calculate(operation_instance);
        println!("Result = {}", result);
        println!("----------------------------------------------------------");
    }
}

fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => a / b,
    }
}
