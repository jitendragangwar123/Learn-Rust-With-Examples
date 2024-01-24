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
