/*
Booleans: bool, which can have the values true or false
Integers: i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, which are signed and unsigned integers of different sizes
Floating-point numbers: f32 and f64, which are single-precision and double-precision floating-point numbers, respectively
Characters: char, which represents a single Unicode character
Strings: &str and String, which represent a sequence of Unicode characters
Arrays: [T; N], which represent a fixed-size array of elements of type T
Slices: [T] and &[T], which represent a variable-size sequence of elements of type T
Tuples: (T1, T2, ..., Tn), which represent a fixed-size sequence of elements of different types
Unit type: (), which represents an empty tuple and is used when no value is needed
*/

`Booleans:` 

  fn main(){
    let is_rust_fun = true;
    let is_rust_hard = false;
    println!("{}\n{}",is_rust_fun,is_rust_hard);
  }
