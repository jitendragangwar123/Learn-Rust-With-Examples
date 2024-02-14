/*
Structs, short for structures, are a way to group related data together in a custom data type.
*/
struct Book {
    title: String,
    author: String,
    publication_year: u32,
}

/*
Defining Structs:
*/
struct Point {
    x: f32,
    y: f32,
}

/*
Creating Instances of Structs:
*/
let point = Point { x: 1, y: 2 };

/*
Accessing Struct Fields: 
*/
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let alice = Person {
        name: String::from("Alice"),
        age: 30,
    };
 
    // Accessing the 'name' and 'age' fields
    println!("Name: {}", alice.name);
    println!("Age: {}", alice.age);
}

/*
Example: You can also modify the values of the fields if the struct instance is mutable.
*/
struct Person {
    name: String,
    age: u32,
}
fn main() {
    let mut alice = Person {
        name: String::from("Alice"),
        age: 30,
    };
 
    // Modifying the 'age' field
    alice.age = 31;
    println!("Updated Age: {}", alice.age);
}

/*
Functions and Structs: Functions can take structs as parameters, and they can also return structs.
*/
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

//calculate the distance 
fn distance(p1:Point, p2:Point) -> f64{
    let x_diff=p1.x - p2.x;
    let y_diff=p1.y - p2.y;
    (x_diff*x_diff + y_diff*y_diff).sqrt().into()
}

//calculate the midpoint
fn midpoint(p1:Point, p2:Point) -> Point{
    let x_mid=(p1.x+p2.x)/2.0;
    let y_mid=(p1.y+p2.y)/2.0;
    Point{x:x_mid,y:y_mid}
}
fn main() {
    let point1=Point{x:4.0, y:3.0};
    let point2=Point{x:3.0, y:2.0};
    
    //let dist=distance(point1,point2);
    let mid=midpoint(point1,point2);
    //println!("The distance between x1,y1,x2,y2 is {}", dist);
    println!("The mid point of poin1 and point2 is {} and {}",mid.x,mid.y);
}

/*
Tuple Structs:
*/
#[derive(Debug)]
struct Point3D(f32, f32, f32);
// calculate the distance 
fn calculate_distance(point1: Point3D, point2: Point3D) -> f32 {
    let dx = point1.0 - point2.0;
    let dy = point1.1 - point2.1;
    let dz = point1.2 - point2.2;
 
    (dx*dx + dy*dy + dz*dz).sqrt()
}

fn main() {
    let point1 = Point3D(1.0, 2.0, 3.0);
    let point2 = Point3D(3.0, 4.0, 1.0);
    let dist=calculate_distance(point1,point2);
    println!("Calculated Distance is {:?}",dist);
}


/*
Unit Structs:
*/
#[derive(Debug)]
struct Empty;
impl Empty {
    fn greet(&self) {
        println!("Hello, I am an empty struct!");
    }
}
fn main(){
    let empty_instance = Empty;
    let msg=empty_instance.greet();
    println!("{:?}", msg);
}

/*
Debugging with Structs: The Debug trait allows you to print the contents of a struct instance using the {:?} format specifier.
*/
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}
fn main() {
    let alice = Person {
        name: String::from("Alice"),
        age: 30,
    };
    println!("Person name is {:?} and age is {:?}", alice.name, alice.age);
}

/*
Implementing Methods for Structs:
*/
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}
 
impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn main() {
    let my_rectangle = Rectangle {
        width: 10.0,
        height: 5.0,
    };
 
    let area = my_rectangle.area();
    println!("The area of the rectangle is: {}", area);
}
