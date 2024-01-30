//Example:1
// taking user input as a string
use std::io::{self, Write};
fn main() {
    let mut s = String::new();
    print!("Please enter your name: ");
    io::stdout().flush().unwrap(); // Flush stdout to display the print message immediately
    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read line"); // Read user input

    println!("Your name is: {}", s.trim());
}

//Example:2
// taking user input as a number
use std::io;
fn main() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap(); 
    let a: i32 = a.trim().parse().unwrap();
    println!("Your number is: {}", a); 
}

//Example:3
// trim the string input
use std::io;
fn main() {
    let mut x = String::new();
    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line");

    x = x.trim().to_string();  // to remove any trailing newline
    println!("Hello {}", x);
}

//Example:4
use std::io;
fn main(){
    let mut buf=String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let v:Vec<&str>=buf.split(" ").collect();
    let w:i64=v[0].trim().parse().unwrap();
    let h:i64=v[1].trim().parse().unwrap();
    
    if w>=60 && h<=130 {
        println!("{}","YES");
    }
    else{
        println!("{}","NO");
    }
}

//Example:5
use std::io;
use std::f64;
fn main(){
    let mut buf=String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut test:i32=buf.trim().parse().unwrap();
    while test>0 {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let v:Vec<&str>=buf.split(" ").collect();
        let x:f64=v[0].trim().parse().unwrap();
        let n:f64=v[1].trim().parse().unwrap();
        
        let res=f64::ceil(n/ 100.0);
        if x>res {
            println!("{}",0);
        }
        
        else{
            println!("{}",res-x);
        }
        test=test-1;
    }
}
