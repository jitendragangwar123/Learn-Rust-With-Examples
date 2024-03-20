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


// to check the one rectangle fit into another rectangle or not
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
            || self.width > other.height && self.height > other.width
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 0,
        height: 40,
    };

    let rect2 = Rectangle {
        width: 30,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Area of the Rectangle: {}", rect1.can_hold(&rect2));
    println!("Area of the Rectangle: {}", rect3.can_hold(&rect2));
}

// to find the area of the square
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // associate function to define a square
    fn square(size:u32)->Self{
        Self { 
            width: size, 
            height: size, 
        }
    }
}

fn main() {
    let square=Rectangle::square(10);
    println!("Area of the Square: {}", square.area());
}

