// to find the factorial of the number
fn fact(n: i32) -> i32 {
    if n == 0 {
        1 
    } else if n > 0 {
        n * fact(n - 1) 
    } else {
        -1 
    }
}

fn main() {
    println!("{:?}", fact(6));
}

// other approach
fn fact(n: i32) -> i32 {
    if n < 0 {
        -1 
    } else if n == 0 {
        1 
    } else {
        (1..=n).product() 
    }
}

fn main() {
    println!("{:?}", fact(6)); 
}
