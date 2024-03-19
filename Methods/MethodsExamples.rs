//Example:To find the area of the Rectangle
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    //methods with the same name of a field
    fn width(&self) -> bool {
        self.width > 0
    }

    //getter for hight
    fn height(&self) -> u32 {
        self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 40,
    };

    println!("Area of the Rectangle: {}", rect1.area());
    //print width of the rectangle
    if rect1.width() {
        println!("Width: {}", rect1.width);
    } else {
        println!("Error");
    }
    //print height of the rectangle
    println!("Height: {}", rect1.height());
}
