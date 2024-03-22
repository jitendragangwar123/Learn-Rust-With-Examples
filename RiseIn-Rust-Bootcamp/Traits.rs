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
    - To implement a trait for a specific type, you use the impl keyword followed by the trait name, the for a keyword, 
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

/*
Implementing Traits for Custom Types (Structs, Enums):
*/
trait Display {
    fn display(&self) -> String;
}
struct Circle {
    radius: f64,
}
struct Rectangle {
    width: f64,
    height: f64,
}
impl Display for Circle {
    fn display(&self) -> String {
        format!("Circle with radius: {}", self.radius)
    }
}
impl Display for Rectangle {
    fn display(&self) -> String {
        format!("Rectangle with width: {} and height: {}", self.width, self.height)
    }
}
fn main(){
    let c = Circle{radius:20.0}; 
    let r = Rectangle{width:4.0,height:5.0};
    println!("{}\n{}",c.display(),r.display());
}

/*
Implementing Traits for Existing Types:
    - You can only implement a trait for a type if either the trait or the type is defined within your own crate.
*/
trait Display {
    fn display(&self) -> String;
}
impl Display for String {
    fn display(&self) -> String {
        self.clone()
    }
}
fn display_something<T: Display>(item: &T) {
    println!("{}", item.display());
}
fn main() {
    let my_string = String::from("Hello, world!");
    display_something(&my_string);
}
