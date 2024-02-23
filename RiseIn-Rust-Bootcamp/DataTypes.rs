/*
1.Scalar Type:
    Booleans: bool, which can have the values true or false
    Integers: i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, which are signed and unsigned integers of different sizes
    Floating-point numbers: f32 and f64, which are single-precision and double-precision floating-point numbers, respectively
    Characters: char, which represents a single Unicode character

2.Compound Type:
    Strings: &str and String, which represent a sequence of Unicode characters
    Arrays: [T; N], which represent a fixed-size array of elements of type T
    Slices: [T] and &[T], which represent a variable-size sequence of elements of type T
    Tuples: (T1, T2, ..., Tn), which represent a fixed-size sequence of elements of different types
    Unit type: (), which represents an empty tuple and is used when no value is needed

3. Custom Types:
    Stucts:
    Enums:
*/

//Booleans:
fn main(){
    let is_rust_fun = true;
    let is_rust_hard = false;
    println!("{}\n{}",is_rust_fun,is_rust_hard);
}

//Integers:
/*
   Length        Signed       Unsigned
   8-bit         i8            u8
   16-bit        i16           u16
   32-bit        i32           u32
   64-bit        i64           u64
   128-bit       i128          u128
   arch          isize         usize
   */
fn main(){
   let small_number:u8=123;
   let big_number:u128=12787667667868;
   let small_number1:i8=-128;
   let big_number1:i128=-12787667667868;
   println!("small_number:{}",small_number);
   println!("big_number:{}",big_number);
   println!("small_number1:{}",small_number1);
   println!("big_number1:{}",big_number1);
}

// Numeral System
   /*
   Decimal
   Hexadecimal(0x)
   Octal (0o)
   Binary (0b)
   Byte (b) -->b'A'
   */
fn main(){
   let decimal=98_000;
   let hex=0xff;
   let octal=0o77;
   let binary=0b11111_000;
   let byte=b'A';
   println!("decimal:{}",decimal);
   println!("hex:{}",hex);
   println!("octal:{}",octal);
   println!("binary:{}",binary);
   println!("byte:{}",byte);
}

fn main(){
    let x: i32 = 42;
    let y: u32 = 45;
    let min_i32 = i32::MIN;
    let max_i32 = i32::MAX;
    let min_u32 = u32::MIN;
    let max_u32 = u32::MAX;
    println!("{}\n{}",x,y);
    println!("The minimum value of i32 is {} and the maximum value is {}.", 
    min_i32, max_i32);
    println!("The minimum value of u32 is {} and the maximum value is {}.", 
    min_u32, max_u32);
}

//Floating-point numbers:
fn main(){
    let pi: f64 = 3.14159;
    let x: f32 = 3.12;
    println!("{}\n{}",pi,x);
}

//Characters:
fn main(){
    let letter_a: char = 'a';
    println!("{}",letter_a);
}


/*
Strings:
1. &str is a reference to a string slice, while String is a growable string type.
2. String slices are immutable by default, while String variables are mutable by default.
3. String is a growable string type that is stored in the heap, while a string slice (&str) is a reference 
   to a fixed-size sequence of characters that can be stored either in the heap or in the program's binary.
*/ 

fn main(){
    let message: &str = "Hello, world!";
    let mut name = String::from("Alice");
    //let name = String::from("Bob");
    println!("{}\n{}",message,name);
}

//Arrays:
fn main(){
    let numbers: [i32; 3] = [1, 2, 3];
    println!("{:?}",numbers);
    let second_number = numbers[1];
    println!("The second number in the array is {}.", second_number);
}

//Slices: A contiguous sequence of elements of the same type.
fn main(){
    let numbers: [i32; 3] = [1, 2, 3];
    let slice = &numbers[1..3];
    println!("Slice of the numbers {:?}", slice);
    let first_element = slice[0];
    println!("The first element of the slice is {}.", first_element);
}

/*
output:
Slice of the numbers [2, 3]
The first element of the slice is 2.
*/

//Tuples:
fn main(){
    let person = ("Jacob", 30);
    let name = person.0;
    let age = person.1;
    println!("The person's name is {} and his age is {}.", name, age);
    let persons = (("Jeetu", "Jay"), 50);
    println!("The person's name is {} and {} and their age is {}.", persons.0.0, persons.0.1, persons.1);
}

//Unit type:
fn main(){
    let result = ();
    println!("The result is {:?}.", result);
}
