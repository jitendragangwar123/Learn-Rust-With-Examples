/*
Rust:
  1. Rust is a modern systems programming language focusing on safety, speed, and concurrency. 
  2. It accomplishes these goals by being memory safe without using garbage collection.
*/

/*
Note: `println!` is a macro that prints text to the console.
*/

//helloWorld.rs
fn main() {
    println!("Hello, world!");
    
    let x: i32 = 42;
    let pi: f64 = 3.14159;
    let is_rust_fun: bool = true;
    let letter_a: char = 'a';
    //function
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }
    //loops
    let x = 42;

    if x >= 0 {
        println!("x is non-negative");
    } else {
        println!("x is negative");
    }

    let mut i = 1;

    while i <= 5 {
        println!("{}", i);
        i += 1;
    }
}

