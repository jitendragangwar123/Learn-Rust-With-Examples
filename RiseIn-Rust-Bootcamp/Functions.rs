// Function:A set of instructions that you can give to the computer to perform a particular task.

// Syntax:
fn function_name(parameter1: type1, parameter2: type2) -> return_type {
    // Function body
    return value;
}

// define the type of parameters
fn another_fn(num:i32,letter:char){
    println!("The value of num is: {} and letter is : {}",num,letter);
}
fn main(){
    println!("Hello");
    another_fn(42,'A');
}

// statements and expressions
fn main(){
    let y={
        let x=3;
        x+1
    };
    println!("The value of y is :{}",y); // 4
}


// Function Arguments and Return Types
fn get_greeting() -> String {
    return String::from("Hello, Rust!");
}
fn main(){
    println!("{}",get_greeting());
}

// Return multiple values using tuples
fn sum(num1:i32,num2:i32) -> (i32,i32){
    (num1+num2,num1-num2)
}
fn main(){
    let x=sum(6,5);
    //println!("Sum is:{} and Diff is: {}",x.0,x.1);
    println!("The sum and diff is: {:?}",x);
}

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
