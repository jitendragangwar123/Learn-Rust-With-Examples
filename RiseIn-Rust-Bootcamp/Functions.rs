// Function:A set of instructions that you can give to the computer to perform a particular task.

// Syntax:
fn function_name(parameter1: type1, parameter2: type2) -> return_type {
    // Function body
    return value;
}

// Function Arguments and Return Types
fn get_greeting() -> String {
    return String::from("Hello, Rust!");
}
println!("{}",get_greeting());


// Passing Parameters to a Function
fn add_numbers(x: i32, y: i32) -> i32 {
    let result = x + y;
    return result;
}
//or 
fn add_numbers(x: i32, y: i32) -> i32 {
    x + y
}
let sum = add_numbers(3, 5);
println!("{}",sum);


// Default Parameters
fn greet(name: Option<&str>) {
    match name {
        Some(n) => println!("Hello, {}!", n),
        None => println!("Hello, Rust!"),
    }
}
println!("{:?}",greet(None));
println!("{:?}",greet(Some("Jay")));


// Returning Values from a Function
fn get_greeting(name: &str) -> String {
    let greeting = format!("Hello, {}!", name);
    return greeting;
}
println!("{}",get_greeting("Jeetu"));
