/*
Enums:
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
