/*
Traits: 
  - Traits are a powerful feature in Rust that allows you to define shared behavior for multiple types.
  - Traits in Rust are similar to interfaces in languages like Java or C#.
*/

/*
Basics of Generics:
  - Generics are a powerful feature that allows you to write flexible and reusable code by defining functions, 
    structs, enums, and even traits that work with multiple types, without having to duplicate the code for each type.
*/

fn identity<T>(value: T) -> T {
    value
}
fn main(){
    let x = identity(42); // works with integers
    let y = identity("hello"); // works with strings
    println!("{} {}",x,y);
}
