/*
Methods:
    Methods in Rust are similar to functions but they are defined within
    the context of a Struct
    (or a enum or a trait object).
    They are called by instances of the struct,and their first parameter
    is always self,which represents the instance of the struct.
*/

struct Rectangle {
    width: u32,
    height: u32,
}

pub fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 90,
    };
    println!("Area of the Rectangle: {}", area(&rect1));
}

fn area(rectange: &Rectangle) -> u32 {
    rectange.width * rectange.height
}
