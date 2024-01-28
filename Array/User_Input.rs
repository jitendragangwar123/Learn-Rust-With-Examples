//Example:1
// taking user input as a string
use std::io::{self, Write};
fn main() {
    let mut s = String::new();

    print!("Please enter your name: ");
    io::stdout().flush().unwrap(); // Flush stdout to display the print message immediately

    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read line"); // Read user input

    println!("Your name is: {}", s.trim());
}

//Example:2
// taking user input as a number
use std::io;
fn main() {
    let mut a = String::new();

    io::stdin().read_line(&mut a).unwrap(); 
    let a: i32 = a.trim().parse().unwrap();
    println!("Your number is: {}", a); 
}

//Example:3
// trim the string input
use std::io;
fn main() {
    let mut x = String::new();

    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line");

    x = x.trim().to_string();  // to remove any trailing newline
    println!("Hello {}", x);
}
