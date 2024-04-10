// initialize an array
fn main() {
    let num: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Done");
}

// index & printing specific elements of an Array
fn main() {
    let num: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", num[2] ); 
}

// changing the elements of an Array
fn main() {
    let mut mnts: [&str; 4] = ["Jan", "Feb", "Dec", "Apr"];
    mnts[2] = "Mar"; 
    println!("{}", mnts[2]);
}

// displaying the count of elements
fn main () {
    let num: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", num.len());
}

// remove the elements from the array
fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];
    vec.remove(2); //put index
    println!("{:?}",vec);
}
