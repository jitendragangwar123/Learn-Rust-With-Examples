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
