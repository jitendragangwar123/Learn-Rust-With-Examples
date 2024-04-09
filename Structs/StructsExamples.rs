//Example: To find the area of the rectangle 
//using tuples
fn main(){
    let rect1=(30,40);
    println!("Area of the Rectangle: {}",area_of_rectangle(rect1));
}

fn area_of_rectangle(dimensions:(u32,u32))->u32{
    dimensions.0*dimensions.1
}

//using structs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 40,
    };
    println!("Rectangle Params: {:#?}",rect1);
    println!("Area of the Rectangle: {}", area_of_rectangle(&rect1));
}
