fn main() {
    // Integer -> 4 bytes, float -> 8 bytes
    let x: i32 = 3; // Explicitly specify x as an i32 integer
    let y: f64 = 6.0; // Explicitly specify y as an f64 floating-point number
    println!("Size of `x` is {} bytes", std::mem::size_of_val(&x));
    println!("Size of `y` is {} bytes", std::mem::size_of_val(&y));
}
