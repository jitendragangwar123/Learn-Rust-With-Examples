// initialize an array
fn main() {
    let num: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Done");
}

// Index & printing specific elements of an Array
fn main() {
    let num: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", num [2] ); 
}

// Changing the elements of an Array
fn main() {
    let mut mnts: [&str; 4] = ["Jan", "Feb", "Dec", "Apr"];
    mnts[2] = "Mar"; 
    println!("{}", mnts[2]);
}

// Displaying the count of elements
fn main () {
    let num: [i32; 6] = [1, 2, 3, 4, 5, 6];
    println!("{}", num.len());
}

// Remove the elements from the array
fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];
    vec.remove(2); //put index
    println!("{:?}",vec);
}
