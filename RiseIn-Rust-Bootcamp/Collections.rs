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
