/*
Slice: Slice are a reference to a contiguous sequence of elements in the collection.
       They are a view into the original collection and do not store any data themselves.
       Slices are used to give a part of a collection to a function or to iterate over a part of the collection. 
*/

/*
Syntax : &[T]
T --> Type of the elements in the collection
& --> Reference to the collection
*/

//Example:1
fn main() {
    //slice of an array of characters
    let arr: [char; 5] = ['a', 'b', 'c', 'd', 'e'];
    let slice: &[char] = &arr[1..3];
    println!("{:?}", slice);

    //slice of a vector of integers
    let vec: Vec<i32> = vec![10, 20, 30, 40, 50];
    let slice = &vec[3..4];
    println!("{:?}", slice);

    //slice for strings
    let s:String=String::from("hello world");
    let hello = &s[1..2];
    let world= &s[8..11];
    println!("{:?} {:?}",hello,world);
}

//Example:2
fn main() {
    let s=String::from("Hello Jay");
    //shortcut for initial index
    let slice=&s[0..3];
    println!("{}",slice);
    let slice=&s[..3];
    println!("{}",slice);

    //shortcut for final index
    let len=s.len();
    let slice=&s[5..len];
    println!("{}",slice);
    let slice=&s[5..];
    println!("{}",slice);

    //shortcut for initial index and final index
    let slice=&s[0..len];
    println!("{}",slice);
    let slice=&s[..]; 
    println!("{}",slice); //Hello Jay
}

//Exercise : get the first word of the string(without using slice)
fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("String : {}", s); //hello world
    println!("first word length: {}", word); //5

    //clear the string
    s.clear();
    println!("String : {}", s); // " "
    println!("first word length: {}", word); //5
}

fn first_word(s: &String) -> usize {
    //convert string to bytes
    let bytes = s.as_bytes();

    //iterate through the bytes and return the index of the first space
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            //if the bytes is a space
            return i;
        }
    }
    //If no space is found return the length of the string
    s.len()
}

//Exercise: get the first word of the string(using slice)
pub fn main() {
    //get the first word of the string(without using slice)
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("String: {}", s); //hello world
    println!("first word: {}", word); //hello

    //string literals are slice
    let s = "hello world";
    let word = first_word(s);
    println!("String: {}", s); //hello world
    println!("first word: {}", word); //hello
}

fn first_word(s: &str) -> &str {
    //convert string to bytes
    let bytes = s.as_bytes();

    //iterate through the bytes and return the index of the first space
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            //if the bytes is a space then return the slice
            return &s[0..i];
        }
    }
    // if no space is found return the length of the string
    &s[..]
}

// Slices are immutable,so you can use the slice here instead of string.
// String literals are slices, so no need to use reference.
