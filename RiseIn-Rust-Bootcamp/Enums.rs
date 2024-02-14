/*
Enums: Rust that allows you to represent multiple related values within a single data type(enum).
*/
enum Weather {
    Sunny,
    Cloudy,
    Rainy,
    Snowy,
}
let current_weather = Weather::Sunny;


/*
Enums with Associated Data:
*/
/*
Example:1
*/
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data.");
        }
        Message::Move { x, y } => {
            println!("Move to coordinates x: {}, y: {}", x, y);
        }
        Message::Write(text) => {
            println!("Text message: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red: {}, green: {}, blue: {}", r, g, b);
        }
    }
}

fn main(){
    let msg = Message::Write(String::from("Hello, Rust!"));
    process_message(msg);
}

/*
Example:2
*/
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn process_message(msg: &Message) {
    match msg {
        Message::Quit => println!("The Quit variant has no data."),
        Message::Move { x, y } => println!("Move to coordinates x: {}, y: {}", x, y),
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change the color to red: {}, green: {}, blue: {}", r, g, b),
    }
}

fn main() {
    let messages = vec![
        Message::Quit,
        Message::Move { x: 1, y: 2 },
        Message::Write(String::from("Hello, Rust!")),
        Message::ChangeColor(0, 1, 2),
    ];

    for msg in messages {
        process_message(&msg);
    }
}

/*
‘if let’ Syntax:
*/

enum Animal {
    Dog(String),
    Cat(String),
    Bird(String),
}

fn main() {
    let my_pet1 = Animal::Cat("Fluffy".to_string());
    let my_pet2 = Animal::Dog("Peter".to_string());

    if let Animal::Cat(name) = my_pet1 {
        println!("My cat's name is: {}", name);
    }if let Animal::Dog(name) = my_pet2 {
        println!("My Dog's name is: {}",name);
    } else {
        println!("My pet is not a cat.");
    }
}

/*
Enums and Methods in Rust:
*/
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
            Message::Write(s) => println!("Write: {}", s),
            Message::ChangeColor(r, g, b) => println!("Change color to R: {}, G: {}, B: {}", r, g, b),
        }
    }
}

fn main() {
    let message = vec![
        Message::Write(String::from("Hello, Rust!")),
        Message::Quit,
        Message::Move{x:1,y:2},
        Message::ChangeColor(1,2,3),
    ];
    
    for msg in message{
        msg.call();
    }
}
