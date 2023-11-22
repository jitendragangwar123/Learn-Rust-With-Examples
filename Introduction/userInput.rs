//Example:1
use std::io::{self, Write};
fn main() {
    let mut s = String::new();

    print!("Please enter your name: ");
    io::stdout().flush().unwrap(); // Flush stdout to display the print message immediately

    io::stdin().read_line(&mut s).expect("Failed to read line"); // Read user input

    println!("Your name is: {}", s.trim());
}

//Example:2
use std::io;
fn main() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap(); // replace  the __
    let a: i32 = a.trim().parse().unwrap();
    println!("Your number is: {}", a); // replace the __
}

//Example:3
use std::io;
fn main() {
    let mut input_a = String::new();
    let mut input_b = String::new();

    io::stdin().read_line(&mut input_a).unwrap();
    io::stdin().read_line(&mut input_b).unwrap();

    let a: i32 = input_a.trim().parse().unwrap();
    let b: i32 = input_b.trim().parse().unwrap();

    // complete the code  
    let sum=a+b;
    let diff=a-b;
    println!("Sum is: {}",sum);
    println!("Difference is: {}",diff);
}

//Example:4
use std::io;
fn main() {
    let mut a_str = String::new();
    io::stdin().read_line(&mut a_str).unwrap();
    let a: i32 = a_str.trim().parse().unwrap();
    
    let mut b_str = String::new();
    io::stdin().read_line(&mut b_str).unwrap();
    let b: i32 = b_str.trim().parse().unwrap();

    println!("{}{}{}", a , b , a + b);
}

//Example:5
use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut iter = input.split_whitespace();

    let a: i32 = iter.next().unwrap().parse().unwrap();
    let b: i32 = iter.next().unwrap().parse().unwrap();
    
    let c = a + 2;
    let d = c + b;

    println!("{}", d);
}
