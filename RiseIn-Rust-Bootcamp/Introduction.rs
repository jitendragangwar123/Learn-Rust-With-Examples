fn main(){
    // variable initialization
    let message = "Hello, world!";
    let x: i32 = 42;
    let y: i32= 8;
    let pi: f64 = 3.14159;
    let is_rust_fun: bool = true;
    let letter_a: char = 'a';
    
    // function initialization
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }
    
    println!("{}",message);
    println!("{}",add(23,2));
    
    // if-else
    if x >= 0 {
	    println!("x is non-negative");
    } else {
      println!("x is negative");
    }
    
    //while loop
    let mut i = 1;
    while i <= 5 {
	println!("{}", i);
      i += 1;
    }  
}
