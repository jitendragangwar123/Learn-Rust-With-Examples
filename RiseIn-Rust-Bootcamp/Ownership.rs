/*
Strings:
      Strings in Rust are dynamically allocated in memory, which means that they can grow or shrink as needed. 
*/
let my_string = String::from("hello, world!"); 

/*
- Rust also has a built-in string slice type, which is called &str.
- String slices are references to a portion of a String or a string literal, and they are often used when you want to manipulate a string without taking ownership of it. 
*/

let my_string = String::from("hello, world!"); 
let slice = &my_string[0..5]; 
println!("{}", slice); 


/*
Ownership:
    In Rust, when you create a value, you become its owner, and you are responsible for managing its memory.
    It follows a "move semantics" model.
    This means that when a value is assigned to a new variable or passed as an argument to a function, the ownership of the value is transferred to the new variable or function.
    This ensures that there is always exactly one owner for each value, which helps to prevent common memory-related bugs such as double-freeing or use-after-free errors. 
*/

//Example: Ownerships in Rust (Move, Clone, Copy)
fn main(){    
    // Copy (Stack-only data)
    // works for Integers,Boolean,Char,Floating-point numbers,Tuples 
    let s1="Hello"; 
    let s2=s1;
    println!("{}",s1);
    println!("{}",s2); 

    // Move : data stored in heap
    let s3=String::from("Hello"); 
    // Move : the s3 value moved into s4
     let s4=s3;
    //println!("{}",s3); // s3 invalidate 
    println!("{}",s4);

    // Clone : data stored in heap
    let s5=String::from("Hello"); // store in heap
    let s6=s5.clone(); 
    println!("{}",s5);
    println!("{}",s6);    
} 

/*
Stack And Heap:
      In Rust, memory is divided into two main categories: the stack and the heap.
      The stack is a region of memory that is used for static memory allocation, which means that the size of the memory is determined at compile-time.
      The heap, on the other hand, is a region of memory that is used for dynamic memory allocation, which means that the size of the memory can change at runtime.
*/

let x = 5; // stored on the stack 
let y = String::from("hello"); // stored on the heap 
println!("x = {}, y = {}", x, y); 

/*
- If the variable needs to be stored on the heap, Rust allocates memory for it on the heap and creates a reference to the memory on the stack. 
- In Rust, each variable has a specific owner, which is responsible for allocating and deallocating memory for the variable.
- When the program exits the main function, Rust automatically deallocates the memory that was allocated for x, y, and z.
- Ownership is an important concept in Rust because it helps to ensure memory safety and prevent common bugs such as null pointer errors, dangling pointers, and memory leaks.
*/

let x = 5; // stored on the stack 
let y = String::from("hello"); // stored on the heap 
let z = y; // ownership of y is moved to z 
println!("x = {}, z = {}", x, z); 

//Example:
//function to give ownership of a string to another function
fn give_ownership()->String{
    let some_string=String::from("Hello s1");
    some_string
}

//function to take and return the ownership of a string
fn take_and_give_ownership(example_string:String)->String{
    example_string
}

pub fn main(){
    let s1=give_ownership();
    println!("s1: {}",s1);

    let s2=String::from("hello from main");
    let s3=take_and_give_ownership(s2);
    println!("s3: {}",s3);
}
