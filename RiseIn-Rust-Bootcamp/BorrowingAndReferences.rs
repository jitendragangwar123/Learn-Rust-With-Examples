/*
Borrowing And References:
    - Borrowing allows you to temporarily access a value without taking ownership of it.
    - When you borrow a value, you create a reference to it, which allows you to read or modify the value without becoming the owner.
*/

/*
There are two types of references in Rust: mutable references and immutable references.
Immutable References:
    - Immutable references are references that allow you to read a value, but not modify it.
    - Create an immutable reference by using the & operator followed by the variable name.
    - Immutable references that you can have multiple immutable references to the same value at the same time.
    - You can't have a mutable reference and an immutable reference to the same value at the same time.
*/

let my_string = String::from("hello, world!");
let my_ref = &my_string;
fn print_string(s: &String) {
    println!("{}", s);
}
let my_string = String::from("hello, world!");
print_string(&my_string);

/*
Mutable References:
    - A mutable reference is a reference to a variable that allows you to modify the value.
*/

fn main() {
    let mut my_string = String::from("hello");
    change_string(&mut my_string);
    println!("{}", my_string); // prints "hello world"
}
fn change_string(s: &mut String) {
    s.push_str(" world");
}

/*
- You can only have one mutable reference to a variable at a time.
- This is to prevent data races and other types of undefined behavior that can occur when multiple threads try to modify the same variable at the same time.
- If you try to create a second mutable reference to the same variable before the first one goes out of scope, you'll get a compile-time error.
- References automatically expire at the end of their scope.
*/

fn main() {
    let mut my_string = String::from("hello");
    let r1 = &my_string; // immutable reference
    let r2 = &my_string; // another immutable reference
    println!("{}, {}", r1, r2); // prints "hello, hello"
    let r3 = &mut my_string; // mutable reference
    r3.push_str(" world");
    println!("{}", r3); // prints "hello world"
}

/*
Dangling References:
    A dangling reference is a reference that points to a memory location that has been deallocated, causing unexpected behavior or a runtime error.
*/
