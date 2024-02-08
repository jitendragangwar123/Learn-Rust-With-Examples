/*
Structs, short for structures, are a way to group related data together in a custom data type.
*/

struct Book {
    title: String,
    author: String,
    publication_year: u32,
}

/*
Defining Structs:
*/

struct Point {
    x: f32,
    y: f32,
}

/*
Creating Instances of Structs:
*/
let point = Point { x: 1, y: 2 };

/*
Accessing Struct Fields: 
*/
struct Person {
    name: String,
    age: u32,
}
 
fn main() {
    let alice = Person {
        name: String::from("Alice"),
        age: 30,
    };
 
    // Accessing the 'name' and 'age' fields
    println!("Name: {}", alice.name);
    println!("Age: {}", alice.age);
}

/*
Example: You can also modify the values of the fields if the struct instance is mutable.
*/
struct Person {
    name: String,
    age: u32,
}
fn main() {
    let mut alice = Person {
        name: String::from("Alice"),
        age: 30,
    };
 
    // Modifying the 'age' field
    alice.age = 31;
    println!("Updated Age: {}", alice.age);
}


/*
*/
