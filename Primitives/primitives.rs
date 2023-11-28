/*
Scalar Types:
  1. Signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
  2. Unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
  3. Floating point: f32, f64
  4. char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
  5. bool either true or false
  6. The unit type (), whose only possible value is an empty tuple: ()
*/

/*
Compound Types:
  1. Arrays like [1, 2, 3]
  2. Tuples like (1, true)
*/

//primitives.rs
fn main() {
    // Variables can be type annotated.
    let logical: bool = true;

    let a_float: f64 = 1.0;  // Regular annotation
    let an_integer   = 5i32; // Suffix annotation

    // Or a default will be used.
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`

    // A type can also be inferred from context.
    let mut inferred_type = 12; // Type i64 is inferred from another line.
    inferred_type = 4294967296i64;

    println!("{}",inferred_type);

    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;
    println!("{}",mutable);

    // Error! The type of a variable can't be changed.
    //mutable = true;

    // Variables can be overwritten with shadowing.
    let mutable = true;
    println!("{}",mutable)
}

/*
Output:-
4294967296
21
true
*/
