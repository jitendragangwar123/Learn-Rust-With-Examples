/*
-Variables are immutable by default.
-To make a variable mutable, you can use the mut keyword before the variable name.
*/

let x = 42; // immutable variable
x = 10; // error: cannot assign twice to immutable variable

let mut x = 42; // mutable variable
x = 10; // OK!

/*
-Rust also has type inference, which means that the compiler can automatically infer the type of a variable based on its value.
*/
let y = 3.14; // Rust infers the type as f64

/*
-Rust also supports shadowing, which means that you can declare a new variable with the same name as an existing variable in a more nested scope.
*/
let x = 42;
let x = x + 1;

//you aren’t allowed to use the mut keyword with constants, Constants aren’t immutable by default.
fn main() { 
    const N: i32 = 1; 
    println!("num: {}", N); 
}


// multiple variables in a single line 
fn main() { 
  let ( first, middle, last ) = ("geeks", "for","geeks"); 
  println!("{} {} {} is amazing", first, middle, last ); 
}

/*
Shadowing:
        Shadowing is an instance where we can make a variable mutable temporarily.
        Shadowing also allows changing the data type of variable. 
*/
fn main() { 
    // original value of gfg  
    // variable is 100 
    let gfg = 100; 
      
    // gfg variable has value 
    // 100-50 = 50 here 
    // gfg variable got shadowed 
    let gfg = gfg -50; 
      
    // Again gfg variable has value 
    // 50*5 = 250 
    // gfg variable got shadowed again 
    let gfg = gfg * 5; 
  
    println!("The value of gfg is: {}", gfg); 
}
