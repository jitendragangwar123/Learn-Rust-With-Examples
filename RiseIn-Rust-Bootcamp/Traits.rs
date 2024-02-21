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

/*
Defining Traits:
    - To define a trait, you use the trait keyword followed by the name of the trait and a block containing the method signatures.
*/
trait Speak {
    fn speak(&self);
}

/*
Implementing Traits:
    - To implement a trait for a specific type, you use the impl keyword followed by the trait name, the for keyword, 
      and the name of the type you want to implement the trait for.
    - You provide the method implementations in a block.
*/

trait Speak {
    fn speak(&self);
}

struct Dog {
    name: String,
}

impl Speak for Dog {
    fn speak(&self) {
        println!("{} says: Woof!", self.name);
    }
}

fn main(){
    let d = Dog{name:String::from("Tom")}; 
    d.speak();
}
