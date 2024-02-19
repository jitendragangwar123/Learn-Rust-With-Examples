use std::io;
enum Operations{
    Add(f64,f64),
    Subtract(f64,f64),
    Multiply(f64,f64),
    Divide(f64,f64),
}

fn calculate(op:Operations) -> f64{
    match op{
        Operations::Add(x,y) => x+y,
        Operations::Subtract(x,y) => x-y,
        Operations::Multiply(x,y) => x*y,
        Operations::Divide(x,y) => x/y,
    }
}

fn main(){
    println!("Enter the first number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let first_number: f64 = input.trim().parse().expect("Invalid input");

    println!("Enter the operation (Add, Subtract, Multiply, Divide):");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let operation: String = input.trim().to_lowercase();

    println!("Enter the second number:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let second_number: f64 = input.trim().parse().expect("Invalid input");

    let operation_enum = match operation.as_str() {
        "add" => Operations::Add(first_number, second_number),
        "subtract" => Operations::Subtract(first_number, second_number),
        "multiply" => Operations::Multiply(first_number, second_number),
        "divide" => Operations::Divide(first_number, second_number),
        _ => {
            println!("Invalid operation!");
            return;
        }
    };

    let result = calculate(operation_enum);
    println!("Result: {}", result);
}
