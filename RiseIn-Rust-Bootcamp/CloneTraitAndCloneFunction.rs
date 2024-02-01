/*
The Clone Trait and Clone Function:
    - The Clone trait is a standard trait in Rust that provides the clone function, which creates a deep copy of a value.
    - When you call clone on a value, it returns a new value that is an independent instance of the original value but has the same data.
    - Both original_string and cloned_string have the same value, but they are separate instances, meaning that modifying one does not affect the other.
*/

let original_string = String::from("Hello, world!");
let cloned_string = original_string.clone(); // Creates a deep copy of the original_string

println!("original_string: {}", original_string);
println!("cloned_string: {}", cloned_string);

/*
Using Clone with Borrowing and References:
    - A function that takes an immutable reference to a String and you want to modify the String in the function.
    - You can use the clone function to create a copy of the String, modify the copy, and then return it.
*/

fn modify_string(s: &String) -> String {
    let mut cloned_string = s.clone(); // Creates a deep copy of the input string
    cloned_string.push_str(" modified");
    cloned_string
}
fn main() {
    let original_string = String::from("Hello, world!");
    let modified_string = modify_string(&original_string);

    println!("original_string: {}", original_string); // "Hello, world!"
    println!("modified_string: {}", modified_string); // "Hello, world! modified"
}
