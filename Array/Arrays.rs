fn main() {
    let num: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Done");
}

// Index & printing specific elements of an Array
fn main() {
    let num: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", num [2] ); //replace underscore
}

// Changing the elements of an Array
fn main() {
    let mut mnts: [&str; 4] = ["Jan", "Feb", "Dec", "Apr"];
    mnts[2] = "Mar"; // replace the underscore
    println!("{}", mnts[2]);
}

// Displaying the count of elements
fn main () {
    let num: [i32; 6] = [1, 2, 3, 4, 5, 6];
    println!("{}", num.len());
}
