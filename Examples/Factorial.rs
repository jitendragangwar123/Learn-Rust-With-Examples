// First Method
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

// Second Method
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
