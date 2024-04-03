fn main() { 
   //integer -> 4 bytes, char -> 4 bytes, bool -> 1 byte, float -> 8 bytes
    let x = 3; 
    let y = 6.0;
    let z:bool=true;
    let c:char='c';
    println!("size of `x` is {} bytes", std::mem::size_of_val(&x)); 
    println!("size of `y` is {} bytes", std::mem::size_of_val(&y));    
    println!("size of `z` is {} bytes", std::mem::size_of_val(&z)); 
    println!("size of `c` is {} bytes", std::mem::size_of_val(&c)); 
}
