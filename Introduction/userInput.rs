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
