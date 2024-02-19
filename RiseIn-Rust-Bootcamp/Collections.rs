/*
Collections: 
    - In Rust, collections are powerful and versatile tools that enable you to store multiple data elements in a single structure.
    - 
*/
/*
Vectors: 
    - Vectors are resizable arrays that store elements of the same data type.
    - To create a vector, you can use the vec! macro or the Vec::new() function.
*/

// Using the vec! macro
let mut numbers = vec![1, 2, 3, 4];
// Using the Vec::new() function
let mut names: Vec<String> = Vec::new();

/*
Updating and Accessing Vector Elements:
*/
fn main(){
    let mut names: Vec<String> = Vec::new();
    names.push(String::from("Alice"));
    names.push(String::from("Bob"));
    let first_name = &names[0]; 
    let second_name = &names[1];
    println!("The first name is: {} and {}", first_name, second_name); 
}

/*
Iterating Through a Vector:
*/
fn main(){
    let mut names: Vec<String> = Vec::new();
    names.push(String::from("Alice"));
    names.push(String::from("Bob"));
    for name in &names {
        println!("Name: {}", name);
    }
}

/*
Slicing a Vector:
*/
/*
Example:1
*/
fn main(){
    let mut names: Vec<String> = Vec::new();
    names.push(String::from("Alice"));
    names.push(String::from("Bob"));
    let slice=&names[0..2];
    println!("Name: {:?}",slice);
}
/*
Example:2
*/
fn main(){
    let numbers = vec![1, 2, 3, 4, 5];
    let slice = &numbers[0..4];
    println!("Number: {:?}",slice);
}

/*
Strings:Strings are a collection of characters, and in Rust, they are implemented as a wrapper over a Vec<u8> to store UTF-8 encoded text.
        There are two main types of strings:
        1. String: Which is a growable, mutable, and heap-allocated data structure.
        2. &str: Which is an immutable, borrowed reference to a string slice.
*/

/*
Appending to a String:
            - push_str : method for adding a string slice.
            - push : method for adding a single character.
*/
fn main(){
    let mut hello = String::from("Hello, ");
    hello.push_str("world!");
    hello.push('!');
    println!("{}", hello);
}

/*
String Slicing and Indexing:
            - Strings are encoded in UTF-8.
            - Indexing them by bytes may not always correspond to a valid Unicode scalar value.
            - you can use slicing to access parts of a string.
*/
fn main(){
    let example = String::from("hello");
    let slice = &example[0..3];
    println!("{}", slice); // "hel"
}

/*
UTF-8 Encoding and Handling Unicode Scalar Values:
    - Strings in Rust are stored as a sequence of bytes representing UTF-8 encoded text.
    - This means that some characters, such as those from non-Latin scripts or special symbols, may occupy more than one byte.
    - You can use the chars method to iterate over Unicode scalar values.
*/
fn main(){
    let text = "こんにちは";
    for c in text.chars() {
        println!("{}", c);
    }
}

/*
Iterating Through a String:
    - You can use the chars method to iterate through the characters of a string.
    - The bytes method to iterate through the individual bytes.
*/
fn main(){
    let example = String::from("hello");

    // Iterate over characters
    for c in example.chars() {
        println!("{}", c);
    }
    
    // Iterate over bytes
    for b in example.bytes() {
        println!("{}", b);
    }
}
/*
Output:
h
e
l
l
o
h
q
104 ---> ASCII Value
101
108
108
111
104
113
*/

/*
Hash Maps:
    - Which allow you to store data in key-value pairs.
    - A hash map is a collection that associates a key with a value.
    - To create a hash map, you can use the HashMap type from the std::collections module.
*/

/*
Creating a Hash Map:
*/
use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Alice"), 10);
    scores.insert(String::from("Bob"), 20);
    println!("Alice's Score: {}",scores["Alice"]);
    println!("Score List: {:?}",scores);
}

/*
Accessing and Updating Hash Map Elements:
*/
use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Alice"), 10);
    scores.insert(String::from("Bob"), 20);
    //Access the hashmap elemets
    let alice_score = scores.get(&String::from("Alice"));
    println!("Alice's score: {:?}", alice_score);
    // Update the hashmap elements
    scores.insert(String::from("Alice"),30);
    println!("Alice's updated score: {:?}",scores);
}

/*
Removing Elements from a Hash Map:
*/
use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Alice"), 10);
    scores.insert(String::from("Bob"), 20);
    // remove hashmap elements
    scores.remove(&String::from("Alice"));
    println!("Updated score: {:?}",scores);
}

/*
Iterating Through a Hash Map:
*/
use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Alice"), 10);
    scores.insert(String::from("Bob"), 20);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
