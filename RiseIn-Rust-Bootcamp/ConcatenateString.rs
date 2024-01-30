use std::io::{self, Write};

// Function to concatenate two string slices and return a new String
fn concatenate_strings(str1: &str, str2: &str) -> String {
    // Create a new String
    let mut result = String::new();

    // Append the contents of the first string slice
    result.push_str(str1);

    // Append the contents of the second string slice
    result.push_str(str2);

    // Return the concatenated result
    result
}

fn main() {
    // Prompt the user for input
    println!("Enter the first string:");
    io::stdout().flush().unwrap();
    let mut string1 = String::new();
    io::stdin().read_line(&mut string1).expect("Failed to read line");

    println!("Enter the second string:");
    io::stdout().flush().unwrap();
    let mut string2 = String::new();
    io::stdin().read_line(&mut string2).expect("Failed to read line");

    // Call the concatenate_strings function with references to user-input strings
    let concatenated_string = concatenate_strings(string1.trim(), string2.trim());

    // Print the concatenated result
    println!("Concatenated String: {}", concatenated_string);
}
